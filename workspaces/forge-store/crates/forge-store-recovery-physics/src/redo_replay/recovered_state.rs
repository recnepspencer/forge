use crate::source_precedence::RecoverySourceReplayBasis;
use crate::{PageLsn, RecoverySourceDecisionTrace};

use super::{RecoveredStateProjection, RedoApplicationPageFact, SkippedRedoFrameReport};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveredPhysicalState {
    recovered_physical_root: String,
    page_lsn_frontier: Option<PageLsn>,
    source_replay_basis: RecoverySourceReplayBasis,
    source_decision_digest: String,
    applied_frame_count: usize,
    skipped_frame_count: usize,
}

impl RecoveredPhysicalState {
    pub fn from_control_rebuild(
        source_trace: &RecoverySourceDecisionTrace,
        pages: &[RedoApplicationPageFact],
    ) -> Self {
        RecoveredStateProjection::from_control_rebuild(source_trace, pages).materialize()
    }

    pub(crate) fn from_pages(
        source_trace: &RecoverySourceDecisionTrace,
        pages: &[RedoApplicationPageFact],
        applied_frame_count: usize,
        skipped_frames: &[SkippedRedoFrameReport],
    ) -> Self {
        RecoveredStateProjection::from_replay(
            source_trace,
            pages,
            applied_frame_count,
            skipped_frames,
        )
        .materialize()
    }

    pub(crate) fn from_projected_parts(
        recovered_physical_root: String,
        page_lsn_frontier: Option<PageLsn>,
        source_replay_basis: RecoverySourceReplayBasis,
        source_decision_digest: String,
        applied_frame_count: usize,
        skipped_frame_count: usize,
    ) -> Self {
        Self {
            recovered_physical_root,
            page_lsn_frontier,
            source_replay_basis,
            source_decision_digest,
            applied_frame_count,
            skipped_frame_count,
        }
    }

    pub fn recovered_physical_root(&self) -> &str {
        &self.recovered_physical_root
    }

    pub const fn page_lsn_frontier(&self) -> Option<PageLsn> {
        self.page_lsn_frontier
    }

    pub(crate) const fn source_replay_basis(&self) -> &RecoverySourceReplayBasis {
        &self.source_replay_basis
    }

    pub fn source_decision_digest(&self) -> &str {
        &self.source_decision_digest
    }

    pub const fn applied_frame_count(&self) -> usize {
        self.applied_frame_count
    }

    pub const fn skipped_frame_count(&self) -> usize {
        self.skipped_frame_count
    }

    pub(crate) fn has_same_recovered_contents(&self, other: &Self) -> bool {
        self.recovered_physical_root == other.recovered_physical_root
            && self.page_lsn_frontier == other.page_lsn_frontier
            && self.applied_frame_count == other.applied_frame_count
            && self.skipped_frame_count == other.skipped_frame_count
    }
}
