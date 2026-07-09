mod deferred;
mod denial;
mod registry;
mod request;
mod snapshot;
#[cfg(test)]
mod tests;

pub use deferred::S8LayoutAdmissionDeferred;
pub use denial::S8LayoutAdmissionDenial;
pub use registry::{
    layout_admission_registry, S8LayoutAdmissionOutcome, S8LayoutRegistrySnapshotOutcome,
};
pub use request::{
    S8LayoutAdmissionRequest, S8LayoutRequestedCapability, S8LayoutStrategyCapability,
    S8RequestedKeyLawSet,
};
pub use snapshot::S8LayoutStrategyRegistrySnapshot;
