mod seal;

pub(super) use seal::published_overlay_cost;
pub(crate) use seal::{
    map_overlay_retention_denial, seal_cleared_overlay, seal_overlay_target, seal_pending_overlay,
    seal_published_overlay,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiVisualOverlayIdentity(u64);

pub struct UiVisualOverlayTarget {
    selection: UiVisualOverlaySelection,
    lease: crate::mounting::UiMountedVisualOverlayLease,
}

pub struct UiPendingVisualOverlay {
    identity: UiVisualOverlayIdentity,
    selection: UiVisualOverlaySelection,
    registration: super::UiPendingVisualOverlayRegistration,
}

pub struct UiPublishedVisualOverlay {
    identity: UiVisualOverlayIdentity,
    selection: UiVisualOverlaySelection,
    published_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    cost: worth_ui_inspection::UiVisualInspectionCostReceipt,
}

pub struct UiVisualOverlayPublicationFailure {
    denial: worth_ui_inspection::UiVisualOverlayDenial,
    pending: Box<UiPendingVisualOverlay>,
}

pub struct UiVisualOverlayClearFailure {
    denial: worth_ui_inspection::UiVisualOverlayDenial,
    published: Box<UiPublishedVisualOverlay>,
}

pub(crate) struct UiPublishingVisualOverlay {
    pub(crate) identity: UiVisualOverlayIdentity,
    pub(crate) selection: UiVisualOverlaySelection,
}

pub(crate) struct UiClearingVisualOverlay {
    pub(crate) identity: UiVisualOverlayIdentity,
    pub(crate) selection: UiVisualOverlaySelection,
    pub(crate) published_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    pub(crate) published_cost: worth_ui_inspection::UiVisualInspectionCostReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiClearedVisualOverlayReceipt {
    identity: UiVisualOverlayIdentity,
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    base_snapshot: super::UiVisualSnapshotIdentity,
    base_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    published_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    cleared_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    cost: worth_ui_inspection::UiVisualInspectionCostReceipt,
}

#[derive(Clone)]
pub(crate) struct UiVisualOverlaySelection {
    pub(crate) session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    pub(crate) base_snapshot: super::UiVisualSnapshotIdentity,
    pub(crate) presentation: super::UiVisualSurfaceCaptureBasis,
    pub(crate) target_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    pub(crate) target_region: worth_ui_inspection::UiClientPhysicalRect,
    pub(crate) host_coordinate_transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pub(crate) trace: worth_ui_inspection::UiVisualIdentityTrace,
}

pub(crate) struct UiVisualOverlayTargetInput {
    pub(crate) session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    pub(crate) base_snapshot: super::UiVisualSnapshotIdentity,
    pub(crate) presentation: super::UiVisualSurfaceCaptureBasis,
    pub(crate) target_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    pub(crate) target_region: worth_ui_inspection::UiClientPhysicalRect,
    pub(crate) host_coordinate_transform: worth_ui_host_contract::UiHostCoordinateTransform,
    pub(crate) trace: worth_ui_inspection::UiVisualIdentityTrace,
    pub(crate) lease: crate::mounting::UiMountedVisualOverlayLease,
}

impl UiVisualOverlayTarget {
    pub(crate) const fn session(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.selection.session
    }

    pub const fn base_snapshot(&self) -> super::UiVisualSnapshotIdentity {
        self.selection.base_snapshot
    }

    pub const fn target(&self) -> &worth_ui_inspection::UiVisualIdentityTrace {
        &self.selection.trace
    }

    pub const fn base_frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.selection.presentation.frame
    }

    pub const fn semantic_surface(&self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.selection.presentation.semantic_surface
    }

    pub const fn binding(&self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        self.selection.presentation.binding
    }

    pub const fn target_region(&self) -> worth_ui_inspection::UiClientPhysicalRect {
        self.selection.target_region
    }

    pub(crate) fn relation(
        &self,
    ) -> Result<
        worth_ui_inspection::UiVisualSnapshotRelation,
        worth_ui_inspection::UiVisualOverlayDenial,
    > {
        self.lease.relation().map_err(map_overlay_retention_denial)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiVisualOverlaySelection,
        crate::mounting::UiMountedVisualOverlayLease,
    ) {
        (self.selection, self.lease)
    }
}

impl UiPendingVisualOverlay {
    pub(crate) const fn session(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.selection.session
    }

    pub const fn identity(&self) -> UiVisualOverlayIdentity {
        self.identity
    }

    pub const fn base_snapshot(&self) -> super::UiVisualSnapshotIdentity {
        self.selection.base_snapshot
    }

    pub const fn base_frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.selection.presentation.frame
    }

    pub const fn target_region(&self) -> worth_ui_inspection::UiClientPhysicalRect {
        self.selection.target_region
    }

    pub const fn target(&self) -> &worth_ui_inspection::UiVisualIdentityTrace {
        &self.selection.trace
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiVisualOverlayIdentity,
        UiVisualOverlaySelection,
        super::UiPendingVisualOverlayRegistration,
    ) {
        (self.identity, self.selection, self.registration)
    }
}

impl UiPublishedVisualOverlay {
    pub(crate) const fn session(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.selection.session
    }

    pub const fn identity(&self) -> UiVisualOverlayIdentity {
        self.identity
    }

    pub const fn base_snapshot(&self) -> super::UiVisualSnapshotIdentity {
        self.selection.base_snapshot
    }

    pub const fn base_frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.selection.presentation.frame
    }

    pub const fn target_region(&self) -> worth_ui_inspection::UiClientPhysicalRect {
        self.selection.target_region
    }

    pub const fn target(&self) -> &worth_ui_inspection::UiVisualIdentityTrace {
        &self.selection.trace
    }

    pub const fn published_frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.published_frame
    }

    pub const fn cost(&self) -> worth_ui_inspection::UiVisualInspectionCostReceipt {
        self.cost
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiVisualOverlayIdentity,
        UiVisualOverlaySelection,
        worth_ui_host_contract::UiMountedFrameIdentity,
        worth_ui_inspection::UiVisualInspectionCostReceipt,
    ) {
        (
            self.identity,
            self.selection,
            self.published_frame,
            self.cost,
        )
    }
}

impl UiVisualOverlayIdentity {
    pub const fn diagnostic_value(self) -> u64 {
        self.0
    }

    pub(crate) const fn issued_by_runtime(value: u64) -> Self {
        Self(value)
    }
}

impl UiClearedVisualOverlayReceipt {
    pub const fn identity(self) -> UiVisualOverlayIdentity {
        self.identity
    }

    pub const fn published_frame(self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.published_frame
    }

    pub const fn session(self) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub const fn base_snapshot(self) -> super::UiVisualSnapshotIdentity {
        self.base_snapshot
    }

    pub const fn base_frame(self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.base_frame
    }

    pub const fn cleared_frame(self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.cleared_frame
    }

    pub const fn cost(self) -> worth_ui_inspection::UiVisualInspectionCostReceipt {
        self.cost
    }
}

impl UiVisualOverlayPublicationFailure {
    pub(crate) fn new(
        denial: worth_ui_inspection::UiVisualOverlayDenial,
        pending: UiPendingVisualOverlay,
    ) -> Self {
        Self {
            denial,
            pending: Box::new(pending),
        }
    }

    pub const fn denial(&self) -> worth_ui_inspection::UiVisualOverlayDenial {
        self.denial
    }

    pub fn into_pending(self) -> UiPendingVisualOverlay {
        *self.pending
    }
}

impl UiVisualOverlayClearFailure {
    pub(crate) fn new(
        denial: worth_ui_inspection::UiVisualOverlayDenial,
        published: UiPublishedVisualOverlay,
    ) -> Self {
        Self {
            denial,
            published: Box::new(published),
        }
    }

    pub const fn denial(&self) -> worth_ui_inspection::UiVisualOverlayDenial {
        self.denial
    }

    pub fn into_published(self) -> UiPublishedVisualOverlay {
        *self.published
    }
}

impl std::fmt::Debug for UiVisualOverlayPublicationFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiVisualOverlayPublicationFailure")
            .field("denial", &self.denial)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Debug for UiVisualOverlayClearFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiVisualOverlayClearFailure")
            .field("denial", &self.denial)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Debug for UiPendingVisualOverlay {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiPendingVisualOverlay")
            .field("identity", &self.identity)
            .field("base_snapshot", &self.selection.base_snapshot)
            .finish_non_exhaustive()
    }
}

impl std::fmt::Debug for UiPublishedVisualOverlay {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiPublishedVisualOverlay")
            .field("identity", &self.identity)
            .field("base_snapshot", &self.selection.base_snapshot)
            .field("published_frame", &self.published_frame)
            .finish_non_exhaustive()
    }
}
