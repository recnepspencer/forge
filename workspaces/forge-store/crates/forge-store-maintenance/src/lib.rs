#![forbid(unsafe_code)]

mod memory_envelopes;

pub use memory_envelopes::{
    CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope, MaintenanceMemoryEnvelopeDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceWorkClass {
    SnapshotRefresh,
    Compaction,
    Reclaim,
    ReplicationPreparation,
    TierMovement,
}
