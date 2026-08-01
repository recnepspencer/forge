pub(crate) struct UiRetainedVisualSnapshotSource {
    pub(crate) session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    pub(crate) identity: super::UiVisualSnapshotIdentity,
    pub(crate) captured_client_extent: worth_ui_inspection::UiClientPhysicalRect,
    pub(crate) presentation: super::UiVisualSurfaceCaptureBasis,
    pub(crate) host_coordinate_transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pub(crate) pixel_artifact: Option<worth_ui_inspection::UiVisualPixelArtifact>,
    pub(crate) evidence: worth_ui_inspection::UiVisualSnapshotEvidence,
    pub(crate) visible_index: super::UiVisibleRegionIndex,
    pub(crate) hit_test_index: super::UiHitTestRegionIndex,
    pub(crate) identity_trace_basis: crate::mounting::UiMountedIdentityTraceBasis,
    pub(crate) snapshot_lease: crate::mounting::UiMountedVisualSnapshotLease,
    pub(crate) resource_lease: super::UiVisualSnapshotResourceLease,
}

impl UiRetainedVisualSnapshotSource {
    pub(crate) fn replace_registered_resource(
        mut self,
        identity: super::UiVisualSnapshotIdentity,
        usage: super::UiVisualRetainedResourceUsage,
    ) -> (Self, worth_ui_inspection::UiVisualPixelArtifactValidity) {
        self.resource_lease = self
            .resource_lease
            .replace(identity.diagnostic_value(), usage);
        let validity = self.resource_lease.pixel_validity();
        (self, validity)
    }
}
