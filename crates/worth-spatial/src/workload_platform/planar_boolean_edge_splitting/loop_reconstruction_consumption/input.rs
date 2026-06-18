use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanDownstreamSplitConsumption;

pub struct PlanarBooleanLoopReconstructionSplitConsumptionInput<'a> {
    downstream_consumption: &'a PlanarBooleanDownstreamSplitConsumption,
}

impl<'a> PlanarBooleanLoopReconstructionSplitConsumptionInput<'a> {
    pub fn from_downstream_split_consumption(
        downstream_consumption: &'a PlanarBooleanDownstreamSplitConsumption,
    ) -> Self {
        Self {
            downstream_consumption,
        }
    }

    pub(crate) fn downstream_consumption(&self) -> &'a PlanarBooleanDownstreamSplitConsumption {
        self.downstream_consumption
    }
}
