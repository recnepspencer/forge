use super::product::PlanarBooleanOverlapCellContainmentMap;
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanCoplanarOverlapArrangementGraph;

pub struct PlanarBooleanOverlapCellContainmentInput<'a> {
    arrangement_graph: &'a PlanarBooleanCoplanarOverlapArrangementGraph,
}

pub struct PlanarBooleanOverlapCellWindingFieldInput<'a> {
    arrangement_graph: &'a PlanarBooleanCoplanarOverlapArrangementGraph,
    containment_map: &'a PlanarBooleanOverlapCellContainmentMap,
}

impl<'a> PlanarBooleanOverlapCellContainmentInput<'a> {
    pub fn from_arrangement(
        arrangement_graph: &'a PlanarBooleanCoplanarOverlapArrangementGraph,
    ) -> Self {
        Self { arrangement_graph }
    }

    pub fn arrangement_graph(&self) -> &'a PlanarBooleanCoplanarOverlapArrangementGraph {
        self.arrangement_graph
    }
}

impl<'a> PlanarBooleanOverlapCellWindingFieldInput<'a> {
    pub fn from_arrangement(
        arrangement_graph: &'a PlanarBooleanCoplanarOverlapArrangementGraph,
        containment_map: &'a PlanarBooleanOverlapCellContainmentMap,
    ) -> Self {
        Self {
            arrangement_graph,
            containment_map,
        }
    }

    pub fn arrangement_graph(&self) -> &'a PlanarBooleanCoplanarOverlapArrangementGraph {
        self.arrangement_graph
    }

    pub fn containment_map(&self) -> &'a PlanarBooleanOverlapCellContainmentMap {
        self.containment_map
    }
}
