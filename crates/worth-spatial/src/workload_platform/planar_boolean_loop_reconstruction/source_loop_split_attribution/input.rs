use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopIslandPartition;

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanSourceLoopSplitAttributionInput<'a> {
    island_partition: &'a PlanarBooleanLoopIslandPartition,
}

impl<'a> PlanarBooleanSourceLoopSplitAttributionInput<'a> {
    pub fn from_island_partition(island_partition: &'a PlanarBooleanLoopIslandPartition) -> Self {
        Self { island_partition }
    }

    pub fn island_partition(self) -> &'a PlanarBooleanLoopIslandPartition {
        self.island_partition
    }
}
