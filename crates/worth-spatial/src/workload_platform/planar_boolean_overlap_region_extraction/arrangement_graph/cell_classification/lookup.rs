use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_overlap_region_extraction::arrangement_graph::{
    PlanarBooleanOverlapArrangementBoundarySegmentRow, PlanarBooleanOverlapArrangementCellRow,
};

pub(crate) struct ValidatedCellClassificationLookup<'a> {
    cells: Vec<ValidatedCellClassification<'a>>,
}

pub(crate) struct ValidatedCellClassification<'a> {
    pub cell: &'a PlanarBooleanOverlapArrangementCellRow,
    pub boundary_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
    pub left_boundary_winding_sum: i16,
    pub right_boundary_winding_sum: i16,
    pub left_supporting_winding_sum: i16,
    pub right_supporting_winding_sum: i16,
}

impl<'a> ValidatedCellClassificationLookup<'a> {
    pub(crate) fn new(cells: Vec<ValidatedCellClassification<'a>>) -> Self {
        Self { cells }
    }

    pub(crate) fn cells(&self) -> &[ValidatedCellClassification<'a>] {
        &self.cells
    }
}

pub(crate) fn boundary_segments_by_identity<'a>(
    boundary_segments: &'a [PlanarBooleanOverlapArrangementBoundarySegmentRow],
) -> BTreeMap<&'a str, &'a PlanarBooleanOverlapArrangementBoundarySegmentRow> {
    boundary_segments
        .iter()
        .map(
            |row: &'a PlanarBooleanOverlapArrangementBoundarySegmentRow| {
                (row.segment_identity(), row)
            },
        )
        .collect()
}
