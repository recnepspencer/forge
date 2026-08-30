use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub(crate) struct RelationalBranchSharingCostCell {
    counters: Arc<Mutex<crate::runtime::RelationalBranchSharingCostCounters>>,
}

impl RelationalBranchSharingCostCell {
    pub(crate) fn detached_owner_snapshot(&self) -> Self {
        Self {
            counters: Arc::new(Mutex::new(self.snapshot())),
        }
    }

    pub(crate) fn record(
        &self,
        update: impl FnOnce(&mut crate::runtime::RelationalBranchSharingCostCounters),
    ) {
        let mut counters = self
            .counters
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        update(&mut counters);
    }

    pub(crate) fn snapshot(&self) -> crate::runtime::RelationalBranchSharingCostCounters {
        *self
            .counters
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}
