use forge_proof::TransitionOutcome;

use super::targets::ForgeQueryDeclarationBoundContributionTarget;
use super::test_support::{declaration_target, ready, success};
use super::{
    materialize_admitted_preview_workflow_foundation, materialize_query_workflow_declaration,
    ForgeQueryWorkflowContributionAuthoring,
};

#[test]
fn workflow_materializer_reuses_admitted_preview_foundation_identity() {
    let declaration = success(materialize_query_workflow_declaration(ready_workflow(
        ForgeQueryWorkflowContributionAuthoring::preview_only_query_inspection(
            "spatial.preview.identity",
            "workflow declarations should bind through the same admitted preview foundation identity",
            crate::facade::runtime::BridgePreviewSessionIdentity::new("preview-session:identity"),
        ),
    )));
    let foundation = success(materialize_admitted_preview_workflow_foundation(ready_workflow(
        ForgeQueryWorkflowContributionAuthoring::preview_only_query_inspection(
            "spatial.preview.identity",
            "workflow declarations should bind through the same admitted preview foundation identity",
            crate::facade::runtime::BridgePreviewSessionIdentity::new("preview-session:identity"),
        ),
    )));

    assert_eq!(declaration.binding().source_digest(), foundation.digest());
    assert_eq!(
        declaration.binding().query_identity_digest(),
        foundation.validated_query_digest().as_str()
    );
    assert_eq!(
        declaration.binding().basis_digest(),
        foundation.binding_digest()
    );
    assert_eq!(
        declaration.binding().preview_request_family(),
        Some(foundation.request_family())
    );
}

#[test]
fn workflow_materializer_denies_runtime_only_postures_at_preview_foundation_boundary() {
    let denied = materialize_query_workflow_declaration(ready(super::proof_integration::create_requested_domain_capability_contribution(
        declaration_target("intent-workflow-discard-read-only-denied"),
        super::ForgeQueryWorkflowContributionPayload::with_runtime_semantics(
            super::ForgeQueryWorkflowContributionPosture::DiscardRequired,
            "spatial.preview.discard.read_only_denied",
            "workflow declaration materialization must deny the same dishonest read-only discard-required preview semantics as preview foundation admission",
            Some(super::ForgeQueryWorkflowRuntimeSemantics::new(
                super::ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    crate::facade::runtime::BridgePreviewSessionIdentity::new(
                        "preview-session:denied",
                    ),
                    crate::workflow::WorkflowPreviewEvaluationClass::ReadOnly,
                ),
                crate::workflow::WorkflowDeclarationFamily::ConflictInspectionNarrow,
                crate::workflow::WorkflowAuthorityTargetFamily::QueryInspection,
                crate::workflow::WorkflowCostClass::InspectionNarrow,
                crate::workflow::WorkflowBudgetClass::InspectionBounded,
                crate::workflow::WorkflowFreshnessPolicy::ExactBasis,
            )),
        ),
    )));

    assert!(matches!(
        denied,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics
    ));
}

fn ready_workflow(
    authoring: ForgeQueryWorkflowContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyWorkflowContribution<
    ForgeQueryDeclarationBoundContributionTarget,
> {
    ready(authoring.bind_to_declaration_target(declaration_target("intent-workflow")))
}
