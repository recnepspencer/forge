use super::super::parity_receipt::denial::{
    PlanarBooleanEdgeSplitReplayParityDenial, PlanarBooleanEdgeSplitReplayParityDenialKind as Kind,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

pub(super) fn validate_checkpoint_receipts(
    replay_receipts: &ReplayReceiptSet,
) -> Result<String, PlanarBooleanEdgeSplitReplayParityDenial> {
    if replay_receipts.replay_checkpoint_identity().is_empty() {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::CheckpointParityMismatch,
            "replay-checkpoint",
            "non-empty checkpoint identity",
            replay_receipts.replay_checkpoint_identity(),
            "checkpoint parity requires a retained replay checkpoint identity",
        ));
    }
    if replay_receipts.replay_evidence_identity().is_empty() {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::CheckpointParityMismatch,
            "replay-evidence",
            "non-empty replay evidence identity",
            replay_receipts.replay_evidence_identity(),
            "checkpoint parity requires retained replay evidence identity",
        ));
    }
    if replay_receipts.counters().replay_rows() == 0 {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::CheckpointParityMismatch,
            "replay-rows",
            "at least one replay row",
            "0",
            "checkpoint parity requires retained replay rows",
        ));
    }
    Ok(format!(
        "checkpoint:{}:{}",
        replay_receipts.replay_checkpoint_identity(),
        replay_receipts.replay_evidence_identity()
    ))
}
