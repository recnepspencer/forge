use crate::RecoveryRedoPlan;

use super::{RecoveryBudgetDenial, RecoveryBudgetDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalTailReplayBudget {
    max_frames: usize,
    max_scanned_segments: usize,
    max_page_redos: usize,
}

impl WalTailReplayBudget {
    pub const fn max_frames(max_frames: usize) -> Self {
        Self {
            max_frames,
            max_scanned_segments: usize::MAX,
            max_page_redos: usize::MAX,
        }
    }

    pub const fn with_max_scanned_segments(mut self, max_scanned_segments: usize) -> Self {
        self.max_scanned_segments = max_scanned_segments;
        self
    }

    pub const fn with_max_page_redos(mut self, max_page_redos: usize) -> Self {
        self.max_page_redos = max_page_redos;
        self
    }

    pub const fn max_frame_count(self) -> usize {
        self.max_frames
    }

    pub const fn max_scanned_segments(self) -> usize {
        self.max_scanned_segments
    }

    pub const fn max_page_redos(self) -> usize {
        self.max_page_redos
    }

    pub(crate) fn admit_replay_work(
        self,
        plan: &RecoveryRedoPlan,
        scanned_segments: usize,
    ) -> Result<(), RecoveryBudgetDenial> {
        let planned_frames = plan.expected().planned_frames();
        admit_replay_frame_count(planned_frames, self.max_frames)?;
        admit_scanned_segment_count(scanned_segments, self.max_scanned_segments)?;
        let planned_page_redos = planned_frames;
        admit_page_redo_count(planned_page_redos, self.max_page_redos)?;
        Ok(())
    }
}

fn admit_replay_frame_count(
    planned_frames: usize,
    max_frames: usize,
) -> Result<(), RecoveryBudgetDenial> {
    if planned_frames > max_frames {
        return Err(RecoveryBudgetDenial::new(
            RecoveryBudgetDenialKind::WalTailFrameBudgetExceeded {
                planned: planned_frames,
                max: max_frames,
            },
        ));
    }
    Ok(())
}

fn admit_scanned_segment_count(
    scanned_segments: usize,
    max_scanned_segments: usize,
) -> Result<(), RecoveryBudgetDenial> {
    if scanned_segments > max_scanned_segments {
        return Err(RecoveryBudgetDenial::new(
            RecoveryBudgetDenialKind::WalTailSegmentBudgetExceeded {
                scanned: scanned_segments,
                max: max_scanned_segments,
            },
        ));
    }
    Ok(())
}

fn admit_page_redo_count(
    planned_page_redos: usize,
    max_page_redos: usize,
) -> Result<(), RecoveryBudgetDenial> {
    if planned_page_redos > max_page_redos {
        return Err(RecoveryBudgetDenial::new(
            RecoveryBudgetDenialKind::PageRedoBudgetExceeded {
                planned: planned_page_redos,
                max: max_page_redos,
            },
        ));
    }
    Ok(())
}
