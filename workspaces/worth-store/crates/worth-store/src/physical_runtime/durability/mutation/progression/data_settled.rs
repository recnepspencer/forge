use super::DataDispatchedPhysicalMutation;
use crate::physical_runtime::{
    durability::CompletionBoundPhysicalDataSettlement, PhysicalMutationIdentity,
};

pub struct DataSettledPhysicalMutation {
    dispatched: DataDispatchedPhysicalMutation,
}

impl DataSettledPhysicalMutation {
    pub(in crate::physical_runtime) fn new(
        settlement: CompletionBoundPhysicalDataSettlement,
    ) -> Self {
        Self {
            dispatched: settlement.into_dispatched(),
        }
    }

    pub const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        self.dispatched.mutation_identity()
    }

    pub const fn dispatched(&self) -> &DataDispatchedPhysicalMutation {
        &self.dispatched
    }
}
