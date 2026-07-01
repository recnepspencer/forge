mod authority;
mod bundle;
mod denial;
mod failure_digest;
mod foundational_materialization;

pub use authority::{EvidenceBundleAuthority, EvidenceBundleReadmissionAuthority};
pub use bundle::{PhysicalCertificationEvidenceBundle, PhysicalEvidenceBundlePrimary};
pub use denial::{
    reject_foundational_materialization_as_store_authority, reject_loose_log_evidence_attempt,
    reject_same_run_self_comparison_evidence_attempt, reject_terminal_json_evidence_attempt,
    PhysicalEvidenceBundleDenial, TerminalProjectionOnlyEvidenceDenied,
};
pub use failure_digest::SimulationFailureDigest;
pub use foundational_materialization::{
    readmit_foundational_physical_evidence_after_boundary,
    BoundaryBridgedPhysicalCertificationEvidenceBundle,
    FoundationalPhysicalCertificationEvidenceBundle, PhysicalEvidenceReportRow,
    ReadmittedPhysicalCertificationEvidenceBundle,
};
