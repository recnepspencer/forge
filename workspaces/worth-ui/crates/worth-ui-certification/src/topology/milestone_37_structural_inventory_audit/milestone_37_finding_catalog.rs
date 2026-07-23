use std::collections::BTreeSet;

use super::CleanupFailureMode;

pub fn milestone_37_cleared_finding_ids() -> BTreeSet<&'static str> {
    BTreeSet::from([
        "F-01", "F-02", "F-03", "F-04", "B-01", "B-02", "B-03", "H-01", "H-02", "O-01", "O-02",
        "O-03", "O-04", "S-01", "A-01", "A-02", "A-03", "A-04", "T-01", "T-02", "T-03", "T-04",
    ])
}

pub fn milestone_37_critical_finding_ids() -> BTreeSet<&'static str> {
    BTreeSet::new()
}

pub fn milestone_37_active_failure_modes() -> BTreeSet<CleanupFailureMode> {
    BTreeSet::new()
}

pub fn rejected_cosmetic_candidate_ids() -> BTreeSet<&'static str> {
    BTreeSet::from([
        "COSMETIC-01",
        "COSMETIC-02",
        "COSMETIC-03",
        "COSMETIC-04",
        "COSMETIC-05",
    ])
}
