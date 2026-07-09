use crate::PageLsn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageLsnSkipApplyDecision {
    SkipAlreadyApplied {
        page_lsn: PageLsn,
        redo_lsn: PageLsn,
    },
    ApplyRedo {
        page_lsn: PageLsn,
        redo_lsn: PageLsn,
    },
}

impl PageLsnSkipApplyDecision {
    pub const fn decide(page_lsn: PageLsn, redo_lsn: PageLsn) -> Self {
        if page_lsn.is_at_or_beyond(redo_lsn) {
            Self::SkipAlreadyApplied { page_lsn, redo_lsn }
        } else {
            Self::ApplyRedo { page_lsn, redo_lsn }
        }
    }
}
