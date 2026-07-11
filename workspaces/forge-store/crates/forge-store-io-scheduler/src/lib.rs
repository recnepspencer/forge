#![forbid(unsafe_code)]

mod backend_capability;
mod admission;
pub mod background_pacing;
mod execution;
pub mod foreground_reservation;
mod interference_accounting;
#[path = "layout_access/foreground_interference_family.rs"]
mod foreground_interference_family;
#[path = "layout_access/pacing_family.rs"]
mod pacing_family;
pub mod queue_execution;
mod resource_envelope;
mod resource_units;
#[path = "layout_access/scheduler_reservation_family.rs"]
mod scheduler_reservation_family;
mod s8_runtime_receipt;
mod security_scope_io;

#[cfg(test)]
mod execution_tests;

pub use backend_capability::{
    admit_backend_capability_for_scheduler_claim,
    admit_secure_frame_backend_capability_for_scheduler_claim,
    IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityDenial,
    IoSchedulerBackendCapabilityRequirement,
};
pub use background_pacing::{
    admit_background_capacity, admit_background_pacing, BackgroundCapacityAdmission,
    BackgroundCapacityAdmissionRequest, BackgroundDebtKind, BackgroundIdleCapacityLease,
    BackgroundIdleCapacityLeaseRequest, BackgroundIoDebt, BackgroundIoPressureClass,
    BackgroundIoPressureShape, BackgroundLeaseRevocation, BackgroundPacingAdmissionBasis,
    BackgroundPacingAdmittedWithDebt, BackgroundPacingCapability, BackgroundPacingCounterSnapshot,
    BackgroundPacingDeferred, BackgroundPacingDenial, BackgroundPacingDenied,
    BackgroundPacingFreshness, BackgroundPacingOutcome, BackgroundPacingProgressionDrift,
    BackgroundPacingProgressionEvidence, BackgroundPacingStaleRebindKind,
    BackgroundPacingStaleRebindRequired, BackgroundPacingThrottle, BackgroundPacingViolation,
    BackgroundPacingYield, BackgroundResourceBudget, BackgroundResourceShortfall,
};
#[cfg(any(test, feature = "certification-test-authority"))]
pub use background_pacing::{
    blob_ingest_background_capacity_for_certification_test,
    blob_ingest_deferred_background_capacity_for_certification_test,
    blob_ingest_denied_background_capacity_for_certification_test,
    blob_ingest_page_write_background_capacity_for_certification_test,
    blob_ingest_rebind_background_capacity_for_certification_test,
    blob_ingest_stale_background_capacity_for_certification_test,
    blob_ingest_throttled_background_capacity_for_certification_test,
    blob_ingest_wal_write_background_capacity_for_certification_test,
    checkpoint_flush_wal_background_capacity_for_certification_test,
    verification_deferred_background_capacity_for_certification_test,
    verification_denied_background_capacity_for_certification_test,
    verification_rebind_background_capacity_for_certification_test,
    verification_stale_background_capacity_for_certification_test,
    verification_throttled_background_capacity_for_certification_test,
    verification_zero_admitted_throttle_background_capacity_for_certification_test,
};
pub use execution::{
    IoQueueCounterSnapshot, IoQueueExecutedEvidenceSource, IoQueueExecutionDenial,
    IoQueueExecutionRecorder,
};
pub use interference_accounting::{
    assess_queue_latency_envelope, BackgroundInterferenceEvidence, InterferenceAttribution,
    InterferenceCounterClaim, InterferenceCounterDenial, InterferenceCounterName,
    InterferenceCounterRequirement, InterferenceCounterRow, InterferenceReplayScope,
    LatencyEnvelopeAssessment, LatencyEnvelopeAssessmentStatus, LatencyEnvelopeClaim,
};
pub use foreground_interference_family::{
    ForegroundInterferenceAccessBudget, ForegroundInterferenceLayoutReport,
    ForegroundInterferencePosture,
};
pub use pacing_family::{BackgroundPacingInterferencePosture, BackgroundPacingLayoutReport};
pub use scheduler_reservation_family::{
    SchedulerReservationInterferencePosture, SchedulerReservationLayoutReport,
};
pub use queue_execution::{
    admit_queue_execution_plan, execute_grouped_ready_queue_plans, execute_ready_queue_plan,
    group_ready_queue_pair, lower_background_queue_lease, lower_buffer_pool_queue_declaration,
    lower_wal_queue_declaration, queue_execution_lowering_authority, AdmittedQueueExecutionPlan,
    ExecutedQueueEvidence, QueueBackpressureCause, QueueExecutedPlan,
    QueueExecutionAdmissionDenial, QueueExecutionAdmissionRequest, QueueExecutionBackpressured,
    QueueExecutionCounterSnapshot, QueueExecutionDenied, QueueExecutionLoweringAuthority,
    QueueExecutionOutcome, QueueExecutionPlanBinding, QueueExecutionProgression,
    QueueExecutionReadyPlan, QueueExecutionReplayIdentity, QueueExecutionViolation,
    QueueGroupedReadyPlans, QueueGroupingBasis, QueueGroupingDenial, QueueGroupingOutcome,
    QueueGroupingRejected, QueueReadAheadBasis, QueueRecoveryOrdering, QueueWorkClass,
    QueueWorkDeclaration, QueueWriteBackBasis, QueueWritebackPolicy, S6QueueDurabilityClass,
};
pub use resource_envelope::{IoQueueResourceEnvelope, IoQueueResourceEnvelopeDenial};
pub use resource_units::{
    BandwidthToken, CacheResidencyHint, DirtyPageBudget, FlushPermit, IoResourceUnitDenial,
    IoResourceUnitKind, QueueSlot, ReadAheadWindow, ReclaimPermit, SyncDebt, WorkerPermit,
    WriteBackWindow,
};
pub use admission::{
    admit_security_scope_for_scheduler, admit_store_published_isolation_capability,
    IoSchedulerBackgroundMaintenanceAssumption, IoSchedulerForegroundInterferenceSurface,
    IoSchedulerIsolationAdmission, IoSchedulerIsolationAdmissionDenial,
    IoSchedulerIsolationCounterSnapshot, IoSchedulerPhysicalStabilityAssumption,
    IoSchedulerSecurityScopeAdmission, IoSchedulerSecurityScopeAdmissionDenial,
    SchedulerSecurityScopeCapability,
};
#[cfg(feature = "certification-test-authority")]
pub use s8_runtime_receipt::s8_maintenance_io_runtime_receipt_for_certification_test;
pub use s8_runtime_receipt::S8MaintenanceIoRuntimeReceipt;
pub use security_scope_io::{
    admit_secure_io_scope_for_scheduler, reject_lower_authority_secure_io_scope_source,
    SecureIoCounterStrength, SecureIoOperation, SecureIoPosture, SecureIoPostureRequirement,
    SecureIoPreservationCounterSnapshot, SecureIoPreservationDenial, SecureIoPreservationReceipt,
    SecureIoPreservationRequest, SecureIoScopeBasis,
};
