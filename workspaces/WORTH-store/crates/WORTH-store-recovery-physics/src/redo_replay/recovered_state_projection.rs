use crate::{PageLsn, RecoverySourceDecisionTrace};

use super::{RecoveredPhysicalState, RedoApplicationPageFact, SkippedRedoFrameReport};

pub(crate) struct RecoveredStateProjection<'a> {
    source_trace: &'a RecoverySourceDecisionTrace,
    pages: &'a [RedoApplicationPageFact],
    applied_frame_count: usize,
    skipped_frame_count: usize,
}

impl<'a> RecoveredStateProjection<'a> {
    pub(crate) const fn from_replay(
        source_trace: &'a RecoverySourceDecisionTrace,
        pages: &'a [RedoApplicationPageFact],
        applied_frame_count: usize,
        skipped_frames: &'a [SkippedRedoFrameReport],
    ) -> Self {
        Self {
            source_trace,
            pages,
            applied_frame_count,
            skipped_frame_count: skipped_frames.len(),
        }
    }

    pub(crate) const fn from_control_rebuild(
        source_trace: &'a RecoverySourceDecisionTrace,
        pages: &'a [RedoApplicationPageFact],
    ) -> Self {
        Self {
            source_trace,
            pages,
            applied_frame_count: 0,
            skipped_frame_count: 0,
        }
    }

    pub(crate) fn materialize(self) -> RecoveredPhysicalState {
        RecoveredPhysicalState::from_projected_parts(
            self.recovered_physical_root(),
            self.page_lsn_frontier(),
            self.source_decision_digest(),
            self.applied_frame_count,
            self.skipped_frame_count,
        )
    }

    fn recovered_physical_root(&self) -> String {
        let page_digest = self
            .pages
            .iter()
            .map(|page| {
                format!(
                    "{}:{}:{}",
                    page.page_id().get(),
                    page.digest_state().page_lsn().lsn().get(),
                    page.digest_state().physical_state_digest()
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        format!("s4-redo-root[{page_digest}]")
    }

    fn page_lsn_frontier(&self) -> Option<PageLsn> {
        self.pages
            .iter()
            .map(|page| page.digest_state().page_lsn())
            .max()
    }

    fn source_decision_digest(&self) -> String {
        self.source_trace.canonical_replay_digest()
    }
}
