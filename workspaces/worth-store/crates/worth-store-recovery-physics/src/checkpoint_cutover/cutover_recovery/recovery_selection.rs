use super::super::{CheckpointId, CheckpointRecoveryCounterSnapshot};
use super::RecoveredCheckpointCutoverState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointCutoverRecoverySelectionKind {
    SelectedCheckpoint,
    NoValidCheckpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointCutoverRecoverySelection {
    kind: CheckpointCutoverRecoverySelectionKind,
    checkpoint_id: Option<CheckpointId>,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl CheckpointCutoverRecoverySelection {
    pub fn from_recovered_state(state: RecoveredCheckpointCutoverState) -> Self {
        let counters = state
            .receipt()
            .map(|receipt| receipt.counters())
            .unwrap_or_default()
            .with_cutover_decision();
        if state.has_selected_checkpoint_basis() {
            let receipt = state
                .receipt()
                .expect("selected state checked receipt presence");
            return Self {
                kind: CheckpointCutoverRecoverySelectionKind::SelectedCheckpoint,
                checkpoint_id: Some(receipt.checkpoint_id().clone()),
                counters,
            };
        }
        Self {
            kind: CheckpointCutoverRecoverySelectionKind::NoValidCheckpoint,
            checkpoint_id: None,
            counters,
        }
    }

    pub const fn kind(&self) -> CheckpointCutoverRecoverySelectionKind {
        self.kind
    }

    pub fn checkpoint_id(&self) -> Option<&CheckpointId> {
        self.checkpoint_id.as_ref()
    }

    pub const fn counters(&self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}
