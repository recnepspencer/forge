use crate::physical_runtime::durability::{
    join_dispatched_data, PhysicalDataEffectSettlement, PhysicalDataSettlementOutcome,
};
use crate::physical_runtime::{PhysicalMutationIdentity, WalDurablePhysicalMutation};

pub struct DataDispatchedPhysicalMutation {
    durable: WalDurablePhysicalMutation,
    effects: Vec<PhysicalDataEffectSettlement>,
}

impl DataDispatchedPhysicalMutation {
    pub(in crate::physical_runtime) fn new(
        durable: WalDurablePhysicalMutation,
        effects: Vec<PhysicalDataEffectSettlement>,
    ) -> Self {
        Self { durable, effects }
    }

    pub const fn mutation_identity(&self) -> PhysicalMutationIdentity {
        self.durable.mutation_identity()
    }

    pub const fn durable(&self) -> &WalDurablePhysicalMutation {
        &self.durable
    }

    pub fn effects(&self) -> &[PhysicalDataEffectSettlement] {
        &self.effects
    }

    pub fn settle_exact_effects(self) -> PhysicalDataSettlementOutcome {
        join_dispatched_data(self)
    }

    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (
        WalDurablePhysicalMutation,
        Vec<PhysicalDataEffectSettlement>,
    ) {
        (self.durable, self.effects)
    }
}
