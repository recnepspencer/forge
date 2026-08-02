use super::WalRangeReservedPhysicalMutation;
use crate::physical_runtime::{
    durability::CompletionBoundPhysicalWalAppendSettlement, PhysicalMutationIdentity,
    PhysicalWalAppendSettlement,
};

pub struct WalAppendedPhysicalMutation {
    reserved: WalRangeReservedPhysicalMutation,
    settlement: PhysicalWalAppendSettlement,
}

impl WalAppendedPhysicalMutation {
    pub(in crate::physical_runtime) fn new(
        reserved: WalRangeReservedPhysicalMutation,
        settlement: CompletionBoundPhysicalWalAppendSettlement,
    ) -> Self {
        Self {
            reserved,
            settlement: settlement.into_settlement(),
        }
    }

    pub const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        self.reserved.mutation_identity()
    }

    pub const fn settlement(&self) -> &PhysicalWalAppendSettlement {
        &self.settlement
    }

    pub const fn reserved(&self) -> &WalRangeReservedPhysicalMutation {
        &self.reserved
    }

    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (
        WalRangeReservedPhysicalMutation,
        PhysicalWalAppendSettlement,
    ) {
        (self.reserved, self.settlement)
    }
}
