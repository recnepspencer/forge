mod access_posture;
mod closeout;
mod counters;
mod errors;
mod query_admission;
mod read_family_adoption;
mod source_firewall;

#[cfg(test)]
mod tests;

pub use access_posture::{
    WorthGraphReadAccessPlanAdoptionPostureKind, WorthGraphReadAccessPlanAdoptionPostureReport,
    WorthGraphReadAccessPlanAdoptionPostureRow, QUERY_ACCESS_POSTURE_MATRIX,
};
pub use closeout::{
    current_worth_graph_read_access_plan_adoption_phase_two_closeout,
    WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout,
};
pub use counters::WorthGraphReadAccessPlanAdoptionPhaseTwoCounters;
pub use errors::{
    WorthGraphReadAccessPlanAdoptionPhaseTwoError,
    WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind,
};
pub use query_admission::{
    WorthGraphReadAccessPlanAdoptionAttempt, WorthGraphReadAccessPlanAdoptionAttemptKind,
};
pub use read_family_adoption::{
    WorthGraphReadAccessPlanAdoptionCarriedGapRow, WorthGraphReadAccessPlanAdoptionLedger,
    WorthGraphReadAccessPlanAdoptionSeedPairing,
};
pub use source_firewall::{
    WorthGraphReadAccessPlanAdoptionSourceFirewallReport,
    WorthGraphReadAccessPlanAdoptionSourceFirewallViolation,
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
