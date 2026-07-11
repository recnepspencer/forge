use crate::WalLsnRange;

use super::{
    replay_index_family::ReplayIndexLayoutReport, RecoveryLayoutAccessDenial,
    RecoveryLayoutAccessDenialKind,
};

impl BoundedWalTailLayoutReport {
    pub fn lookup_tail_range(
        replay_index: &ReplayIndexLayoutReport,
        requested_range: WalLsnRange,
    ) -> Result<BoundedWalTailLayoutReport, RecoveryLayoutAccessDenial> {
        let frontier = replay_index.replay_frontier();
        if requested_range.start() < frontier.start()
            || requested_range.end_exclusive() > frontier.end_exclusive()
        {
            return Err(RecoveryLayoutAccessDenial::new(
                RecoveryLayoutAccessDenialKind::BoundedWalTailLookupOutOfRange,
            ));
        }
        Ok(BoundedWalTailLayoutReport {
            requested_range,
            replay_frontier: frontier,
            ordered_range_count: replay_index.ordered_range_count(),
            segment_count: replay_index.segment_count(),
        })
    }
}

pub(crate) fn lookup_recovery_tail_range(
    replay_index: &ReplayIndexLayoutReport,
    requested_range: WalLsnRange,
) -> Result<BoundedWalTailLayoutReport, RecoveryLayoutAccessDenial> {
    BoundedWalTailLayoutReport::lookup_tail_range(replay_index, requested_range)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedWalTailLayoutReport {
    requested_range: WalLsnRange,
    replay_frontier: WalLsnRange,
    ordered_range_count: usize,
    segment_count: usize,
}

impl BoundedWalTailLayoutReport {
    pub const fn requested_range(&self) -> WalLsnRange {
        self.requested_range
    }

    pub const fn replay_frontier(&self) -> WalLsnRange {
        self.replay_frontier
    }

    pub const fn ordered_range_count(&self) -> usize {
        self.ordered_range_count
    }

    pub const fn segment_count(&self) -> usize {
        self.segment_count
    }
}
