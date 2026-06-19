use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

use super::counters::PlanarBooleanLoopReplayParityCounters;
use super::denial::{PlanarBooleanLoopReplayParityDenial, PlanarBooleanLoopReplayParityDenialKind};
use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionEvidenceReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopCheckpointParityReceipt {
    checkpoint_identity: String,
    replay_evidence_identity: String,
}

pub struct ComparePlanarBooleanLoopCheckpointParity;

impl ComparePlanarBooleanLoopCheckpointParity {
    pub(crate) fn compare(
        original_evidence_receipt: &PlanarBooleanLoopReconstructionEvidenceReceipt,
        replayed_evidence_receipt: &PlanarBooleanLoopReconstructionEvidenceReceipt,
        replay_receipts: &ReplayReceiptSet,
        counters: &mut PlanarBooleanLoopReplayParityCounters,
    ) -> Result<PlanarBooleanLoopCheckpointParityReceipt, PlanarBooleanLoopReplayParityDenial> {
        if original_evidence_receipt.replay_checkpoint_identity()
            != replayed_evidence_receipt.replay_checkpoint_identity()
        {
            counters.rejected_replay_mismatch();
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::CheckpointAuthorityMismatch,
                original_evidence_receipt.replay_checkpoint_identity(),
                replayed_evidence_receipt.replay_checkpoint_identity(),
                *counters,
            ));
        }
        let checkpoint_identity = replay_receipts.replay_checkpoint_identity();
        if checkpoint_identity != original_evidence_receipt.replay_checkpoint_identity() {
            counters.rejected_replay_mismatch();
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::CheckpointAuthorityMismatch,
                original_evidence_receipt.replay_checkpoint_identity(),
                checkpoint_identity,
                *counters,
            ));
        }
        if checkpoint_identity.is_empty() {
            counters.rejected_replay_mismatch();
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::CheckpointParityMismatch,
                "non-empty retained replay checkpoint identity",
                checkpoint_identity,
                *counters,
            ));
        }
        let replay_evidence_identity = replay_receipts.replay_evidence_identity();
        if original_evidence_receipt.replay_evidence_identity()
            != replayed_evidence_receipt.replay_evidence_identity()
        {
            counters.rejected_replay_mismatch();
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::CheckpointAuthorityMismatch,
                original_evidence_receipt.replay_evidence_identity(),
                replayed_evidence_receipt.replay_evidence_identity(),
                *counters,
            ));
        }
        if replay_evidence_identity != original_evidence_receipt.replay_evidence_identity() {
            counters.rejected_replay_mismatch();
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::CheckpointAuthorityMismatch,
                original_evidence_receipt.replay_evidence_identity(),
                replay_evidence_identity,
                *counters,
            ));
        }
        if replay_evidence_identity.is_empty() {
            counters.rejected_replay_mismatch();
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::CheckpointParityMismatch,
                "non-empty retained replay evidence identity",
                replay_evidence_identity,
                *counters,
            ));
        }
        if replay_receipts.counters().replay_rows() == 0 {
            counters.rejected_replay_mismatch();
            return Err(PlanarBooleanLoopReplayParityDenial::new(
                PlanarBooleanLoopReplayParityDenialKind::CheckpointParityMismatch,
                "retained replay rows > 0",
                "0",
                *counters,
            ));
        }
        counters.compared_checkpoints();
        Ok(PlanarBooleanLoopCheckpointParityReceipt {
            checkpoint_identity: checkpoint_identity.to_string(),
            replay_evidence_identity: replay_evidence_identity.to_string(),
        })
    }
}

impl PlanarBooleanLoopCheckpointParityReceipt {
    pub fn checkpoint_identity(&self) -> &str {
        &self.checkpoint_identity
    }

    pub fn replay_evidence_identity(&self) -> &str {
        &self.replay_evidence_identity
    }
}
