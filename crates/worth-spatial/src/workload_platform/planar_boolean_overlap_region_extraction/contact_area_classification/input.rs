use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanAreaOverlapComponentSet, PlanarBooleanBoundaryContactComponentSet,
    PlanarBooleanOverlapIslandComponentBundle, PlanarBooleanOverlapIslandPartition,
    PlanarBooleanOverlapIslandSet,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanBoundaryContactClassificationInput<'a> {
    overlap_islands: &'a PlanarBooleanOverlapIslandSet,
    boundary_contact_components: &'a PlanarBooleanBoundaryContactComponentSet,
    area_overlap_components: &'a PlanarBooleanAreaOverlapComponentSet,
}

impl<'a> PlanarBooleanBoundaryContactClassificationInput<'a> {
    pub fn new(
        overlap_islands: &'a PlanarBooleanOverlapIslandSet,
        boundary_contact_components: &'a PlanarBooleanBoundaryContactComponentSet,
        area_overlap_components: &'a PlanarBooleanAreaOverlapComponentSet,
    ) -> Self {
        Self {
            overlap_islands,
            boundary_contact_components,
            area_overlap_components,
        }
    }

    pub fn from_island_partition(
        island_partition: &'a PlanarBooleanOverlapIslandPartition,
    ) -> Self {
        Self::new(
            island_partition.overlap_islands(),
            island_partition.boundary_contact_components(),
            island_partition.area_overlap_components(),
        )
    }

    pub fn from_island_component_bundle(
        island_component_bundle: &'a PlanarBooleanOverlapIslandComponentBundle,
    ) -> Self {
        Self::new(
            island_component_bundle.overlap_islands(),
            island_component_bundle.boundary_contact_components(),
            island_component_bundle.area_overlap_components(),
        )
    }

    pub fn overlap_islands(self) -> &'a PlanarBooleanOverlapIslandSet {
        self.overlap_islands
    }

    pub fn boundary_contact_components(self) -> &'a PlanarBooleanBoundaryContactComponentSet {
        self.boundary_contact_components
    }

    pub fn area_overlap_components(self) -> &'a PlanarBooleanAreaOverlapComponentSet {
        self.area_overlap_components
    }
}
