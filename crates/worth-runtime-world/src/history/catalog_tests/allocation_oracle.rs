use std::mem::size_of;
use std::sync::Arc;

use crate::identity::CompositeCommitIdentity;

use super::super::super::CompositeCommitParent;
use super::super::reachability::HistoryReachabilityRecord;
use super::super::{
    CompositeHistoryCatalogEntry, CompositeRuntimeWorldCommit, HistoryReservationMetadata,
};

/// Test-local logical allocation model. It intentionally repeats the named
/// layout charges instead of calling any production metadata function.
pub(super) struct AllocationOracle;

impl AllocationOracle {
    pub(super) fn installed_resident(_commit: &CompositeRuntimeWorldCommit) -> usize {
        sum([
            size_of::<CompositeRuntimeWorldCommit>(),
            size_of::<Arc<CompositeRuntimeWorldCommit>>(),
            size_of::<CompositeCommitIdentity>(),
            size_of::<CompositeHistoryCatalogEntry>(),
            size_of::<CompositeCommitIdentity>(),
            size_of::<HistoryReachabilityRecord>(),
            size_of::<Box<CompositeHistoryCatalogEntry>>(),
            size_of::<Box<HistoryReachabilityRecord>>(),
        ])
    }

    pub(super) fn reservation_resident(commit: &CompositeRuntimeWorldCommit) -> usize {
        let held_parent_identity = match commit.parent() {
            CompositeCommitParent::Root => 0,
            CompositeCommitParent::Ordinary(_) => size_of::<CompositeCommitIdentity>(),
        };
        sum([
            size_of::<CompositeCommitIdentity>(),
            size_of::<HistoryReservationMetadata>(),
            size_of::<CompositeCommitIdentity>(),
            held_parent_identity,
            size_of::<Box<HistoryReservationMetadata>>(),
        ])
    }

    pub(super) fn reservation_plus_installation(commit: &CompositeRuntimeWorldCommit) -> usize {
        sum([
            Self::reservation_resident(commit),
            Self::installed_resident(commit),
        ])
    }
}

fn sum<const N: usize>(parts: [usize; N]) -> usize {
    parts
        .into_iter()
        .try_fold(0usize, |total, part| total.checked_add(part))
        .expect("the independent test allocation model fits usize")
}
