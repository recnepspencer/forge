use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::identity::CompositeCommitIdentity;

use super::counters::{lock_counters, HistoryCatalogCountersHandle};
use super::denial::CompositeHistoryCatalogDenial;

/// The only reachability fact the history owner stores for an installed
/// occurrence. A dependency is acquired before a child reservation escapes;
/// direct protection is acquired by an exact RAII obligation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(in crate::history) struct HistoryReachabilityRecord {
    descendant_dependencies: usize,
    direct_protections: usize,
}

impl HistoryReachabilityRecord {
    pub(super) const fn descendant_dependencies(self) -> usize {
        self.descendant_dependencies
    }

    pub(super) const fn direct_protections(self) -> usize {
        self.direct_protections
    }
}

/// Catalog-owned exact reachability index. It has one row per installed
/// commit and never reconstructs ancestry during reclamation.
#[derive(Debug)]
pub(in crate::history) struct HistoryReachabilityIndex {
    records: BTreeMap<CompositeCommitIdentity, HistoryReachabilityRecord>,
    counters: HistoryCatalogCountersHandle,
}

pub(in crate::history) type HistoryReachabilityHandle = Arc<Mutex<HistoryReachabilityIndex>>;

pub(in crate::history) fn lock_index(
    index: &HistoryReachabilityHandle,
) -> MutexGuard<'_, HistoryReachabilityIndex> {
    index
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl HistoryReachabilityIndex {
    pub(super) fn new(counters: HistoryCatalogCountersHandle) -> Self {
        Self {
            records: BTreeMap::new(),
            counters,
        }
    }

    pub(super) fn install(&mut self, identity: CompositeCommitIdentity) {
        assert!(
            self.records
                .insert(identity, HistoryReachabilityRecord::default())
                .is_none(),
            "an installed history occurrence has one reachability row"
        );
        lock_counters(&self.counters).record_reachability_row_installed();
    }

    pub(super) fn lookup(
        &mut self,
        identity: &CompositeCommitIdentity,
    ) -> Option<HistoryReachabilityRecord> {
        lock_counters(&self.counters).record_reachability_lookup();
        self.records.get(identity).copied()
    }

    pub(super) fn increment_descendant_dependency(
        &mut self,
        parent: &CompositeCommitIdentity,
    ) -> Result<(), CompositeHistoryCatalogDenial> {
        let record = self
            .records
            .get_mut(parent)
            .expect("validated installed parent has a reachability row");
        record.descendant_dependencies =
            record
                .descendant_dependencies
                .checked_add(1)
                .ok_or_else(|| {
                    CompositeHistoryCatalogDenial::DependencyCountOverflow(parent.clone())
                })?;
        lock_counters(&self.counters).record_dependency_increment();
        Ok(())
    }

    pub(super) fn decrement_descendant_dependency(&mut self, parent: &CompositeCommitIdentity) {
        let record = self
            .records
            .get_mut(parent)
            .expect("a reclaimed child retains an installed parent row");
        record.descendant_dependencies = record
            .descendant_dependencies
            .checked_sub(1)
            .expect("each installed child owns one parent dependency");
        lock_counters(&self.counters).record_dependency_decrement();
    }

    pub(super) fn increment_direct_protection(
        &mut self,
        identity: &CompositeCommitIdentity,
    ) -> Result<(), CompositeHistoryCatalogDenial> {
        let record = self
            .records
            .get_mut(identity)
            .expect("validated installed protection target has a reachability row");
        record.direct_protections = record.direct_protections.checked_add(1).ok_or_else(|| {
            CompositeHistoryCatalogDenial::ProtectionCountOverflow(identity.clone())
        })?;
        lock_counters(&self.counters).record_direct_protection_acquisition();
        Ok(())
    }

    pub(in crate::history) fn decrement_direct_protection(
        &mut self,
        identity: &CompositeCommitIdentity,
    ) {
        let record = self
            .records
            .get_mut(identity)
            .expect("a live protection obligation retains its installed row");
        record.direct_protections = record
            .direct_protections
            .checked_sub(1)
            .expect("each protection obligation releases one direct protection");
        lock_counters(&self.counters).record_direct_protection_release();
    }

    pub(super) fn remove_installed(
        &mut self,
        identity: &CompositeCommitIdentity,
    ) -> HistoryReachabilityRecord {
        let record = self
            .records
            .remove(identity)
            .expect("reclaiming an installed commit has an index row");
        assert_eq!(record, HistoryReachabilityRecord::default());
        lock_counters(&self.counters).record_reachability_row_removed();
        record
    }
}
