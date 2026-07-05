use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapAdjacencyRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ValidatedArrangementBoundarySegment<'a> {
    pub(crate) source_loop_identity: &'a str,
    pub(crate) operand_side: PlanarBooleanCommonPlaneOperandSide,
    pub(crate) source_loop_winding_sign: i8,
    pub(crate) source_edge_identity: &'a str,
    pub(crate) fragment_identity: &'a str,
    pub(crate) boundary_role: PlanarBooleanOverlapChainBoundaryRole,
    pub(crate) ordinal: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ValidatedArrangementBoundaryComponent<'a> {
    pub(crate) source_loop_identities: Vec<&'a str>,
    pub(crate) segments: Vec<ValidatedArrangementBoundarySegment<'a>>,
    pub(crate) ordinal: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ValidatedArrangementCell<'a> {
    pub(crate) source_loop_identities: Vec<&'a str>,
    pub(crate) supporting_island_identity: Option<&'a str>,
    pub(crate) supporting_island_member_source_loop_identities: Vec<&'a str>,
    pub(crate) supporting_island_member_source_loop_operand_sides:
        Vec<PlanarBooleanCommonPlaneOperandSide>,
    pub(crate) supporting_island_member_source_loop_winding_signs: Vec<i8>,
    pub(crate) components: Vec<ValidatedArrangementBoundaryComponent<'a>>,
    pub(crate) ordinal: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ValidatedArrangementNeighborhood<'a> {
    row: &'a PlanarBooleanOverlapAdjacencyRow,
    segments: Vec<ValidatedArrangementBoundarySegment<'a>>,
    components: Vec<ValidatedArrangementBoundaryComponent<'a>>,
    cells: Vec<ValidatedArrangementCell<'a>>,
}

impl<'a> ValidatedArrangementNeighborhood<'a> {
    pub(crate) fn new(
        row: &'a PlanarBooleanOverlapAdjacencyRow,
        segments: Vec<ValidatedArrangementBoundarySegment<'a>>,
        components: Vec<ValidatedArrangementBoundaryComponent<'a>>,
        cells: Vec<ValidatedArrangementCell<'a>>,
    ) -> Self {
        Self {
            row,
            segments,
            components,
            cells,
        }
    }

    pub(crate) fn row(&self) -> &'a PlanarBooleanOverlapAdjacencyRow {
        self.row
    }

    pub(crate) fn segments(&self) -> &[ValidatedArrangementBoundarySegment<'a>] {
        &self.segments
    }

    pub(crate) fn components(&self) -> &[ValidatedArrangementBoundaryComponent<'a>] {
        &self.components
    }

    pub(crate) fn cells(&self) -> &[ValidatedArrangementCell<'a>] {
        &self.cells
    }
}

pub(crate) struct ValidatedOverlapArrangementLookup<'a> {
    ordered_neighborhoods: Vec<ValidatedArrangementNeighborhood<'a>>,
}

impl<'a> ValidatedOverlapArrangementLookup<'a> {
    pub(crate) fn new(ordered_neighborhoods: Vec<ValidatedArrangementNeighborhood<'a>>) -> Self {
        Self {
            ordered_neighborhoods,
        }
    }

    pub(crate) fn ordered_neighborhoods(&self) -> &[ValidatedArrangementNeighborhood<'a>] {
        &self.ordered_neighborhoods
    }
}
