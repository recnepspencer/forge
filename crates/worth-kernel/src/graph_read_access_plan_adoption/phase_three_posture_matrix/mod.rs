mod cap_ledger;
mod closeout;
mod counters;
mod errors;
mod phase_four_seed;
mod posture_resolution;

#[cfg(test)]
mod tests;

pub use cap_ledger::{
    WorthGraphReadAccessPostureCapLedger, WorthGraphReadAccessPostureCapReport,
    WorthGraphReadAccessPostureCapRow, WorthGraphReadAccessPostureFamilyCount,
};
pub use closeout::{
    current_worth_graph_read_access_posture_matrix_closeout,
    WorthGraphReadAccessPostureMatrixCloseout,
};
pub use counters::WorthGraphReadAccessPostureMatrixCounters;
pub use errors::{
    WorthGraphReadAccessPostureMatrixError, WorthGraphReadAccessPostureMatrixErrorKind,
};
pub use phase_four_seed::WorthGraphReadAccessPhaseFourSeed;
pub use posture_resolution::{
    WorthGraphReadAccessResolvedPosture, WorthGraphReadRequirementPostureMap,
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
