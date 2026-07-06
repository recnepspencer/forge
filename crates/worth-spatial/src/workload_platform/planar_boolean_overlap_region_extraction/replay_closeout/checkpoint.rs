use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::evidence::PlanarBooleanOverlapRegionEvidenceReceipt;
use super::replay::{
    PlanarBooleanOverlapRegionReplayParityCounters, PlanarBooleanOverlapRegionReplayParityDenial,
    PlanarBooleanOverlapRegionReplayParityDenialKind,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCheckpointParityReceipt {
    checkpoint_identity: String,
    replay_evidence_identity: String,
}

pub struct ComparePlanarBooleanOverlapRegionCheckpointParity;

impl ComparePlanarBooleanOverlapRegionCheckpointParity {
    pub fn compare(
        original: &PlanarBooleanOverlapRegionEvidenceReceipt,
        replayed: &PlanarBooleanOverlapRegionEvidenceReceipt,
        replay_receipts: &ReplayReceiptSet,
        counters: &mut PlanarBooleanOverlapRegionReplayParityCounters,
    ) -> Result<
        PlanarBooleanOverlapRegionCheckpointParityReceipt,
        PlanarBooleanOverlapRegionReplayParityDenial,
    > {
        if original.replay_checkpoint_identity() != replay_receipts.replay_checkpoint_identity()
            || replayed.replay_checkpoint_identity() != replay_receipts.replay_checkpoint_identity()
            || original.replay_evidence_identity() != replay_receipts.replay_evidence_identity()
            || replayed.replay_evidence_identity() != replay_receipts.replay_evidence_identity()
        {
            counters.rejected_replay_mismatch();
            return Err(PlanarBooleanOverlapRegionReplayParityDenial::new(
                PlanarBooleanOverlapRegionReplayParityDenialKind::CheckpointAuthorityMismatch,
                original.replay_checkpoint_identity(),
                replay_receipts.replay_checkpoint_identity(),
                *counters,
            ));
        }

        Ok(PlanarBooleanOverlapRegionCheckpointParityReceipt {
            checkpoint_identity: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "planar-boolean-overlap-region-checkpoint-parity".to_string(),
                    format!(
                        "checkpoint:{}",
                        replay_receipts.replay_checkpoint_identity()
                    ),
                    format!(
                        "replay-evidence:{}",
                        replay_receipts.replay_evidence_identity()
                    ),
                ],
            ),
            replay_evidence_identity: replay_receipts.replay_evidence_identity().to_string(),
        })
    }
}

impl PlanarBooleanOverlapRegionCheckpointParityReceipt {
    pub fn checkpoint_identity(&self) -> &str {
        &self.checkpoint_identity
    }
    pub fn replay_evidence_identity(&self) -> &str {
        &self.replay_evidence_identity
    }
}
