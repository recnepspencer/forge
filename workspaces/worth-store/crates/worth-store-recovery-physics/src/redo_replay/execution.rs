use super::{
    RecoveredPhysicalState, RecoveryRedoPlan, RedoApplicationCursor, RedoApplicationPageFact,
    RedoPlanningDenial, SkippedRedoFrameReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedoExecutionReceipt {
    recovered_state: RecoveredPhysicalState,
    planned_frame_count: usize,
    applied_frame_count: usize,
    skipped_frames: Vec<SkippedRedoFrameReport>,
    recovered_pages: Vec<RedoApplicationPageFact>,
}

impl RedoExecutionReceipt {
    pub(crate) fn from_plan(
        plan: &RecoveryRedoPlan,
        cursor: &RedoApplicationCursor,
    ) -> Result<Self, RedoPlanningDenial> {
        let executed_frames = apply_planned_redo_frames(plan, cursor)?;
        let recovered_pages = capture_recovered_cursor_pages(&executed_frames.cursor);
        let recovered_state = materialize_execution_recovered_state(
            plan,
            &recovered_pages,
            executed_frames.applied_frame_count,
            &executed_frames.skipped_frames,
        );
        Ok(Self {
            recovered_state,
            planned_frame_count: plan.frames().len(),
            applied_frame_count: executed_frames.applied_frame_count,
            skipped_frames: executed_frames.skipped_frames,
            recovered_pages,
        })
    }

    pub const fn recovered_state(&self) -> &RecoveredPhysicalState {
        &self.recovered_state
    }

    pub const fn planned_frame_count(&self) -> usize {
        self.planned_frame_count
    }

    pub const fn applied_frame_count(&self) -> usize {
        self.applied_frame_count
    }

    pub fn skipped_frames(&self) -> &[SkippedRedoFrameReport] {
        &self.skipped_frames
    }

    pub fn recovered_cursor(&self) -> Result<RedoApplicationCursor, RedoPlanningDenial> {
        RedoApplicationCursor::new(self.recovered_pages.clone())
    }
}

struct ExecutedRedoFrames {
    cursor: RedoApplicationCursor,
    applied_frame_count: usize,
    skipped_frames: Vec<SkippedRedoFrameReport>,
}

fn apply_planned_redo_frames(
    plan: &RecoveryRedoPlan,
    cursor: &RedoApplicationCursor,
) -> Result<ExecutedRedoFrames, RedoPlanningDenial> {
    let mut cursor = cursor.clone();
    let mut applied_frame_count = 0usize;
    let mut skipped_frames = Vec::new();
    for frame in plan.frames() {
        if cursor.apply_frame(frame)? {
            skipped_frames.push(SkippedRedoFrameReport::already_current_page(
                frame.redo_lsn(),
                frame.target_page(),
            ));
        } else {
            applied_frame_count += 1;
        }
    }
    Ok(ExecutedRedoFrames {
        cursor,
        applied_frame_count,
        skipped_frames,
    })
}

fn capture_recovered_cursor_pages(cursor: &RedoApplicationCursor) -> Vec<RedoApplicationPageFact> {
    cursor.pages().to_vec()
}

fn materialize_execution_recovered_state(
    plan: &RecoveryRedoPlan,
    recovered_pages: &[RedoApplicationPageFact],
    applied_frame_count: usize,
    skipped_frames: &[SkippedRedoFrameReport],
) -> RecoveredPhysicalState {
    RecoveredPhysicalState::from_pages(
        plan.source_trace(),
        recovered_pages,
        applied_frame_count,
        skipped_frames,
    )
}
