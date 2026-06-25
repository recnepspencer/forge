use super::super::slice_classification::WorthGraphReadAccessUnresolvedSliceRow;
use super::posture_projection::{
    WorthGraphReadAccessSpatialDensePostureOutcome,
    WorthGraphReadAccessSpatialDensePostureProjection,
};

pub(crate) fn admitted_posture_projection(
    row: &WorthGraphReadAccessUnresolvedSliceRow,
) -> WorthGraphReadAccessSpatialDensePostureProjection {
    WorthGraphReadAccessSpatialDensePostureProjection::new(
        row,
        WorthGraphReadAccessSpatialDensePostureOutcome::AdmittedPlanRequiresExecutionReceipt,
        None,
        None,
        None,
    )
}
