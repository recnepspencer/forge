use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanLoopReconstructionSplitConsumption;

pub struct PlanarBooleanLoopReconstructionRequestInput<'a> {
    split_consumption: &'a PlanarBooleanLoopReconstructionSplitConsumption,
}

impl<'a> PlanarBooleanLoopReconstructionRequestInput<'a> {
    pub fn from_split_consumption(
        split_consumption: &'a PlanarBooleanLoopReconstructionSplitConsumption,
    ) -> Self {
        Self { split_consumption }
    }

    pub(crate) fn split_consumption(&self) -> &'a PlanarBooleanLoopReconstructionSplitConsumption {
        self.split_consumption
    }
}
