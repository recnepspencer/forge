use super::*;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

pub(super) fn conflict_scope_identity(
    declaration: &QueryWorkflowDeclaration,
    merge_declaration: &LoweredMergeWorkflowDeclaration,
    merge_class_family: &str,
    merge_class: &str,
    merge_class_admission: &str,
    row_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "workflow_conflict_scope_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("conflict_declaration"),
            declaration.report().declaration_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("merge_declaration"),
            merge_declaration
                .declaration()
                .report()
                .declaration_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target"),
            &merge_declaration.merge_request().target_branch().0,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source"),
            &merge_declaration.merge_request().source_branch().0,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("merge_intent"),
            merge_declaration.merge_intent().as_str(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("row_digest"), row_digest)
        .field_shape(
            ForgeQueryEvidenceTag::new("merge_class_family"),
            merge_class_family,
        )
        .field_shape(ForgeQueryEvidenceTag::new("merge_class"), merge_class)
        .field_shape(
            ForgeQueryEvidenceTag::new("merge_class_admission"),
            merge_class_admission,
        )
        .seal()
}

pub(super) fn post_merge_scope_identity(
    declaration: &QueryWorkflowDeclaration,
    outcome: &WorkflowAuthorityOutcomeArtifact,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "workflow_post_merge_scope_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("inspection_declaration"),
            declaration.report().declaration_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("authoritative_outcome"),
            outcome.authoritative_outcome_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            outcome.family().as_str(),
        )
        .seal()
}

pub(super) fn workflow_authority_request_identity(
    family: WorkflowAuthorityOutcomeFamily,
    request_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "workflow_authority_request_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("request"), request_identity)
        .seal()
}

pub(super) fn workflow_authoritative_outcome_identity(
    declaration: &QueryWorkflowDeclaration,
    family: &WorkflowAuthorityOutcomeFamily,
    authority_request_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "workflow_authoritative_outcome_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("declaration"),
            declaration.report().declaration_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("binding"),
            declaration.report().binding_identity(),
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("authority_request"),
            authority_request_identity,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            declaration.binding().basis_identity(),
        )
        .seal()
}

pub(super) fn delivery_or_failure_identity(
    outcome: &WorkflowAuthorityOutcomeArtifact,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "workflow_delivery_or_failure_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("outcome"),
            outcome.authoritative_outcome_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("freshness"),
            outcome.freshness_outcome().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("budget"),
            outcome.budget_outcome().as_str(),
        )
        .seal()
}

pub(super) fn workflow_replay_bundle_identity(
    outcome: &WorkflowAuthorityOutcomeArtifact,
    delivery_or_failure_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "workflow_replay_bundle_v1",
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("query"),
            outcome.source_query_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("plan"),
            outcome.source_plan_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            outcome.source_basis_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("declaration"),
            outcome.source_declaration_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_target"),
            outcome.authority_target_family().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("request"),
            outcome.authority_request_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("outcome"),
            outcome.authoritative_outcome_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("delivery"),
            delivery_or_failure_identity,
        )
        .seal()
}
