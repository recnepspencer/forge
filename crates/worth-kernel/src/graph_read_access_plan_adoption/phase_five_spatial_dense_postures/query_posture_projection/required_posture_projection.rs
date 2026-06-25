use super::super::slice_classification::WorthGraphReadAccessUnresolvedSliceRow;
use super::posture_projection::{
    WorthGraphReadAccessSpatialDensePostureOutcome,
    WorthGraphReadAccessSpatialDensePostureProjection,
};

pub(crate) fn required_posture_projection(
    row: &WorthGraphReadAccessUnresolvedSliceRow,
    outcome: WorthGraphReadAccessSpatialDensePostureOutcome,
) -> WorthGraphReadAccessSpatialDensePostureProjection {
    WorthGraphReadAccessSpatialDensePostureProjection::new(row, outcome, None, None, None)
}
