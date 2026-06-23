use super::replay_rows::PlanarBooleanEdgeSplitReplayParityRow;

pub(super) fn replay_parity_identity(
    retained_replay_stage_identity: &str,
    replay_checkpoint_identity: &str,
    replay_evidence_identity: &str,
    rows: &[PlanarBooleanEdgeSplitReplayParityRow],
) -> String {
    let mut identity = format!(
        "edge-split-replay-parity:{retained_replay_stage_identity}:{replay_checkpoint_identity}:{replay_evidence_identity}"
    );
    for row in rows {
        identity.push_str(":row:");
        identity.push_str(row.parity_row_identity());
    }
    identity
}

pub(super) fn validator_receipt_identity(parity_identity: &str) -> String {
    format!("validate-planar-boolean-replay-parity:{parity_identity}")
}
