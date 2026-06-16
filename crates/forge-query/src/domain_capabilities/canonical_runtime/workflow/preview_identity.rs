use forge_runtime_bridge::facade::BridgePreviewSessionDeclarationIdentity;

use crate::domain_capabilities::payloads::ForgeQueryWorkflowContributionPayload;
use crate::identity::{CanonicalQueryDigest, ValidatedQueryDigest};

pub(super) fn preview_canonical_query_identity(
    source_label: &str,
    binding_identity: &crate::ForgeQueryEvidenceIdentity,
    request_identity: &crate::ForgeQueryEvidenceIdentity,
    preview_session_identity: &forge_runtime_bridge::facade::BridgePreviewSessionIdentity,
    evaluation_class: &crate::workflow::WorkflowPreviewEvaluationClass,
    request_family_label: &str,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::WorkflowContextBinding,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("identity_family"),
        "forge_query_domain_preview_query_v1",
    )
    .field_shape(crate::ForgeQueryEvidenceTag::new("source"), source_label)
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("binding"),
        binding_identity,
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("request"),
        request_identity,
    )
    .field_bridge_authority_identity(
        crate::ForgeQueryEvidenceTag::new("preview_session"),
        &preview_session_identity.bridge_trust_boundary(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("evaluation"),
        evaluation_class.as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("request_family"),
        request_family_label,
    )
    .seal()
}

pub(super) fn preview_validated_query_identity(
    canonical_query_identity: &crate::ForgeQueryEvidenceIdentity,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::WorkflowContextBinding,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("identity_family"),
        "forge_query_domain_preview_validated_query_v1",
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("canonical"),
        canonical_query_identity,
    )
    .seal()
}

pub(super) fn preview_declaration_identity(
    payload: &ForgeQueryWorkflowContributionPayload,
    binding_identity: &crate::ForgeQueryEvidenceIdentity,
    request_identity: &crate::ForgeQueryEvidenceIdentity,
    preview_session_identity: &forge_runtime_bridge::facade::BridgePreviewSessionIdentity,
    evaluation_class: &crate::workflow::WorkflowPreviewEvaluationClass,
    request_family_label: &str,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::WorkflowContextBinding,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("identity_family"),
        "domain_preview_declaration_v1",
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("semantic_code"),
        payload.semantic_code(),
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("binding"),
        binding_identity,
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("request"),
        request_identity,
    )
    .field_bridge_authority_identity(
        crate::ForgeQueryEvidenceTag::new("preview_session"),
        &preview_session_identity.bridge_trust_boundary(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("evaluation"),
        evaluation_class.as_str(),
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("request_family"),
        request_family_label,
    )
    .seal()
}

pub(super) fn canonical_query_digest_from_identity(
    identity: &crate::ForgeQueryEvidenceIdentity,
) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_evidence_identity(identity)
}

pub(super) fn validated_query_digest_from_identity(
    identity: &crate::ForgeQueryEvidenceIdentity,
) -> ValidatedQueryDigest {
    ValidatedQueryDigest::from_evidence_identity(identity)
}

pub(super) fn sealed_preview_declaration_bridge_identity(
    identity: &crate::ForgeQueryEvidenceIdentity,
) -> BridgePreviewSessionDeclarationIdentity {
    BridgePreviewSessionDeclarationIdentity::from_bridge_evidence(
        &identity.bridge_external_identity_evidence(),
    )
}

pub(super) fn preview_declaration_digest_identity(
    preview_declaration_identity: &crate::ForgeQueryEvidenceIdentity,
    canonical_query_digest: &CanonicalQueryDigest,
    validated_query_digest: &ValidatedQueryDigest,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::WorkflowContextBinding,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("identity_family"),
        "forge_query_domain_preview_declaration_v1",
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("declaration"),
        preview_declaration_identity,
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("canonical"),
        &domain_preview_canonical_query_digest_evidence(canonical_query_digest),
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("validated"),
        &domain_preview_validated_query_digest_evidence(validated_query_digest),
    )
    .seal()
}

fn domain_preview_canonical_query_digest_evidence(
    digest: &CanonicalQueryDigest,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::WorkflowContextBinding,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("identity_family"),
        "domain_preview_canonical_query_digest_evidence_v1",
    )
    .field_value(
        crate::ForgeQueryEvidenceTag::new("canonical_query_digest"),
        digest.as_str(),
    )
    .seal()
}

fn domain_preview_validated_query_digest_evidence(
    digest: &ValidatedQueryDigest,
) -> crate::ForgeQueryEvidenceIdentity {
    crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::WorkflowContextBinding,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("identity_family"),
        "domain_preview_validated_query_digest_evidence_v1",
    )
    .field_value(
        crate::ForgeQueryEvidenceTag::new("validated_query_digest"),
        digest.as_str(),
    )
    .seal()
}
