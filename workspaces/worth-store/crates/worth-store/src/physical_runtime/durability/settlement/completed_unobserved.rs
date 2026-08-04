use super::PhysicalMutationCompletedBreadth;
use crate::physical_runtime::PhysicalMutationIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletedUnobservedPhysicalMutation {
    mutation: PhysicalMutationIdentity,
    breadth: PhysicalMutationCompletedBreadth,
}

impl CompletedUnobservedPhysicalMutation {
    pub(in crate::physical_runtime) const fn new(
        mutation: PhysicalMutationIdentity,
        breadth: PhysicalMutationCompletedBreadth,
    ) -> Self {
        Self { mutation, breadth }
    }

    pub const fn mutation_identity(self) -> PhysicalMutationIdentity {
        self.mutation
    }

    pub const fn completed_breadth(self) -> PhysicalMutationCompletedBreadth {
        self.breadth
    }
}
