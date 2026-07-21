use crate::compatibility::managed_live::WorthUiManagedLiveCompatibilityObservation;

/// Read-only structural observation of one runtime Query binding owner.
///
/// Counts are derived from retained production state. The observation carries
/// no installed reference, settlement, consumer contract, or live resource and
/// therefore cannot be promoted back into lifecycle authority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiRuntimeQueryStateObservation {
    query_installed: bool,
    installed_reference_count: usize,
    stale_installed_reference_count: usize,
    settled_snapshot_count: usize,
    orphan_settled_snapshot_count: usize,
    managed_live: WorthUiManagedLiveCompatibilityObservation,
}

/// Non-authoritative classification of an installed-reference membership check.
///
/// This result can diagnose ownership but cannot recover or mint Query authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryReferenceMembershipObservation {
    QueryFree,
    ExactInstalledReference,
    ForeignInstalledReference,
}

impl WorthUiRuntimeQueryStateObservation {
    pub(crate) fn installed(
        installed_reference_count: usize,
        stale_installed_reference_count: usize,
        settled_snapshot_count: usize,
        orphan_settled_snapshot_count: usize,
        managed_live: WorthUiManagedLiveCompatibilityObservation,
    ) -> Self {
        Self {
            query_installed: true,
            installed_reference_count,
            stale_installed_reference_count,
            settled_snapshot_count,
            orphan_settled_snapshot_count,
            managed_live,
        }
    }

    pub fn query_installed(self) -> bool {
        self.query_installed
    }

    pub fn installed_reference_count(self) -> usize {
        self.installed_reference_count
    }

    pub fn stale_installed_reference_count(self) -> usize {
        self.stale_installed_reference_count
    }

    pub fn settled_snapshot_count(self) -> usize {
        self.settled_snapshot_count
    }

    pub fn orphan_settled_snapshot_count(self) -> usize {
        self.orphan_settled_snapshot_count
    }

    pub fn managed_live(self) -> WorthUiManagedLiveCompatibilityObservation {
        self.managed_live
    }
}
