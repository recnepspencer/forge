mod closeout;
mod counters;
mod cutover_proof;
mod errors;
mod execution_binding;
mod phase_five_seed;
mod query_plan_projection;
mod receipt_boundary;
mod slice_selection;
mod source_firewall;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use closeout::current_worth_graph_read_access_first_vertical_slice_closeout_with_construction_execution;
pub use closeout::{
    current_worth_graph_read_access_first_vertical_slice_closeout,
    WorthGraphReadAccessFirstVerticalSliceCloseout,
};
pub use counters::WorthGraphReadAccessFirstVerticalSliceCounters;
pub use cutover_proof::{
    WorthGraphReadAccessSliceCutoverProof, WorthGraphReadAccessSliceCutoverStatus,
};
pub use errors::{
    WorthGraphReadAccessFirstVerticalSliceError, WorthGraphReadAccessFirstVerticalSliceErrorKind,
};
pub use phase_five_seed::WorthGraphReadAccessFirstVerticalSliceSeed;
pub use query_plan_projection::{
    WorthGraphReadAccessSlicePlanProjection, WorthGraphReadAccessSlicePlanProjectionStatus,
};
pub use receipt_boundary::{
    WorthGraphReadAccessSliceReceiptProjection, WorthGraphReadAccessSliceReceiptStatus,
};
pub use slice_selection::{
    WorthGraphReadAccessSelectedVerticalSlice, WorthGraphReadAccessSliceSelectionReason,
};
pub use source_firewall::{
    reject_post_admission_local_graph_read_residue,
    WorthGraphReadAccessPostAdmissionSourceFirewallReport,
    WorthGraphReadAccessPostAdmissionSourceFirewallViolation,
};

pub(crate) fn stable_digest(parts: &[String]) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update(b"\0");
    }
    format!("{:x}", hasher.finalize())
}
