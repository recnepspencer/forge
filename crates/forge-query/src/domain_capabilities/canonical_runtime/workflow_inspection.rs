use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::{
    ForgeQueryDomainCapabilityTransitionOutcome, ForgeQueryMaterializationReadyWorkflowContribution,
};
use crate::workflow::{
    admit_query_workflow_declaration, inspect_merge_conflicts, inspect_post_merge_outcome,
    shape_merge_authority_outcome, shape_writeback_authority_outcome,
    QueryConflictInspectionArtifact, QueryPostMergeInspectionArtifact, WorkflowDeclarationRequest,
};

use super::workflow::ForgeQueryWorkflowDeclarationMaterializationTarget;
use super::workflow_semantics::{
    inconsistent_workflow_runtime_semantics_denial, missing_workflow_runtime_semantics_denial,
    workflow_inspection_semantics_match_runtime, workflow_runtime_semantics_match_posture,
};

pub fn materialize_query_conflict_inspection_artifact<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<QueryConflictInspectionArtifact>
where
    T: ForgeQueryWorkflowDeclarationMaterializationTarget,
{
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return TransitionOutcome::Denied(missing_workflow_runtime_semantics_denial(
            "workflow conflict inspection materialization",
            payload,
            domain_contribution.target().kind(),
            domain_contribution.request_digest(),
        ));
    };
    if !workflow_runtime_semantics_match_posture(payload.posture(), runtime_semantics) {
        return TransitionOutcome::Denied(inconsistent_workflow_runtime_semantics_denial(
            "workflow conflict inspection materialization",
            payload,
            runtime_semantics,
            domain_contribution.target().kind(),
            domain_contribution.request_digest(),
        ));
    }
    let Some(inspection_semantics) = payload.inspection_semantics() else {
        return TransitionOutcome::Denied(missing_inspection_semantics_denial(
            domain_contribution.target().kind(),
            payload.semantic_code(),
            domain_contribution.request_digest(),
        ));
    };
    if !workflow_inspection_semantics_match_runtime(runtime_semantics, inspection_semantics) {
        return TransitionOutcome::Denied(inconsistent_inspection_semantics_denial(
            domain_contribution.target().kind(),
            payload.semantic_code(),
            domain_contribution.request_digest(),
            "workflow conflict inspection semantics do not match runtime declaration semantics",
        ));
    }
    let Some((lowered_merge, relational_inspection)) =
        inspection_semantics.lowered_merge_conflict()
    else {
        return TransitionOutcome::Denied(inconsistent_inspection_semantics_denial(
            domain_contribution.target().kind(),
            payload.semantic_code(),
            domain_contribution.request_digest(),
            "workflow conflict inspection requires merge-conflict semantics",
        ));
    };

    let declaration = match admit_query_workflow_declaration(
        lowered_merge.declaration().binding(),
        WorkflowDeclarationRequest::new(
            runtime_semantics.declaration_family().clone(),
            runtime_semantics.authority_target_family().clone(),
            runtime_semantics.cost_class().clone(),
            runtime_semantics.budget_class().clone(),
            runtime_semantics.freshness_policy().clone(),
        ),
    ) {
        Ok(declaration) => declaration,
        Err(error) => {
            return TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                "workflow-preview",
                domain_contribution.target().kind(),
                domain_contribution.request_digest(),
                format!(
                    "workflow conflict inspection declaration denied for `{}` with `{:?}`",
                    payload.semantic_code(),
                    error.failure_class()
                ),
            ))
        }
    };

    inspect_merge_conflicts(&declaration, lowered_merge, relational_inspection).map_or_else(
        |error| {
            TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                "workflow-preview",
                domain_contribution.target().kind(),
                domain_contribution.request_digest(),
                format!(
                    "workflow conflict inspection denied for `{}` with `{:?}`",
                    payload.semantic_code(),
                    error.failure_class()
                ),
            ))
        },
        TransitionOutcome::Success,
    )
}

pub fn materialize_query_post_merge_inspection_artifact<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<QueryPostMergeInspectionArtifact>
where
    T: ForgeQueryWorkflowDeclarationMaterializationTarget,
{
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return TransitionOutcome::Denied(missing_workflow_runtime_semantics_denial(
            "workflow post-merge inspection materialization",
            payload,
            domain_contribution.target().kind(),
            domain_contribution.request_digest(),
        ));
    };
    if !workflow_runtime_semantics_match_posture(payload.posture(), runtime_semantics) {
        return TransitionOutcome::Denied(inconsistent_workflow_runtime_semantics_denial(
            "workflow post-merge inspection materialization",
            payload,
            runtime_semantics,
            domain_contribution.target().kind(),
            domain_contribution.request_digest(),
        ));
    }
    let Some(inspection_semantics) = payload.inspection_semantics() else {
        return TransitionOutcome::Denied(missing_inspection_semantics_denial(
            domain_contribution.target().kind(),
            payload.semantic_code(),
            domain_contribution.request_digest(),
        ));
    };
    if !workflow_inspection_semantics_match_runtime(runtime_semantics, inspection_semantics) {
        return TransitionOutcome::Denied(inconsistent_inspection_semantics_denial(
            domain_contribution.target().kind(),
            payload.semantic_code(),
            domain_contribution.request_digest(),
            "workflow post-merge inspection semantics do not match runtime declaration semantics",
        ));
    }
    let (binding, outcome) = if let Some(lowered_merge) =
        inspection_semantics.post_merge_from_merge_input()
    {
        (
            lowered_merge.declaration().binding(),
            shape_merge_authority_outcome(lowered_merge),
        )
    } else if let Some(lowered_writeback) = inspection_semantics.post_merge_from_writeback_input() {
        (
            lowered_writeback.declaration().binding(),
            shape_writeback_authority_outcome(lowered_writeback),
        )
    } else {
        return TransitionOutcome::Denied(inconsistent_inspection_semantics_denial(
            domain_contribution.target().kind(),
            payload.semantic_code(),
            domain_contribution.request_digest(),
            "workflow post-merge inspection requires merge-outcome or writeback-outcome semantics",
        ));
    };

    let declaration = match admit_query_workflow_declaration(
        binding,
        WorkflowDeclarationRequest::new(
            runtime_semantics.declaration_family().clone(),
            runtime_semantics.authority_target_family().clone(),
            runtime_semantics.cost_class().clone(),
            runtime_semantics.budget_class().clone(),
            runtime_semantics.freshness_policy().clone(),
        ),
    ) {
        Ok(declaration) => declaration,
        Err(error) => {
            return TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                "workflow-preview",
                domain_contribution.target().kind(),
                domain_contribution.request_digest(),
                format!(
                    "workflow post-merge inspection declaration denied for `{}` with `{:?}`",
                    payload.semantic_code(),
                    error.failure_class()
                ),
            ))
        }
    };

    inspect_post_merge_outcome(&declaration, &outcome).map_or_else(
        |error| {
            TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                "workflow-preview",
                domain_contribution.target().kind(),
                domain_contribution.request_digest(),
                format!(
                    "workflow post-merge inspection denied for `{}` with `{:?}`",
                    payload.semantic_code(),
                    error.failure_class()
                ),
            ))
        },
        TransitionOutcome::Success,
    )
}

fn missing_inspection_semantics_denial(
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    semantic_code: &str,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "workflow-preview",
        target_kind,
        request_digest,
        format!("workflow inspection materialization requires inspection semantics for `{semantic_code}`"),
    )
}

fn inconsistent_inspection_semantics_denial(
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    semantic_code: &str,
    request_digest: &str,
    message: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics,
        "workflow-preview",
        target_kind,
        request_digest,
        format!("{message} for `{semantic_code}`"),
    )
}
