use worth_ui_host_contract::{UiMountedAllocationProjection, UiMountedOmissionReason};

use super::UiMountedProjectionDenial;

pub(super) fn lower_allocation(
    projection: Result<
        Option<UiMountedAllocationProjection>,
        crate::runtime::UiMountedAllocationProjectionDenial,
    >,
) -> Result<UiMountedAllocationProjection, UiMountedProjectionDenial> {
    projection
        .map_err(|denial| match denial {
            crate::runtime::UiMountedAllocationProjectionDenial::NonFiniteGeometry => {
                UiMountedProjectionDenial::NonFiniteGeometry
            }
            crate::runtime::UiMountedAllocationProjectionDenial::NegativeExtent => {
                UiMountedProjectionDenial::NegativeExtent
            }
        })
        .map(|projection| {
            projection.unwrap_or(UiMountedAllocationProjection::Omitted(
                UiMountedOmissionReason::NoCommittedAllocation,
            ))
        })
}
