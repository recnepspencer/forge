mod contract;
mod denial;
mod effect_contract;
mod intent;
mod resource_demand;
mod scope;
mod wal_append_scope;
mod wal_barrier_scope;

pub use contract::{
    PhysicalWorkDurabilityRequirement, PhysicalWorkEffectClass, PhysicalWorkOperationFamily,
    PhysicalWorkRecoveryDisposition,
};
pub use denial::PhysicalWorkDeclarationDenial;
pub use intent::PhysicalWorkIntent;
pub(in crate::physical_runtime) use intent::PhysicalWorkIntentParts;
pub(in crate::physical_runtime) use resource_demand::PhysicalWorkResourceDemand;
pub use scope::PhysicalWorkScope;
pub use wal_append_scope::PhysicalWalAppendScope;
pub use wal_barrier_scope::PhysicalWalBarrierScope;

#[cfg(test)]
mod tests;
