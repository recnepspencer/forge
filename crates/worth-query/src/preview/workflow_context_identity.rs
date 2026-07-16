use crate::identity::{
    CanonicalQueryDigest, CanonicalResultShapeDigest, ValidatedQueryDigest,
    ValidatedResultShapeDigest,
};
#[cfg(test)]
use crate::identity::{CollectionPlanDigest, ResultDigest};
use crate::workflow::{
    workflow_canonical_query_digest_evidence, workflow_validated_query_digest_evidence,
};
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};
use worth_runtime_bridge::facade::{
    BridgePreviewLifecycleStateKind, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, PreviewExecutionRecordIdentity,
};

use super::PreviewEvaluationClass;
#[cfg(test)]
use super::PreviewSessionBindingTuple;

pub(super) fn preview_session_identity_record_label(
    identity: &BridgePreviewSessionIdentity,
) -> &str {
    identity.terminal_projection_for_reporting()
}

pub(crate) fn preview_lifecycle_state_label(kind: BridgePreviewLifecycleStateKind) -> &'static str {
    match kind {
        BridgePreviewLifecycleStateKind::Declared => "declared",
        BridgePreviewLifecycleStateKind::Admitted => "admitted",
        BridgePreviewLifecycleStateKind::Active => "active",
        BridgePreviewLifecycleStateKind::Discarded => "discarded",
        BridgePreviewLifecycleStateKind::Promoted => "promoted",
    }
}

pub(super) fn compose_preview_session_binding_tuple_digest(
    canonical_query_digest: &CanonicalQueryDigest,
    canonical_result_shape_digest: &CanonicalResultShapeDigest,
    validated_query_digest: &ValidatedQueryDigest,
    validated_result_shape_digest: &ValidatedResultShapeDigest,
    evaluation_class: &PreviewEvaluationClass,
    preview_session_identity: &BridgePreviewSessionIdentity,
    declaration_identity: &BridgePreviewSessionDeclarationIdentity,
    declaration_digest: &str,
    lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    execution_record_identity: Option<&PreviewExecutionRecordIdentity>,
    replay_bundle_digest: Option<&str>,
    promotion_record_identity: Option<&str>,
    promotion_proof_digest: Option<&str>,
) -> String {
    let mut encoder =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "worth_query_preview_session_binding_tuple_v1",
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("canonical_query"),
                &workflow_canonical_query_digest_evidence(canonical_query_digest),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("canonical_result_shape"),
                canonical_result_shape_digest.as_str(),
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("validated_query"),
                &workflow_validated_query_digest_evidence(validated_query_digest),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("validated_result_shape"),
                validated_result_shape_digest.as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("evaluation_class"),
                evaluation_class.as_str(),
            )
            .field_bridge_retained_evidence_identity(
                WorthQueryEvidenceTag::new("preview_session"),
                &preview_session_identity.bridge_admission_evidence(),
            )
            .field_bridge_retained_evidence_identity(
                WorthQueryEvidenceTag::new("declaration_identity"),
                &declaration_identity.bridge_admission_evidence(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("declaration_digest"),
                declaration_digest,
            )
            .field_shape(
                WorthQueryEvidenceTag::new("lifecycle"),
                preview_lifecycle_state_label(lifecycle_state_kind),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("replay_bundle"),
                replay_bundle_digest.unwrap_or("none"),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("promotion_record"),
                promotion_record_identity.unwrap_or("none"),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("promotion_proof"),
                promotion_proof_digest.unwrap_or("none"),
            );
    encoder = match execution_record_identity {
        Some(identity) => encoder.field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("execution_record"),
            &identity.bridge_admission_evidence(),
        ),
        None => encoder.field_shape(WorthQueryEvidenceTag::new("execution_record"), "none"),
    };
    encoder.seal().as_str().to_string()
}

#[cfg(test)]
pub(super) fn compose_preview_binding_tuple_workflow_identity(
    binding_tuple: &PreviewSessionBindingTuple,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_preview_binding_tuple_identity_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("binding_digest"),
            binding_tuple.digest(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("canonical_query"),
            &workflow_canonical_query_digest_evidence(binding_tuple.canonical_query_digest()),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("validated_query"),
            &workflow_validated_query_digest_evidence(binding_tuple.validated_query_digest()),
        )
        .seal()
}

#[cfg(test)]
pub(super) fn compose_preview_declaration_digest_workflow_identity(
    binding_tuple: &PreviewSessionBindingTuple,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_preview_declaration_digest_identity_v1",
        )
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("declaration"),
            &binding_tuple
                .declaration_identity()
                .bridge_admission_evidence(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("declaration_digest"),
            binding_tuple.declaration_digest(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("canonical_query"),
            &workflow_canonical_query_digest_evidence(binding_tuple.canonical_query_digest()),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("validated_query"),
            &workflow_validated_query_digest_evidence(binding_tuple.validated_query_digest()),
        )
        .seal()
}

#[cfg(test)]
pub(super) fn compose_preview_comparison_ordering_digest(parts: &[String]) -> String {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_preview_comparison_ordering_v1",
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("ordering_part"),
            parts.iter().map(String::as_str),
        )
        .seal()
        .as_str()
        .to_string()
}

#[cfg(test)]
pub(super) fn compose_preview_comparison_materialization_boundary_digest(
    parts: &[String],
) -> String {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_preview_comparison_materialization_boundary_v1",
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("materialization_part"),
            parts.iter().map(String::as_str),
        )
        .seal()
        .as_str()
        .to_string()
}

#[cfg(test)]
pub(super) fn compose_preview_comparison_eligibility_digest(
    canonical_query_digest: &CanonicalQueryDigest,
    canonical_result_shape_digest: &CanonicalResultShapeDigest,
    collection_digest: Option<&CollectionPlanDigest>,
    result_family: &str,
    ordering_digest: &str,
    materialization_boundary_digest: &str,
    shape_check_width: usize,
) -> String {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_preview_comparison_eligibility_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("canonical_query"),
            &workflow_canonical_query_digest_evidence(canonical_query_digest),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("canonical_result_shape"),
            canonical_result_shape_digest.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("collection"),
            collection_digest
                .map(CollectionPlanDigest::as_str)
                .unwrap_or("detail"),
        )
        .field_shape(WorthQueryEvidenceTag::new("result_family"), result_family)
        .field_shape(WorthQueryEvidenceTag::new("ordering"), ordering_digest)
        .field_shape(
            WorthQueryEvidenceTag::new("materialization_boundary"),
            materialization_boundary_digest,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("shape_check_width"),
            shape_check_width,
        )
        .seal()
        .as_str()
        .to_string()
}

#[cfg(test)]
pub(super) fn compose_preview_comparison_candidate_digest(
    validated_query_digest: &ValidatedQueryDigest,
    result_digest: &ResultDigest,
    canonical_query_digest: &CanonicalQueryDigest,
    canonical_result_shape_digest: &CanonicalResultShapeDigest,
    collection_digest: Option<&CollectionPlanDigest>,
    result_family: &str,
    ordering_digest: &str,
    materialization_boundary_digest: &str,
    shape_check_width: usize,
) -> String {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_preview_comparison_candidate_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("validated_query"),
            &workflow_validated_query_digest_evidence(validated_query_digest),
        )
        .field_shape(WorthQueryEvidenceTag::new("result"), result_digest.as_str())
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("canonical_query"),
            &workflow_canonical_query_digest_evidence(canonical_query_digest),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("canonical_result_shape"),
            canonical_result_shape_digest.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("collection"),
            collection_digest
                .map(CollectionPlanDigest::as_str)
                .unwrap_or("detail"),
        )
        .field_shape(WorthQueryEvidenceTag::new("result_family"), result_family)
        .field_shape(WorthQueryEvidenceTag::new("ordering"), ordering_digest)
        .field_shape(
            WorthQueryEvidenceTag::new("materialization_boundary"),
            materialization_boundary_digest,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("shape_check_width"),
            shape_check_width,
        )
        .seal()
        .as_str()
        .to_string()
}

#[cfg(test)]
pub(super) fn compose_preview_execution_comparison_admission_digest(
    preview_execution_digest: &str,
    preview_comparison_digest: &str,
    candidate_comparison_digest: &str,
    candidate_basis_digest: &str,
    candidate_result_digest: &str,
) -> String {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_preview_execution_comparison_admission_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("preview_execution"),
            preview_execution_digest,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("preview_comparison"),
            preview_comparison_digest,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("candidate_comparison"),
            candidate_comparison_digest,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("candidate_basis"),
            candidate_basis_digest,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("candidate_result"),
            candidate_result_digest,
        )
        .seal()
        .as_str()
        .to_string()
}

#[cfg(test)]
pub(super) fn compose_preview_execution_report_digest(
    binding_digest: &str,
    basis_digest: &str,
    preview_session_identity: &BridgePreviewSessionIdentity,
    lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    execution_record_identity: &PreviewExecutionRecordIdentity,
    result_digest: &str,
    comparison_eligibility_digest: &str,
    workflow_foundation_digest: &str,
) -> String {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_preview_execution_report_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("binding"), binding_digest)
        .field_shape(WorthQueryEvidenceTag::new("basis"), basis_digest)
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("preview_session"),
            &preview_session_identity.bridge_admission_evidence(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("lifecycle"),
            preview_lifecycle_state_label(lifecycle_state_kind),
        )
        .field_bridge_retained_evidence_identity(
            WorthQueryEvidenceTag::new("execution_record"),
            &execution_record_identity.bridge_admission_evidence(),
        )
        .field_shape(WorthQueryEvidenceTag::new("result"), result_digest)
        .field_shape(
            WorthQueryEvidenceTag::new("comparison"),
            comparison_eligibility_digest,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("workflow"),
            workflow_foundation_digest,
        )
        .seal()
        .as_str()
        .to_string()
}

pub(super) fn compose_preview_live_admission_digest(
    preview_binding_digest: &str,
    live_subscription_digest: &str,
    live_family: &str,
) -> String {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_preview_live_admission_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("preview_binding"),
            preview_binding_digest,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("live_subscription"),
            live_subscription_digest,
        )
        .field_shape(WorthQueryEvidenceTag::new("live_family"), live_family)
        .seal()
        .as_str()
        .to_string()
}
