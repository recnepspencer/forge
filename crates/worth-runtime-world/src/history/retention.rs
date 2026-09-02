use std::sync::Arc;

use crate::identity::RuntimeWorldOwnerIdentity;
use crate::retention::ComponentBasisDependencyClass;

use super::catalog::{lock_index, HistoryReachabilityHandle};

/// Closed protection classes keep the history owner from accepting a raw
/// boolean or caller-defined retention authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::history) enum HistoryProtectionClass {
    ProductHead,
    ExplicitObligation,
}

/// One exact installed commit protection. It is move-only and releases its
/// direct protection at most once when the owner of the obligation drops it.
#[must_use = "an exact history protection must remain live while needed"]
pub(crate) struct CompositeHistoryProtectionObligation {
    reachability: HistoryReachabilityHandle,
    identity: crate::identity::CompositeCommitIdentity,
    class: HistoryProtectionClass,
}

impl std::fmt::Debug for CompositeHistoryProtectionObligation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CompositeHistoryProtectionObligation")
            .field("identity", &self.identity)
            .field("class", &self.class)
            .finish_non_exhaustive()
    }
}

impl Drop for CompositeHistoryProtectionObligation {
    fn drop(&mut self) {
        let mut reachability = lock_index(&self.reachability);
        reachability.decrement_direct_protection(&self.identity);
    }
}

impl CompositeHistoryProtectionObligation {
    pub(in crate::history) fn new(
        reachability: Arc<std::sync::Mutex<super::catalog::HistoryReachabilityIndex>>,
        identity: crate::identity::CompositeCommitIdentity,
        class: HistoryProtectionClass,
    ) -> Self {
        Self {
            reachability,
            identity,
            class,
        }
    }
}

/// History's future retention lane consumes an exact component dependency
/// class; it never becomes a second owner-lease authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HistoryRetentionContract {
    owner: RuntimeWorldOwnerIdentity,
    dependency: ComponentBasisDependencyClass,
}

impl HistoryRetentionContract {
    pub(crate) const fn new(
        owner: RuntimeWorldOwnerIdentity,
        dependency: ComponentBasisDependencyClass,
    ) -> Self {
        Self { owner, dependency }
    }

    pub(crate) const fn owner(self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    pub(crate) const fn dependency(self) -> ComponentBasisDependencyClass {
        self.dependency
    }
}
