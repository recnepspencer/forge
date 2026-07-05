use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanCoplanarOverlapArrangementGraph, PlanarBooleanOverlapCellContainmentMap,
    PlanarBooleanOverlapCellWindingField,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanOverlapIslandCandidateInput<'a> {
    arrangement_graph: &'a PlanarBooleanCoplanarOverlapArrangementGraph,
    containment_map: &'a PlanarBooleanOverlapCellContainmentMap,
    winding_field: &'a PlanarBooleanOverlapCellWindingField,
}

impl<'a> PlanarBooleanOverlapIslandCandidateInput<'a> {
    pub fn from_cell_classification(
        arrangement_graph: &'a PlanarBooleanCoplanarOverlapArrangementGraph,
        containment_map: &'a PlanarBooleanOverlapCellContainmentMap,
        winding_field: &'a PlanarBooleanOverlapCellWindingField,
    ) -> Self {
        Self {
            arrangement_graph,
            containment_map,
            winding_field,
        }
    }

    pub fn arrangement_graph(self) -> &'a PlanarBooleanCoplanarOverlapArrangementGraph {
        self.arrangement_graph
    }

    pub fn containment_map(self) -> &'a PlanarBooleanOverlapCellContainmentMap {
        self.containment_map
    }

    pub fn winding_field(self) -> &'a PlanarBooleanOverlapCellWindingField {
        self.winding_field
    }
}
