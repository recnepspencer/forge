#![forbid(unsafe_code)]

mod execution;
mod resource_envelope;

#[cfg(test)]
mod execution_tests;

pub use execution::{
    IoQueueCounterSnapshot, IoQueueExecutedEvidenceSource, IoQueueExecutionDenial,
    IoQueueExecutionRecorder,
};
pub use resource_envelope::{IoQueueResourceEnvelope, IoQueueResourceEnvelopeDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoWorkClass {
    Foreground,
    Checkpoint,
    Compaction,
    Scrub,
    BlobMigration,
}
