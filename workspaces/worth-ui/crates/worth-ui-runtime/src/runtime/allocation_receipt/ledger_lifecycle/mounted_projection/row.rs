//! One committed mounted allocation projection row.

use worth_ui_host_contract::{
    UiMountedAllocationBasis, UiMountedAllocationProjection, UiMountedCanonicalBox,
    UiMountedCanonicalBoxInput, UiMountedCoordinateSpace, UiMountedGeometryDenial,
    UiMountedOmissionReason, UiMountedTransformProjection,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct UiMountedAllocationProjectionRow {
    projection: Result<UiMountedAllocationProjection, UiMountedAllocationProjectionDenial>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedAllocationProjectionDenial {
    NonFiniteGeometry,
    NegativeExtent,
}

impl UiMountedAllocationProjectionRow {
    pub(super) fn from_receipt(receipt: &super::UiAllocationReceipt) -> Self {
        Self {
            projection: project_receipt(receipt),
        }
    }

    pub(super) fn projection(
        self,
    ) -> Result<UiMountedAllocationProjection, UiMountedAllocationProjectionDenial> {
        self.projection
    }
}

fn project_receipt(
    receipt: &super::UiAllocationReceipt,
) -> Result<UiMountedAllocationProjection, UiMountedAllocationProjectionDenial> {
    let geometry = receipt.geometry_evidence();
    let basis = allocation_basis(receipt);
    match geometry.bounds() {
        super::UiAllocationGeometryKnowledge::Known(bounds) => canonical_box(bounds)
            .map(|bounds| UiMountedAllocationProjection::Known { bounds, basis }),
        super::UiAllocationGeometryKnowledge::NotKnownAtAllocation => geometry
            .portal_anchor_observation()
            .map(|observation| {
                canonical_box(observation.observed_bounds()).map(|bounds| {
                    UiMountedAllocationProjection::PortalAnchorObservation { bounds, basis }
                })
            })
            .unwrap_or_else(|| {
                Ok(UiMountedAllocationProjection::Omitted(
                    UiMountedOmissionReason::AllocationBoundsUnknown,
                ))
            }),
    }
}

fn canonical_box(
    bounds: super::UiAllocationAxisAlignedBounds,
) -> Result<UiMountedCanonicalBox, UiMountedAllocationProjectionDenial> {
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x: bounds.x(),
        y: bounds.y(),
        width: bounds.width(),
        height: bounds.height(),
        coordinate_space: coordinate_space(bounds.coordinate_space()),
    })
    .map_err(|denial| match denial {
        UiMountedGeometryDenial::NonFinite => {
            UiMountedAllocationProjectionDenial::NonFiniteGeometry
        }
        UiMountedGeometryDenial::NegativeExtent => {
            UiMountedAllocationProjectionDenial::NegativeExtent
        }
    })
}

fn allocation_basis(receipt: &super::UiAllocationReceipt) -> UiMountedAllocationBasis {
    UiMountedAllocationBasis::new(
        receipt.identity().identity_digest(),
        receipt.generation().identity_digest(),
        receipt.identity().coordinate_ownership().identity_digest(),
        UiMountedTransformProjection::Omitted(UiMountedOmissionReason::NotDefinedByCurrentRuntime),
    )
}

fn coordinate_space(
    space: crate::evidence::UiMeasurementCoordinateSpace,
) -> UiMountedCoordinateSpace {
    match space {
        crate::evidence::UiMeasurementCoordinateSpace::Viewport => {
            UiMountedCoordinateSpace::Viewport
        }
        crate::evidence::UiMeasurementCoordinateSpace::Window => UiMountedCoordinateSpace::Window,
        crate::evidence::UiMeasurementCoordinateSpace::GraphNodeLocal => {
            UiMountedCoordinateSpace::GraphNodeLocal
        }
        crate::evidence::UiMeasurementCoordinateSpace::HostSurface => {
            UiMountedCoordinateSpace::HostSurface
        }
        crate::evidence::UiMeasurementCoordinateSpace::PortalLayer => {
            UiMountedCoordinateSpace::PortalLayer
        }
    }
}
