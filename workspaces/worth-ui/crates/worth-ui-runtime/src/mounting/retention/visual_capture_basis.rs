pub(crate) struct UiMountedVisualCaptureBasis {
    lease: super::UiMountedVisualSnapshotLease,
    regions: super::super::UiMountedVisualRegionBasis,
    identity_trace: super::super::UiMountedIdentityTraceBasis,
}

impl UiMountedVisualCaptureBasis {
    pub(super) fn new(
        lease: super::UiMountedVisualSnapshotLease,
        regions: super::super::UiMountedVisualRegionBasis,
        identity_trace: super::super::UiMountedIdentityTraceBasis,
    ) -> Self {
        Self {
            lease,
            regions,
            identity_trace,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        super::UiMountedVisualSnapshotLease,
        super::super::UiMountedVisualRegionBasis,
        super::super::UiMountedIdentityTraceBasis,
    ) {
        (self.lease, self.regions, self.identity_trace)
    }
}
