mod admitted_source_submission;
mod durable_resize;
mod host_measurement;
mod interaction;
mod outcome;
mod query_projection;
mod source_fact;
mod state;

pub(in crate::runtime::allocation_frame_dispatch) use admitted_source_submission::submit_admitted_source_fact;
pub(super) use admitted_source_submission::UiAllocationFrameAdmissionAttempt;

pub(crate) use durable_resize::WorthUiDurableResizeSubmission;
pub(crate) use host_measurement::WorthUiHostMeasurementSubmission;
pub(crate) use interaction::WorthUiInteractionSubmission;
pub use outcome::UiAllocationFrameGatewayOutcome;
pub(crate) use query_projection::WorthUiQueryProjectionSubmission;
pub use source_fact::{
    UiAllocationFrameQuerySettlementPosture, UiAllocationFrameQueryWarningPosture,
    UiAllocationFrameSourceFact, UiAllocationFrameSourceFactPosture,
};
pub(crate) use state::UiAllocationFrameGatewayState;

#[cfg(test)]
pub(crate) mod query_test_support;
#[cfg(test)]
mod tests;
