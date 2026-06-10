use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

pub const MILESTONE_NINE_SIX_SUITE_NAME: &str =
    "Milestone 9.6 Identity And Stop-Class Hostile Certification Matrix";

pub const MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "evidence-identity-delimiter-collision-resistance",
    "family-admission-message-rewording-stability",
    "graph-domain-invariant-message-rewording-stability",
    "session-label-render-collision-distinctness",
    "session-label-same-family-replay-collision",
];

pub const MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "joined-string-evidence-identity-collapses-distinct-fields",
    "consumer-message-substring-routing-drifts",
];

pub fn identity_boundary_hostile_matrix_digest() -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact)
        .field_shape(
            ForgeQueryEvidenceTag::new("suite_name"),
            MILESTONE_NINE_SIX_SUITE_NAME,
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("canonical_row"),
            MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES.iter().copied(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("rejection_row"),
            MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES.iter().copied(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("combined_drift_axes"),
            "digest-delimiter|message-reword|label-collision",
        )
        .seal()
        .as_str()
        .to_string()
}
