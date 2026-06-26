use super::super::slice_classification::WorthGraphReadAccessUnresolvedSliceRow;
use super::posture_projection::{
    WorthGraphReadAccessSpatialDensePostureOutcome,
    WorthGraphReadAccessSpatialDensePostureProjection,
};

pub(crate) fn denied_posture_projection(
    row: &WorthGraphReadAccessUnresolvedSliceRow,
) -> WorthGraphReadAccessSpatialDensePostureProjection {
    WorthGraphReadAccessSpatialDensePostureProjection::new(
        row,
        WorthGraphReadAccessSpatialDensePostureOutcome::DeniedByQueryPosture,
        None,
        None,
        None,
    )
}
