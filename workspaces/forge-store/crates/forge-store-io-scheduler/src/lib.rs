#![forbid(unsafe_code)]

mod backend_capability;
pub mod background_pacing;
mod execution;
pub mod foreground_reservation;
mod interference_accounting;
pub mod queue_execution;
mod resource_envelope;
mod resource_units;
mod s6_later_readiness_handoff;
mod s6_readiness;
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
    BackgroundPacingAdmittedWithDebt, BackgroundPacingCounterSnapshot, BackgroundPacingDeferred,
    BackgroundPacingDenial, BackgroundPacingDenied, BackgroundPacingFreshness,
    BackgroundPacingOutcome, BackgroundPacingProgressionDrift, BackgroundPacingProgressionEvidence,
    BackgroundPacingStaleRebindKind, BackgroundPacingStaleRebindRequired, BackgroundPacingThrottle,
    BackgroundPacingViolation, BackgroundPacingYield, BackgroundResourceBudget,
    BackgroundResourceShortfall,
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
#[cfg(any(test, feature = "certification-test-authority"))]
pub use s6_later_readiness_handoff::background_pacing_outcome_for_later_readiness_certification_test;
#[cfg(any(test, feature = "certification-test-authority"))]
pub use s6_later_readiness_handoff::s7_placement_io_readiness_handoff_for_certification_test;
pub use s6_later_readiness_handoff::{
    admit_s11_operator_io_readiness_seed, publish_s10_backup_export_io_readiness_handoff,
    publish_s10_compaction_io_readiness_handoff, publish_s10_repair_scan_io_readiness_handoff,
    publish_s11_operator_io_readiness_handoff, publish_s7_placement_io_readiness_handoff,
    readmit_s10_backup_export_io_readiness_after_publication,
    readmit_s10_compaction_io_readiness_after_publication,
    readmit_s10_repair_scan_io_readiness_after_publication,
    readmit_s11_operator_io_readiness_after_publication,
    readmit_s7_placement_io_readiness_after_publication,
    reject_certification_only_evidence_as_later_readiness_handoff,
    reject_raw_s6_counters_as_later_readiness_handoff, S10BackupExportIoReadinessHandoff,
    S10BackupExportPacingEvidence, S10CompactionIoReadinessHandoff, S10CompactionPacingEvidence,
    S10RepairScanIoReadinessHandoff, S10RepairScanPacingEvidence, S11OperatorIoReadinessHandoff,
    S11OperatorIoReadinessSeed, S6LaterReadinessHandoffDenial, S6LaterReadinessReadmissionState,
    S7PlacementIoReadinessHandoff,
};
pub use s6_readiness::{
    admit_s5_1_security_scope_for_s6_io_qos, admit_s6_io_qos_isolation_readiness,
    admit_store_published_s6_io_qos_isolation_readiness,
    reject_hardware_queue_depth_claim_as_s6_readiness,
    reject_log_or_metric_projection_as_s6_readiness, reject_media_qos_claim_as_s6_readiness,
    IoSchedulerBackgroundMaintenanceAssumption, IoSchedulerForegroundInterferenceSurface,
    IoSchedulerPhysicalStabilityAssumption, IoSchedulerS6CounterSnapshot,
    IoSchedulerS6ReadinessAdmission, IoSchedulerS6ReadinessDenial, IoSchedulerS6ReadinessRequest,
    IoSchedulerS6SecurityScopeAdmission, IoSchedulerUnsupportedQosNonClaim,
    S6IoQosSecurityScopeHandoff, S6IoQosSecurityScopePermission,
};
pub use security_scope_io::{
    admit_secure_io_scope_for_scheduler, reject_lower_authority_secure_io_scope_source,
    SecureIoCounterStrength, SecureIoOperation, SecureIoPosture, SecureIoPostureRequirement,
    SecureIoPreservationCounterSnapshot, SecureIoPreservationDenial, SecureIoPreservationReceipt,
    SecureIoPreservationRequest, SecureIoScopeBasis,
};
