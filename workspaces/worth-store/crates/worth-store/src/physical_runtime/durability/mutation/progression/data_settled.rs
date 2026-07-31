use super::DataDispatchedPhysicalMutation;
use crate::physical_runtime::PhysicalMutationIdentity;

pub struct DataSettledPhysicalMutation {
    dispatched: DataDispatchedPhysicalMutation,
}

impl DataSettledPhysicalMutation {
    pub(in crate::physical_runtime) const fn new(
        dispatched: DataDispatchedPhysicalMutation,
    ) -> Self {
        Self { dispatched }
    }

    pub const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        self.dispatched.mutation_identity()
    }

    pub const fn dispatched(&self) -> &DataDispatchedPhysicalMutation {
        &self.dispatched
    }
}
