use crate::{CheckpointBaseAdmission, WalTailRedoSource};

use super::{RecoveryBudgetDenial, RecoveryBudgetDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointIntervalContract {
    max_tail_frames: usize,
}

impl CheckpointIntervalContract {
    pub const fn max_tail_frames(max_tail_frames: usize) -> Self {
        Self { max_tail_frames }
    }

    pub const fn max_tail_frame_count(self) -> usize {
        self.max_tail_frames
    }

    pub(crate) fn admit_tail(
        self,
        checkpoint: &CheckpointBaseAdmission,
        tail: &WalTailRedoSource,
    ) -> Result<usize, RecoveryBudgetDenial> {
        let expected_tail_start = checkpoint.covered_lsn_range().end_exclusive();
        let tail_range = tail.lsn_range();
        if tail_range.start() != expected_tail_start {
            return Err(RecoveryBudgetDenial::new(
                RecoveryBudgetDenialKind::CheckpointIntervalMismatch {
                    checkpoint_end: expected_tail_start,
                    tail_start: tail_range.start(),
                },
            ));
        }
        let tail_frames = tail_range
            .end_exclusive()
            .get()
            .saturating_sub(tail_range.start().get()) as usize;
        if tail_frames > self.max_tail_frames {
            return Err(RecoveryBudgetDenial::new(
                RecoveryBudgetDenialKind::WalTailFrameBudgetExceeded {
                    planned: tail_frames,
                    max: self.max_tail_frames,
                },
            ));
        }
        Ok(tail_frames)
    }
}
