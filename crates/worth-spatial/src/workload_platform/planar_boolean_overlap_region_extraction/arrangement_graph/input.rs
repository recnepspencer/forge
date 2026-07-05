use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapAdjacencyOrderingBasis, PlanarBooleanOverlapRegionAdjacencyIndex,
};

pub struct PlanarBooleanOverlapArrangementGraphInput<'a> {
    adjacency_index: &'a PlanarBooleanOverlapRegionAdjacencyIndex,
    ordering_basis: &'a PlanarBooleanOverlapAdjacencyOrderingBasis,
}

impl<'a> PlanarBooleanOverlapArrangementGraphInput<'a> {
    pub fn from_adjacency(
        adjacency_index: &'a PlanarBooleanOverlapRegionAdjacencyIndex,
        ordering_basis: &'a PlanarBooleanOverlapAdjacencyOrderingBasis,
    ) -> Self {
        Self {
            adjacency_index,
            ordering_basis,
        }
    }

    pub fn adjacency_index(&self) -> &'a PlanarBooleanOverlapRegionAdjacencyIndex {
        self.adjacency_index
    }

    pub fn ordering_basis(&self) -> &'a PlanarBooleanOverlapAdjacencyOrderingBasis {
        self.ordering_basis
    }
}
