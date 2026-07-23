mod admitted_source_submission;
mod durable_resize;
mod host_measurement;
mod interaction;
mod outcome;
mod query_settled_fact;
mod source_fact;
mod state;

pub(super) use admitted_source_submission::UiAllocationFrameAdmissionAttempt;
pub(in crate::runtime::allocation_frame_dispatch) use admitted_source_submission::{
    submit_admitted_source_fact, UiAllocationFrameSourceSubmission,
};

pub(crate) use durable_resize::WorthUiDurableResizeSubmission;
pub(crate) use host_measurement::WorthUiHostMeasurementSubmission;
pub(crate) use interaction::WorthUiInteractionSubmission;
pub use outcome::UiAllocationFrameGatewayOutcome;
pub(crate) use query_settled_fact::WorthUiQuerySettledFactSubmission;
pub use query_settled_fact::{
    WorthUiQueryFrameIngressCounters, WorthUiQueryFrameIngressDenial,
    WorthUiQueryFrameIngressOutcome,
};
pub use source_fact::{
    UiAllocationFrameQueryWarningPosture, UiAllocationFrameSourceFact,
    UiAllocationFrameSourceFactPosture,
};
pub(crate) use state::UiAllocationFrameGatewayState;

#[cfg(test)]
mod tests;
