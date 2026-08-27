//! One committed mounted allocation projection row.

use worth_ui_host_contract::{
    UiMountedAllocationBasis, UiMountedAllocationProjection, UiMountedCanonicalBox,
    UiMountedCanonicalBoxInput, UiMountedCoordinateSpace, UiMountedGeometryDenial,
    UiMountedOmissionReason, UiMountedTransformProjection,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct UiMountedAllocationProjectionRow {
    projection: Result<UiMountedAllocationProjection, UiMountedAllocationProjectionDenial>,
    viewport_bounds:
        Result<Option<UiCommittedViewportGeometry>, UiMountedAllocationProjectionDenial>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UiCommittedViewportGeometry(UiMountedCanonicalBox);

// Construction admits only canonical finite committed viewport evidence.
impl Eq for UiCommittedViewportGeometry {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedAllocationProjectionDenial {
    NonFiniteGeometry,
    NegativeExtent,
}

impl UiMountedAllocationProjectionRow {
    pub(super) fn from_receipt(receipt: &super::UiAllocationReceipt) -> Self {
        Self {
            projection: project_receipt(receipt),
            viewport_bounds: project_viewport_bounds(receipt),
        }
    }

    pub(super) fn projection(
        self,
    ) -> Result<UiMountedAllocationProjection, UiMountedAllocationProjectionDenial> {
        self.projection
    }

    pub(super) fn viewport_bounds(
        self,
    ) -> Result<Option<UiCommittedViewportGeometry>, UiMountedAllocationProjectionDenial> {
        self.viewport_bounds
    }
}

impl UiCommittedViewportGeometry {
    pub(crate) const fn mounted_box(self) -> UiMountedCanonicalBox {
        self.0
    }

    #[cfg(test)]
    pub(crate) const fn for_test(bounds: UiMountedCanonicalBox) -> Self {
        Self(bounds)
    }
}

fn project_viewport_bounds(
    receipt: &super::UiAllocationReceipt,
) -> Result<Option<UiCommittedViewportGeometry>, UiMountedAllocationProjectionDenial> {
    receipt
        .committed_allocation()
        .measurement_basis()
        .evidence_inputs()
        .iter()
        .find_map(|evidence| {
            let result = evidence.as_host_measurement_result()?;
            let crate::evidence::UiMeasurementValue::ViewportExtent(extent) = result.value() else {
                return None;
            };
            Some(
                UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
                    x: 0.0,
                    y: 0.0,
                    width: extent.width,
                    height: extent.height,
                    coordinate_space: coordinate_space(result.coordinate_space()),
                })
                .map(UiCommittedViewportGeometry),
            )
        })
        .transpose()
        .map_err(|denial| match denial {
            UiMountedGeometryDenial::NonFinite => {
                UiMountedAllocationProjectionDenial::NonFiniteGeometry
            }
            UiMountedGeometryDenial::NegativeExtent => {
                UiMountedAllocationProjectionDenial::NegativeExtent
            }
        })
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
