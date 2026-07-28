mod sealed {
    pub trait Target {
        fn into_capture_route(self) -> super::UiVisualTargetRoute;
    }
}

pub trait UiVisualTarget: sealed::Target + 'static {}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct UiVisualSurfaceCaptureBasis {
    pub(crate) frame: worth_ui_host_contract::UiMountedFrameIdentity,
    pub(crate) presentation_attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    pub(crate) semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    pub(crate) host_surface: worth_ui_host_contract::UiHostSurfaceIdentity,
    pub(crate) binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    pub(crate) epoch: worth_ui_host_contract::UiHostPresentationEpoch,
}

pub enum UiVisualTargetRoute {
    Host(UiVisualSurfaceCaptureBasis),
    DerivedRegion(UiDerivedRegionTargetSource),
}

pub struct UiDerivedRegionTargetSource {
    pub(crate) snapshot: super::UiRetainedVisualSnapshotSource,
    pub(crate) region: worth_ui_inspection::UiClientPhysicalRect,
}

pub struct UiCurrentPresentedSurfaceTarget {
    basis: UiVisualSurfaceCaptureBasis,
    _inspection_lease: crate::mounting::UiMountedRetentionLease,
}

pub struct UiRetainedPresentedSurfaceTarget {
    basis: UiVisualSurfaceCaptureBasis,
    _inspection_lease: crate::mounting::UiMountedRetentionLease,
}

pub struct UiMountedNodeVisualTarget {
    basis: UiVisualSurfaceCaptureBasis,
    receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    _inspection_lease: crate::mounting::UiMountedRetentionLease,
}

pub struct UiClientRegionVisualTarget {
    snapshot: super::UiRetainedVisualSnapshotSource,
    region: worth_ui_inspection::UiClientPhysicalRect,
}

macro_rules! host_target {
    ($target:ty) => {
        impl sealed::Target for $target {
            fn into_capture_route(self) -> UiVisualTargetRoute {
                UiVisualTargetRoute::Host(self.basis)
            }
        }

        impl UiVisualTarget for $target {}
    };
}

host_target!(UiCurrentPresentedSurfaceTarget);
host_target!(UiRetainedPresentedSurfaceTarget);
host_target!(UiMountedNodeVisualTarget);

impl sealed::Target for UiClientRegionVisualTarget {
    fn into_capture_route(self) -> UiVisualTargetRoute {
        UiVisualTargetRoute::DerivedRegion(UiDerivedRegionTargetSource {
            snapshot: self.snapshot,
            region: self.region,
        })
    }
}

impl UiVisualTarget for UiClientRegionVisualTarget {}

impl UiCurrentPresentedSurfaceTarget {
    pub const fn frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.basis.frame
    }

    pub const fn semantic_surface(&self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.basis.semantic_surface
    }

    pub const fn binding(&self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        self.basis.binding
    }
}

impl UiRetainedPresentedSurfaceTarget {
    pub const fn frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.basis.frame
    }

    pub const fn semantic_surface(&self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.basis.semantic_surface
    }

    pub const fn binding(&self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        self.basis.binding
    }
}

impl UiMountedNodeVisualTarget {
    pub const fn frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.basis.frame
    }

    pub const fn receipt(&self) -> worth_ui_host_contract::UiMountedNodeReceiptIdentity {
        self.receipt
    }
}

impl UiClientRegionVisualTarget {
    pub const fn snapshot(&self) -> super::UiVisualSnapshotIdentity {
        self.snapshot.identity
    }

    pub const fn region(&self) -> worth_ui_inspection::UiClientPhysicalRect {
        self.region
    }
}

pub(crate) fn into_capture_route<Target: UiVisualTarget>(target: Target) -> UiVisualTargetRoute {
    sealed::Target::into_capture_route(target)
}

pub(crate) fn seal_current_surface_target(
    basis: UiVisualSurfaceCaptureBasis,
    lease: crate::mounting::UiMountedRetentionLease,
) -> UiCurrentPresentedSurfaceTarget {
    UiCurrentPresentedSurfaceTarget {
        basis,
        _inspection_lease: lease,
    }
}

pub(crate) fn seal_retained_surface_target(
    basis: UiVisualSurfaceCaptureBasis,
    lease: crate::mounting::UiMountedRetentionLease,
) -> UiRetainedPresentedSurfaceTarget {
    UiRetainedPresentedSurfaceTarget {
        basis,
        _inspection_lease: lease,
    }
}

pub(crate) fn seal_mounted_node_target(
    basis: UiVisualSurfaceCaptureBasis,
    receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    lease: crate::mounting::UiMountedRetentionLease,
) -> UiMountedNodeVisualTarget {
    UiMountedNodeVisualTarget {
        basis,
        receipt,
        _inspection_lease: lease,
    }
}

pub(crate) fn seal_region_target(
    snapshot: super::UiRetainedVisualSnapshotSource,
    region: worth_ui_inspection::UiClientPhysicalRect,
) -> UiClientRegionVisualTarget {
    UiClientRegionVisualTarget { snapshot, region }
}
