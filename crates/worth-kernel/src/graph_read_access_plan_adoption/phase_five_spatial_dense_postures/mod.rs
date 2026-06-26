mod batch_admission;
mod bounded_execution;
mod cap_ledger;
mod closeout;
mod closeout_digest;
mod counters;
mod errors;
mod phase_six_seed;
mod query_posture_projection;
mod slice_classification;
mod source_firewall;

#[cfg(test)]
mod tests;

pub use batch_admission::{
    WorthGraphReadAccessGroupedAdmissionMeasurementStatus,
    WorthGraphReadAccessGroupedAdmissionReport, WorthGraphReadAccessGroupedAdmissionRow,
};
pub use bounded_execution::{
    WorthGraphReadAccessBoundedExecutionContract,
    WorthGraphReadAccessBoundedExecutionContractStatus,
};
pub use closeout::{
    current_worth_graph_read_access_spatial_dense_posture_closeout,
    WorthGraphReadAccessSpatialDensePostureCloseout,
};
pub use counters::WorthGraphReadAccessSpatialDensePostureCounters;
pub use errors::{
    WorthGraphReadAccessSpatialDensePostureError, WorthGraphReadAccessSpatialDensePostureErrorKind,
};
pub use phase_six_seed::WorthGraphReadAccessSpatialDensePhaseSixSeed;
pub use query_posture_projection::{
    WorthGraphReadAccessSpatialDensePostureOutcome,
    WorthGraphReadAccessSpatialDensePostureProjection,
};
pub use slice_classification::{
    WorthGraphReadAccessUnresolvedSliceKind, WorthGraphReadAccessUnresolvedSliceRow,
};
pub use source_firewall::{
    reject_spatial_dense_local_graph_read_residue,
    WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    WorthGraphReadAccessSpatialDenseSourceFirewallViolation,
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
