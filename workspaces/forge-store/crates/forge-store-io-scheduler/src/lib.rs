#![forbid(unsafe_code)]

mod execution;
mod resource_envelope;
mod s6_readiness;

#[cfg(test)]
mod execution_tests;

pub use execution::{
    IoQueueCounterSnapshot, IoQueueExecutedEvidenceSource, IoQueueExecutionDenial,
    IoQueueExecutionRecorder,
};
pub use resource_envelope::{IoQueueResourceEnvelope, IoQueueResourceEnvelopeDenial};
pub use s6_readiness::{
    admit_s6_io_qos_isolation_readiness, admit_store_published_s6_io_qos_isolation_readiness,
    reject_hardware_queue_depth_claim_as_s6_readiness,
    reject_log_or_metric_projection_as_s6_readiness, reject_media_qos_claim_as_s6_readiness,
    IoSchedulerBackgroundMaintenanceAssumption, IoSchedulerForegroundInterferenceSurface,
    IoSchedulerPhysicalStabilityAssumption, IoSchedulerS6CounterSnapshot,
    IoSchedulerS6ReadinessAdmission, IoSchedulerS6ReadinessDenial, IoSchedulerS6ReadinessRequest,
    IoSchedulerUnsupportedQosNonClaim,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoWorkClass {
    Foreground,
    Checkpoint,
    Compaction,
    Scrub,
    BlobMigration,
}
