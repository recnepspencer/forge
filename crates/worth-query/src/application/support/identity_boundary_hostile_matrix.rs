#[path = "identity_boundary_hostile_digest_rows.rs"]
mod identity_boundary_hostile_digest_rows;
#[path = "identity_boundary_hostile_stop_rows.rs"]
mod identity_boundary_hostile_stop_rows;

use identity_boundary_hostile_digest_rows::{
    authoritative_intent_receipt_identity_delimiter_boundaries_row,
    effect_intent_receipt_identity_delimiter_boundaries_row,
    evidence_identity_delimiter_collision_resistance_row,
    intent_provenance_chain_identity_delimiter_boundaries_row,
    preview_intent_receipt_inspection_basis_identity_delimiter_boundaries_row,
    preview_intent_receipt_inspection_identity_delimiter_boundaries_row,
};
use identity_boundary_hostile_stop_rows::{
    consumer_message_substring_routing_drift_row, family_admission_message_rewording_stability_row,
    graph_domain_invariant_message_rewording_stability_row,
    joined_string_evidence_identity_collapse_row, session_label_render_collision_distinctness_row,
    session_label_same_family_replay_collision_row,
};

use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIdentityBoundaryHostileMatrixRow {
    name: &'static str,
    certified: bool,
    witness_digest: String,
}

impl WorthQueryIdentityBoundaryHostileMatrixRow {
    pub(super) fn new(name: &'static str, certified: bool, witness_digest: String) -> Self {
        Self {
            name,
            certified,
            witness_digest,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn certified(&self) -> bool {
        self.certified
    }

    pub fn witness_digest(&self) -> &str {
        &self.witness_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIdentityBoundaryHostileMatrixArtifact {
    suite_name: &'static str,
    certified: bool,
    canonical_rows: Vec<WorthQueryIdentityBoundaryHostileMatrixRow>,
    rejection_rows: Vec<WorthQueryIdentityBoundaryHostileMatrixRow>,
    artifact_digest: String,
}

#[allow(dead_code)]
impl WorthQueryIdentityBoundaryHostileMatrixArtifact {
    pub fn suite_name(&self) -> &'static str {
        self.suite_name
    }

    pub fn certified(&self) -> bool {
        self.certified
    }

    pub fn canonical_rows(&self) -> &[WorthQueryIdentityBoundaryHostileMatrixRow] {
        &self.canonical_rows
    }

    pub fn rejection_rows(&self) -> &[WorthQueryIdentityBoundaryHostileMatrixRow] {
        &self.rejection_rows
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}

pub const MILESTONE_NINE_SIX_SUITE_NAME: &str =
    "Milestone 9.6 Identity And Stop-Class Hostile Certification Matrix";

#[allow(dead_code)]
pub const MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "evidence-identity-delimiter-collision-resistance",
    "authoritative-intent-receipt-identity-delimiter-boundaries",
    "effect-intent-receipt-identity-delimiter-boundaries",
    "intent-provenance-chain-identity-delimiter-boundaries",
    "preview-intent-receipt-inspection-basis-identity-delimiter-boundaries",
    "preview-intent-receipt-inspection-identity-delimiter-boundaries",
    "family-admission-message-rewording-stability",
    "graph-domain-invariant-message-rewording-stability",
    "session-label-render-collision-distinctness",
    "session-label-same-family-replay-collision",
];

#[allow(dead_code)]
pub const MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "joined-string-evidence-identity-collapses-distinct-fields",
    "consumer-message-substring-routing-drifts",
];

pub fn identity_boundary_hostile_matrix_artifact() -> WorthQueryIdentityBoundaryHostileMatrixArtifact
{
    let canonical_rows = vec![
        evidence_identity_delimiter_collision_resistance_row(),
        authoritative_intent_receipt_identity_delimiter_boundaries_row(),
        effect_intent_receipt_identity_delimiter_boundaries_row(),
        intent_provenance_chain_identity_delimiter_boundaries_row(),
        preview_intent_receipt_inspection_basis_identity_delimiter_boundaries_row(),
        preview_intent_receipt_inspection_identity_delimiter_boundaries_row(),
        family_admission_message_rewording_stability_row(),
        graph_domain_invariant_message_rewording_stability_row(),
        session_label_render_collision_distinctness_row(),
        session_label_same_family_replay_collision_row(),
    ];
    let rejection_rows = vec![
        joined_string_evidence_identity_collapse_row(),
        consumer_message_substring_routing_drift_row(),
    ];
    let certified = canonical_rows
        .iter()
        .chain(rejection_rows.iter())
        .all(WorthQueryIdentityBoundaryHostileMatrixRow::certified);
    let artifact_digest =
        worth_query_evidence_identity(WorthQueryEvidenceScope::RuntimeHostileCertificationArtifact)
            .field_shape(
                WorthQueryEvidenceTag::new("suite_name"),
                MILESTONE_NINE_SIX_SUITE_NAME,
            )
            .field_bool(WorthQueryEvidenceTag::new("certified"), certified)
            .field_value_sequence(
                WorthQueryEvidenceTag::new("canonical_row"),
                canonical_rows
                    .iter()
                    .map(WorthQueryIdentityBoundaryHostileMatrixRow::name),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("canonical_row_witness"),
                canonical_rows
                    .iter()
                    .map(WorthQueryIdentityBoundaryHostileMatrixRow::witness_digest),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("rejection_row"),
                rejection_rows
                    .iter()
                    .map(WorthQueryIdentityBoundaryHostileMatrixRow::name),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("rejection_row_witness"),
                rejection_rows
                    .iter()
                    .map(WorthQueryIdentityBoundaryHostileMatrixRow::witness_digest),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("combined_drift_axes"),
                "digest-delimiter|message-reword|label-collision",
            )
            .seal()
            .as_str()
            .to_string();
    WorthQueryIdentityBoundaryHostileMatrixArtifact {
        suite_name: MILESTONE_NINE_SIX_SUITE_NAME,
        certified,
        canonical_rows,
        rejection_rows,
        artifact_digest,
    }
}

#[allow(dead_code)]
pub fn identity_boundary_hostile_matrix_digest() -> String {
    identity_boundary_hostile_matrix_artifact()
        .artifact_digest()
        .to_string()
}
