use crate::identity::hash_parts;

use super::super::matrix_kind::CausalInspectionRepresentativeKind;

pub(super) struct RowDigestParts<'a> {
    pub(super) kind: CausalInspectionRepresentativeKind,
    pub(super) query_digest: &'a str,
    pub(super) query_observation_receipt_digest: &'a str,
    pub(super) causal_observation_anchor_digest: &'a str,
    pub(super) inspection_digest: Option<&'a str>,
    pub(super) artifact_digest: Option<&'a str>,
    pub(super) causal_envelope_digest: Option<&'a str>,
    pub(super) evidence_reference_collection_digest: Option<&'a str>,
    pub(super) relational_authority_digest: Option<&'a str>,
    pub(super) bridge_route_digest: Option<&'a str>,
    pub(super) bridge_evaluation_digest: Option<&'a str>,
    pub(super) bridge_source_materialization_digest: Option<&'a str>,
    pub(super) bridge_structural_digest: Option<&'a str>,
    pub(super) bridge_stream_digest: Option<&'a str>,
    pub(super) bridge_preview_digest: Option<&'a str>,
    pub(super) bridge_writeback_digest: Option<&'a str>,
    pub(super) bridge_replay_digest: Option<&'a str>,
    pub(super) signal_invalidation_digest: Option<&'a str>,
    pub(super) signal_evaluation_digest: Option<&'a str>,
    pub(super) signal_forensic_availability_digest: Option<&'a str>,
    pub(super) signal_replay_cursor_digest: Option<&'a str>,
    pub(super) signal_lineage_digest: Option<&'a str>,
    pub(super) signal_provenance_digest: Option<&'a str>,
    pub(super) replay_posture_digest: Option<&'a str>,
    pub(super) materialization_policy_digest: Option<&'a str>,
    pub(super) redaction_policy_digest: Option<&'a str>,
    pub(super) materialization_receipt_digest: Option<&'a str>,
    pub(super) counter_snapshot_digest: Option<&'a str>,
    pub(super) failure_digest: Option<&'a str>,
}

pub(super) fn row_digest(parts: RowDigestParts<'_>) -> String {
    hash_parts(&[
        "causal_inspection_representative_row_digest_set_v1".to_string(),
        format!("kind:{}", parts.kind.as_str()),
        format!("query:{}", parts.query_digest),
        format!("observation:{}", parts.query_observation_receipt_digest),
        format!("anchor:{}", parts.causal_observation_anchor_digest),
        format!("inspection:{}", parts.inspection_digest.unwrap_or("none")),
        format!("artifact:{}", parts.artifact_digest.unwrap_or("none")),
        format!(
            "envelope:{}",
            parts.causal_envelope_digest.unwrap_or("none")
        ),
        format!(
            "references:{}",
            parts.evidence_reference_collection_digest.unwrap_or("none")
        ),
        format!(
            "relational-authority:{}",
            parts.relational_authority_digest.unwrap_or("none")
        ),
        format!(
            "bridge-route:{}",
            parts.bridge_route_digest.unwrap_or("none")
        ),
        format!(
            "bridge-evaluation:{}",
            parts.bridge_evaluation_digest.unwrap_or("none")
        ),
        format!(
            "bridge-source:{}",
            parts.bridge_source_materialization_digest.unwrap_or("none")
        ),
        format!(
            "bridge-structural:{}",
            parts.bridge_structural_digest.unwrap_or("none")
        ),
        format!(
            "bridge-stream:{}",
            parts.bridge_stream_digest.unwrap_or("none")
        ),
        format!(
            "bridge-preview:{}",
            parts.bridge_preview_digest.unwrap_or("none")
        ),
        format!(
            "bridge-writeback:{}",
            parts.bridge_writeback_digest.unwrap_or("none")
        ),
        format!(
            "bridge-replay:{}",
            parts.bridge_replay_digest.unwrap_or("none")
        ),
        format!(
            "signal-invalidation:{}",
            parts.signal_invalidation_digest.unwrap_or("none")
        ),
        format!(
            "signal-evaluation:{}",
            parts.signal_evaluation_digest.unwrap_or("none")
        ),
        format!(
            "signal-forensic-availability:{}",
            parts.signal_forensic_availability_digest.unwrap_or("none")
        ),
        format!(
            "signal-replay-cursor:{}",
            parts.signal_replay_cursor_digest.unwrap_or("none")
        ),
        format!(
            "signal-lineage:{}",
            parts.signal_lineage_digest.unwrap_or("none")
        ),
        format!(
            "signal-provenance:{}",
            parts.signal_provenance_digest.unwrap_or("none")
        ),
        format!(
            "replay-posture:{}",
            parts.replay_posture_digest.unwrap_or("none")
        ),
        format!(
            "materialization-policy:{}",
            parts.materialization_policy_digest.unwrap_or("none")
        ),
        format!(
            "redaction-policy:{}",
            parts.redaction_policy_digest.unwrap_or("none")
        ),
        format!(
            "materialization-receipt:{}",
            parts.materialization_receipt_digest.unwrap_or("none")
        ),
        format!(
            "counter:{}",
            parts.counter_snapshot_digest.unwrap_or("none")
        ),
        format!("failure:{}", parts.failure_digest.unwrap_or("none")),
    ])
}
