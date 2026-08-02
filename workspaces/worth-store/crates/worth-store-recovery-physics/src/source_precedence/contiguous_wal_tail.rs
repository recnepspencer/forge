use crate::{
    CheckpointCutoverReceipt, CheckpointId, CheckpointValidationDenial,
    CheckpointValidationDenialKind, WalLsnRange,
};

/// Recovery evidence that a WAL tail begins at one checkpoint's exact boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContiguousWalTailProof {
    checkpoint_id: CheckpointId,
    tail_range: WalLsnRange,
}

impl ContiguousWalTailProof {
    pub fn prove(
        checkpoint: &CheckpointCutoverReceipt,
        tail_range: WalLsnRange,
    ) -> Result<Self, CheckpointValidationDenial> {
        if checkpoint.covered_lsn_range().range().end_exclusive() != tail_range.start() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::WalRetentionWithoutContiguousTail,
                checkpoint.counters().with_retention_decision(),
            )
            .with_lsn_pair(
                checkpoint.covered_lsn_range().range().end_exclusive(),
                tail_range.start(),
            ));
        }
        Ok(Self {
            checkpoint_id: checkpoint.checkpoint_id().clone(),
            tail_range,
        })
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn tail_range(&self) -> WalLsnRange {
        self.tail_range
    }
}
