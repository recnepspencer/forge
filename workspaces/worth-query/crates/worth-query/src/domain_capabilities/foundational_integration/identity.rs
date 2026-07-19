use sha2::{Digest, Sha256};
use worth_foundational::BoundaryArtifactId;

use crate::domain_capabilities::identity::domain_capability_scope_encoder;
use crate::domain_capabilities::payloads::WorthQueryDomainCapabilityCategory;
use crate::domain_capabilities::targets::WorthQueryDomainCapabilityTargetKind;
use crate::evidence_identity::{WorthQueryEvidenceIdentity, WorthQueryEvidenceTag};

pub(crate) fn diagnostic_scope_identity(
    category: WorthQueryDomainCapabilityCategory,
    target_kind: WorthQueryDomainCapabilityTargetKind,
) -> WorthQueryEvidenceIdentity {
    domain_capability_scope_encoder("worth_query_domain_capability_diagnostic_scope_v1")
        .field_shape(WorthQueryEvidenceTag::new("category"), category.as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("target_kind"),
            target_kind.as_str(),
        )
        .seal()
}

pub(crate) fn diagnostic_label_identity(
    role: &'static str,
    value: &str,
) -> WorthQueryEvidenceIdentity {
    domain_capability_scope_encoder("worth_query_domain_capability_diagnostic_label_v1")
        .field_shape(WorthQueryEvidenceTag::new("role"), role)
        .field_shape(WorthQueryEvidenceTag::new("value"), value)
        .seal()
}

pub(crate) fn diagnostic_code_identity(
    role: &'static str,
    semantic_code: &str,
) -> WorthQueryEvidenceIdentity {
    domain_capability_scope_encoder("worth_query_domain_capability_diagnostic_code_v1")
        .field_shape(WorthQueryEvidenceTag::new("role"), role)
        .field_shape(WorthQueryEvidenceTag::new("semantic_code"), semantic_code)
        .seal()
}

pub(crate) fn boundary_artifact_id(identity: &WorthQueryEvidenceIdentity) -> BoundaryArtifactId {
    let mut hasher = Sha256::new();
    hasher.update(identity.as_str().as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    BoundaryArtifactId::new(u64::from_be_bytes(bytes))
}
