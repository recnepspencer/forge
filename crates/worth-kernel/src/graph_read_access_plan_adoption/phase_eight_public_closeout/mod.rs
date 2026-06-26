mod closeout;
mod closeout_counters;
mod closeout_digest;
mod errors;
mod milestone_nine_seed;
mod proof_exports;

#[cfg(test)]
mod tests;

pub use closeout::{
    current_worth_graph_read_access_plan_adoption_closeout,
    WorthGraphReadAccessPlanAdoptionCloseout,
};
pub use closeout_counters::WorthGraphReadAccessPlanAdoptionCloseoutCounters;
pub use errors::{
    WorthGraphReadAccessPlanAdoptionCloseoutError,
    WorthGraphReadAccessPlanAdoptionCloseoutErrorKind,
};
pub use milestone_nine_seed::WorthGraphReadAccessPlanAdoptionMilestoneNineSeed;
pub use proof_exports::{
    WorthGraphReadAccessPlanAdoptionDeletionExport, WorthGraphReadAccessPlanAdoptionPostureExport,
    WorthGraphReadAccessPlanAdoptionReceiptExport, WorthGraphReadAccessPlanAdoptionResidueExport,
    WorthGraphReadAccessPlanAdoptionSourceFirewallExport,
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
