mod contract;
mod denial;
mod effect_contract;
mod intent;
mod resource_demand;
mod scope;

pub use contract::{
    PhysicalWorkDurabilityRequirement, PhysicalWorkEffectClass, PhysicalWorkOperationFamily,
    PhysicalWorkRecoveryDisposition,
};
pub use denial::PhysicalWorkDeclarationDenial;
pub use intent::PhysicalWorkIntent;
pub(in crate::physical_runtime) use intent::PhysicalWorkIntentParts;
pub(in crate::physical_runtime) use resource_demand::PhysicalWorkResourceDemand;
pub use scope::PhysicalWorkScope;

#[cfg(test)]
mod tests;
