use super::checkpoint::PlanarBooleanLoopCheckpointParityReceipt;
use super::counters::PlanarBooleanLoopReplayParityCounters;
use super::row::PlanarBooleanLoopReplayParityRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReplayParityReceipt {
    replay_identity: String,
    checkpoint_receipt: PlanarBooleanLoopCheckpointParityReceipt,
    rows: Vec<PlanarBooleanLoopReplayParityRow>,
    counters: PlanarBooleanLoopReplayParityCounters,
}

impl PlanarBooleanLoopReplayParityReceipt {
    pub(crate) fn new(
        replay_identity: String,
        checkpoint_receipt: PlanarBooleanLoopCheckpointParityReceipt,
        rows: Vec<PlanarBooleanLoopReplayParityRow>,
        counters: PlanarBooleanLoopReplayParityCounters,
    ) -> Self {
        Self {
            replay_identity,
            checkpoint_receipt,
            rows,
            counters,
        }
    }

    pub fn replay_identity(&self) -> &str {
        &self.replay_identity
    }

    pub fn checkpoint_receipt(&self) -> &PlanarBooleanLoopCheckpointParityReceipt {
        &self.checkpoint_receipt
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopReplayParityRow] {
        &self.rows
    }

    pub fn counters(&self) -> PlanarBooleanLoopReplayParityCounters {
        self.counters
    }
}
