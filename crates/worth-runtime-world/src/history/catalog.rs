use crate::budget::RuntimeWorldBudgetLimit;

/// Phase 1 installation contract for the later history catalog. The catalog
/// lane receives named limits without taking ownership of the lifecycle root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RuntimeWorldHistoryCatalogContract {
    maximum_commits: RuntimeWorldBudgetLimit,
    maximum_metadata_bytes: RuntimeWorldBudgetLimit,
}

impl RuntimeWorldHistoryCatalogContract {
    pub(crate) const fn installed(
        maximum_commits: RuntimeWorldBudgetLimit,
        maximum_metadata_bytes: RuntimeWorldBudgetLimit,
    ) -> Self {
        Self {
            maximum_commits,
            maximum_metadata_bytes,
        }
    }

    pub(crate) const fn maximum_commits(self) -> RuntimeWorldBudgetLimit {
        self.maximum_commits
    }

    pub(crate) const fn maximum_metadata_bytes(self) -> RuntimeWorldBudgetLimit {
        self.maximum_metadata_bytes
    }
}
