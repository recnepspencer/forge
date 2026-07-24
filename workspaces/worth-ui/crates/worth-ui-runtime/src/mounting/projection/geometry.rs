use worth_ui_host_contract::{
    UiMountedAllocationBasis, UiMountedAllocationProjection, UiMountedCanonicalBox,
    UiMountedCanonicalBoxInput, UiMountedCoordinateSpace, UiMountedGeometryDenial,
    UiMountedOmissionReason, UiMountedTransformProjection,
};

use super::UiMountedProjectionDenial;

pub(super) fn lower_allocation(
    receipt: Option<&crate::runtime::UiAllocationReceipt>,
) -> Result<UiMountedAllocationProjection, UiMountedProjectionDenial> {
    let Some(receipt) = receipt else {
        return Ok(UiMountedAllocationProjection::Omitted(
            UiMountedOmissionReason::NoCommittedAllocation,
        ));
    };
    let geometry = receipt.geometry_evidence();
    let basis = allocation_basis(receipt);
    match geometry.bounds() {
        crate::runtime::UiAllocationGeometryKnowledge::Known(bounds) => canonical_box(bounds)
            .map(|bounds| UiMountedAllocationProjection::Known { bounds, basis }),
        crate::runtime::UiAllocationGeometryKnowledge::NotKnownAtAllocation => geometry
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
    bounds: crate::runtime::UiAllocationAxisAlignedBounds,
) -> Result<UiMountedCanonicalBox, UiMountedProjectionDenial> {
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x: bounds.x(),
        y: bounds.y(),
        width: bounds.width(),
        height: bounds.height(),
        coordinate_space: coordinate_space(bounds.coordinate_space()),
    })
    .map_err(|denial| match denial {
        UiMountedGeometryDenial::NonFinite => UiMountedProjectionDenial::NonFiniteGeometry,
        UiMountedGeometryDenial::NegativeExtent => UiMountedProjectionDenial::NegativeExtent,
    })
}

fn allocation_basis(receipt: &crate::runtime::UiAllocationReceipt) -> UiMountedAllocationBasis {
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
