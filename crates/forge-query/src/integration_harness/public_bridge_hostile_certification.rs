use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicBridgeHostileCertificationComposeInput {
    pub pending_artifact: String,
    pub branch_basis_a: String,
    pub branch_basis_b: String,
    pub preview_discard: String,
    pub receipt_one: ForgeQueryEvidenceIdentity,
    pub title_one: String,
    pub receipt_two: ForgeQueryEvidenceIdentity,
    pub title_two: String,
    pub preview_promote: String,
    pub title_three: String,
}

pub fn public_bridge_hostile_certification_evidence_label(
    identity: &ForgeQueryEvidenceIdentity,
) -> String {
    identity.reporting_projection().to_string()
}

pub fn public_bridge_hostile_published_artifact_component_digest(
    snapshot_identity: &ForgeQueryEvidenceIdentity,
    binding_for_reporting: &str,
    title: &str,
) -> String {
    public_bridge_hostile_certification_evidence_label(
        &forge_query_evidence_identity(
            ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("snapshot"), snapshot_identity)
        .field_value(ForgeQueryEvidenceTag::new("binding"), binding_for_reporting)
        .field_shape(ForgeQueryEvidenceTag::new("title"), title)
        .seal(),
    )
}

pub fn compose_public_bridge_hostile_certification_digest(
    input: PublicBridgeHostileCertificationComposeInput,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact)
        .field_value(
            ForgeQueryEvidenceTag::new("pending_artifact"),
            input.pending_artifact,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("branch_basis_a"),
            input.branch_basis_a,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("branch_basis_b"),
            input.branch_basis_b,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("preview_discard"),
            input.preview_discard,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_one"),
            &input.receipt_one,
        )
        .field_shape(ForgeQueryEvidenceTag::new("title_one"), input.title_one)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("receipt_two"),
            &input.receipt_two,
        )
        .field_shape(ForgeQueryEvidenceTag::new("title_two"), input.title_two)
        .field_value(
            ForgeQueryEvidenceTag::new("preview_promote"),
            input.preview_promote,
        )
        .field_shape(ForgeQueryEvidenceTag::new("title_three"), input.title_three)
        .seal()
}
