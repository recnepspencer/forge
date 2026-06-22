use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopIslandPartition, PlanarBooleanLoopSourceProvenanceBundle,
    PlanarBooleanReconstructedLoopBoundary, PlanarBooleanSourceLoopSplitAttribution,
};

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanLoopRoleOutcomeBoundaryInput<'a> {
    reconstructed_loop_boundary: &'a PlanarBooleanReconstructedLoopBoundary,
    island_partition: &'a PlanarBooleanLoopIslandPartition,
    source_loop_split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
    source_provenance: &'a PlanarBooleanLoopSourceProvenanceBundle,
}

impl<'a> PlanarBooleanLoopRoleOutcomeBoundaryInput<'a> {
    pub fn from_reconstructed_loop_products_and_provenance(
        reconstructed_loop_boundary: &'a PlanarBooleanReconstructedLoopBoundary,
        island_partition: &'a PlanarBooleanLoopIslandPartition,
        source_loop_split_attribution: &'a PlanarBooleanSourceLoopSplitAttribution,
        source_provenance: &'a PlanarBooleanLoopSourceProvenanceBundle,
    ) -> Self {
        Self {
            reconstructed_loop_boundary,
            island_partition,
            source_loop_split_attribution,
            source_provenance,
        }
    }

    pub fn reconstructed_loop_boundary(self) -> &'a PlanarBooleanReconstructedLoopBoundary {
        self.reconstructed_loop_boundary
    }

    pub fn island_partition(self) -> &'a PlanarBooleanLoopIslandPartition {
        self.island_partition
    }

    pub fn source_loop_split_attribution(self) -> &'a PlanarBooleanSourceLoopSplitAttribution {
        self.source_loop_split_attribution
    }

    pub fn source_provenance(self) -> &'a PlanarBooleanLoopSourceProvenanceBundle {
        self.source_provenance
    }
}
