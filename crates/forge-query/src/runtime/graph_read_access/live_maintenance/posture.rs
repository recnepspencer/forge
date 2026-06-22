#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryLiveGraphReadAccessPosture {
    AdmittedLiveIncrementalMaintenance,
    AdmittedLiveSnapshotRefresh,
    LivePersistentIndexRequired,
    LiveAsyncMaterializationRequired,
    LiveStoreBackedCapabilityRequired,
    LiveAccessCapabilityRegistrationRequired,
    DeniedLiveMaintenanceBudget,
    DeniedLiveMaintenanceSupport,
}

impl ForgeQueryLiveGraphReadAccessPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdmittedLiveIncrementalMaintenance => "admitted-live-incremental-maintenance",
            Self::AdmittedLiveSnapshotRefresh => "admitted-live-snapshot-refresh",
            Self::LivePersistentIndexRequired => "live-persistent-index-required",
            Self::LiveAsyncMaterializationRequired => "live-async-materialization-required",
            Self::LiveStoreBackedCapabilityRequired => "live-store-backed-capability-required",
            Self::LiveAccessCapabilityRegistrationRequired => {
                "live-access-capability-registration-required"
            }
            Self::DeniedLiveMaintenanceBudget => "denied-live-maintenance-budget",
            Self::DeniedLiveMaintenanceSupport => "denied-live-maintenance-support",
        }
    }

    pub fn is_admitted(&self) -> bool {
        matches!(
            self,
            Self::AdmittedLiveIncrementalMaintenance | Self::AdmittedLiveSnapshotRefresh
        )
    }
}
