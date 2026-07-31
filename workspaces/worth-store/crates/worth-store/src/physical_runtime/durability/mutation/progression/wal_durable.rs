use super::WalAppendedPhysicalMutation;
use crate::physical_runtime::{
    durability::CompletionBoundPhysicalWalBarrierSettlement, PhysicalMutationIdentity,
    PhysicalWalBarrierSettlement,
};

pub struct WalDurablePhysicalMutation {
    appended: WalAppendedPhysicalMutation,
    settlement: PhysicalWalBarrierSettlement,
}

impl WalDurablePhysicalMutation {
    pub(in crate::physical_runtime) const fn new(
        appended: WalAppendedPhysicalMutation,
        settlement: CompletionBoundPhysicalWalBarrierSettlement,
    ) -> Self {
        Self {
            appended,
            settlement: settlement.settlement(),
        }
    }

    pub const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        self.appended.mutation_identity()
    }

    pub const fn member_basis(&self) -> crate::physical_runtime::PhysicalWalMemberBasis {
        self.appended.reserved().member_basis()
    }

    pub const fn barrier_settlement(&self) -> PhysicalWalBarrierSettlement {
        self.settlement
    }

    pub const fn appended(&self) -> &WalAppendedPhysicalMutation {
        &self.appended
    }

    pub(in crate::physical_runtime) fn data_frames(
        &self,
    ) -> &[crate::physical_runtime::durability::WalBoundPhysicalDataFrame] {
        self.appended.reserved().data().frames()
    }
}
