use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_COMPACTION_PLAN_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CompactionPlanIdentity(u64);

impl CompactionPlanIdentity {
    pub(super) fn issue() -> Self {
        Self(NEXT_COMPACTION_PLAN_IDENTITY.fetch_add(1, Ordering::Relaxed))
    }
}
