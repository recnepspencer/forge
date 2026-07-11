mod admission;
mod denial;
mod evidence;
mod foundational_lowering;
mod identity;
mod isolation_denial;
mod proof_progression;
mod request;
mod scheduler_capability;
mod shortcut_denials;
pub mod interference {
    mod assumptions;
    pub use assumptions::{
        BackgroundMaintenanceIsolationAssumption, ForegroundInterferenceSurface,
        PhysicalStabilityAssumption,
    };
}
pub(crate) mod isolation_evidence {
    pub(crate) mod basis;
}
pub(crate) use isolation_evidence::basis::{
    ExecutedIsolationBasis, IsolationEvidenceProjection, SchedulerIsolationCapabilityBasis,
    SchedulerIsolationProof,
};

pub use admission::{
    admit_physical_isolation_entry, admit_physical_isolation_entry_checked,
    PhysicalIsolationEntryAdmission, PhysicalIsolationEntryCheckedOutcome,
};
pub use denial::{PhysicalIsolationEntryDenial, PhysicalIsolationEntryRebindRequired};
pub use evidence::PhysicalIsolationEntryEvidence;
pub use foundational_lowering::PhysicalIsolationEntryFoundationalEvidence;
pub use identity::{PhysicalIsolationEntryIdentity, PhysicalIsolationRootEpochBasis};
pub use isolation_denial::{
    reject_copied_closeout_report_as_isolation_readiness,
    reject_log_or_terminal_projection_as_isolation_readiness,
    reject_missing_latch_counters_as_isolation_readiness,
    reject_missing_protected_byte_footprint_as_isolation_readiness,
    reject_missing_reclaim_counters_as_isolation_readiness,
    reject_synthetic_wait_label_as_isolation_readiness,
    reject_unsupported_qos_claim_as_isolation_readiness, IsolationReadinessDenial,
};
pub use proof_progression::{
    PhysicalIsolationAdmittedEntryRecipe, PhysicalIsolationEntryProofProgression,
    PhysicalIsolationEntryProofRequest, PhysicalIsolationLoweredEntryRecipe,
    PhysicalIsolationResolvedEntryRecipe, S4RecoveryReadinessBasis,
};
pub use request::PhysicalIsolationEntryRequest;
#[cfg(any(test, feature = "certification-authority"))]
pub use scheduler_capability::publish_scheduler_isolation_capability_for_certification_test;
pub use scheduler_capability::{
    publish_scheduler_isolation_capability_from_executed_evidence, SchedulerIsolationCapability,
    UnsupportedQoSClaim,
};
pub use shortcut_denials::{
    reject_copied_recovery_fields_as_physical_isolation_entry,
    reject_foundational_or_proof_projection_as_physical_isolation_entry,
    reject_json_authority_as_physical_isolation_entry,
    reject_live_runtime_state_as_physical_isolation_entry,
    reject_semantic_snapshot_as_physical_isolation_entry,
    reject_stale_recovery_readiness_as_physical_isolation_entry,
    reject_terminal_projection_as_physical_isolation_entry,
    require_rebound_s4_recovery_readiness_for_physical_isolation_entry,
};
