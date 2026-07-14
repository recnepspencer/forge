mod admission_operation;
mod compatibility;
mod deferred;
mod denial;
mod request;
#[cfg(test)]
pub(crate) mod tests;

pub use admission_operation::{
    layout_admission_cases, layout_admission_registry, LayoutAdmissionCaseId,
    LayoutAdmissionOutcome, LayoutAdmissionView, LayoutStrategyRegistrySnapshot,
};
pub(crate) use compatibility::family_lane_supports_operation;
pub use deferred::LayoutAdmissionDeferred;
pub use denial::{LayoutAdmissionDenial, LayoutAdmissionDenialCase};
pub use request::{LayoutAdmissionRequest, LayoutRequestedCapability, LayoutStrategyCapability};
