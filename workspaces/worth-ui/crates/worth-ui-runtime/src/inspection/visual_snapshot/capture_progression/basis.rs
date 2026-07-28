pub(crate) struct UiPinnedVisualCaptureBasis {
    pub(super) session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    pub(super) capture_identity: u64,
    pub(super) presentation: super::super::UiVisualSurfaceCaptureBasis,
    pub(super) snapshot_lease: crate::mounting::UiMountedVisualSnapshotLease,
    pub(super) visual_regions: crate::mounting::UiMountedVisualRegionBasis,
    pub(super) identity_trace_basis: crate::mounting::UiMountedIdentityTraceBasis,
    pub(super) registration: super::super::UiVisualCaptureRegistrationLease,
}

pub(crate) struct UiRequestedHostVisualCaptureBasis {
    pub(super) pinned: UiPinnedVisualCaptureBasis,
    pub(super) host_request: worth_ui_host_contract::UiHostVisualCaptureRequest,
}

pub(crate) struct UiObservedHostVisualCaptureBasis {
    pub(super) requested: UiRequestedHostVisualCaptureBasis,
    pub(super) observation: worth_ui_host_contract::UiHostCaptureObservation,
}

pub(crate) struct UiIndexedVisualCaptureBasis {
    pub(super) requested: UiRequestedHostVisualCaptureBasis,
    pub(super) _observation: UiHostObservationWitness,
    pub(super) validated: UiValidatedHostVisualCapture,
}

pub(crate) struct UiHostObservationWitness(());

pub(crate) struct UiValidatedHostVisualCapture {
    pub(crate) transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pub(crate) pixels: Option<worth_ui_inspection::UiVisualPixelArtifact>,
    pub(crate) visible_index: super::super::UiVisibleRegionIndex,
    pub(crate) hit_test_index: super::super::UiHitTestRegionIndex,
    pub(crate) spatial_cost: super::super::UiSpatialIndexBuildCost,
}

pub(crate) struct UiValidatedHostVisualCaptureInput {
    pub(crate) transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pub(crate) pixels: Option<worth_ui_inspection::UiVisualPixelArtifact>,
    pub(crate) visible_index: super::super::UiVisibleRegionIndex,
    pub(crate) hit_test_index: super::super::UiHitTestRegionIndex,
    pub(crate) spatial_cost: super::super::UiSpatialIndexBuildCost,
}

pub(crate) trait UiPinnedBasisAccess {
    fn pinned(&self) -> &UiPinnedVisualCaptureBasis;
}

impl UiValidatedHostVisualCapture {
    pub(crate) fn from_runtime_validation(input: UiValidatedHostVisualCaptureInput) -> Self {
        Self {
            transform: input.transform,
            pixels: input.pixels,
            visible_index: input.visible_index,
            hit_test_index: input.hit_test_index,
            spatial_cost: input.spatial_cost,
        }
    }
}

impl UiHostObservationWitness {
    pub(super) const fn issued_after_observation() -> Self {
        Self(())
    }
}

impl UiPinnedBasisAccess for UiPinnedVisualCaptureBasis {
    fn pinned(&self) -> &UiPinnedVisualCaptureBasis {
        self
    }
}

impl UiPinnedBasisAccess for UiRequestedHostVisualCaptureBasis {
    fn pinned(&self) -> &UiPinnedVisualCaptureBasis {
        &self.pinned
    }
}

impl UiPinnedBasisAccess for UiObservedHostVisualCaptureBasis {
    fn pinned(&self) -> &UiPinnedVisualCaptureBasis {
        &self.requested.pinned
    }
}
