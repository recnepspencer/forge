mod deferred;
mod denial;
mod registry;
mod request;
mod snapshot;
#[cfg(test)]
pub(crate) mod tests;

pub use deferred::LayoutAdmissionDeferred;
pub use denial::{LayoutAdmissionDenial, LayoutAdmissionDenialCase};
pub(crate) use registry::family_lane_supports_operation;
pub use registry::layout_admission_registry;
pub use request::{LayoutAdmissionRequest, LayoutRequestedCapability, LayoutStrategyCapability};
pub use snapshot::LayoutStrategyRegistrySnapshot;
