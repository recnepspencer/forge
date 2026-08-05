mod counters;
mod errors;
mod modes;
mod seam;
mod support;

pub use counters::PolicyAwareSeamCounters;
pub use errors::{PolicyAwareExecutionSeamError, PolicyAwareExecutionSeamFailureClass};
pub use modes::PolicyAwareExecutionMode;
pub use seam::{PolicyAwareExecutionSeam, PolicyAwareExecutionSeamIdentity};
pub use support::{
    deny_durable_policy_artifact_reload_claim, deny_durable_policy_cursor_claim,
    deny_durable_policy_delivery_metadata_reload_claim, deny_policy_cross_tenant_fanout_claim,
    deny_policy_per_row_allocation_claim, deny_saved_query_policy_bypass_claim,
    deny_unsupported_policy_workflow_composition_claim,
    runtime_backed_policy_execution_seam_handoff_report,
    runtime_backed_policy_execution_seam_support_profile, PolicyExecutionSeamHandoffReport,
    PolicyExecutionSeamSupportProfile, PolicyExecutionSeamSupportStatus,
    PolicyExecutionSeamSurface,
};

#[cfg(test)]
mod tests;
