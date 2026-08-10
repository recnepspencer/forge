use crate::identity::{
    CanonicalQueryDigest, CanonicalResultShapeDigest, ValidatedQueryDigest,
    ValidatedResultShapeDigest,
};
#[cfg(test)]
use crate::preview::PreviewSessionBindingTuple;
use crate::workflow::{
    workflow_canonical_query_digest_evidence, workflow_validated_query_digest_evidence,
};
use crate::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag};
use worth_runtime_bridge::facade::{
    BridgePreviewLifecycleStateKind, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, PreviewExecutionRecordIdentity,
};

use super::super::PreviewEvaluationClass;

pub(in crate::preview) fn compose_preview_session_binding_tuple_digest(
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
                super::preview_lifecycle_identity::preview_lifecycle_state_label(
                    lifecycle_state_kind,
                ),
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
pub(in crate::preview) fn compose_preview_binding_tuple_workflow_identity(
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
pub(in crate::preview) fn compose_preview_declaration_digest_workflow_identity(
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
