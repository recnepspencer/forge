#![forbid(unsafe_code)]

pub mod layout_projection;

mod memory_envelopes;
mod scrub_workflow;

pub use layout_projection::{
    MaintenanceQueueAccessBudget, MaintenanceQueueClass, MaintenanceQueueInterferencePosture,
    MaintenanceQueueLayoutReport,
};
pub use memory_envelopes::{
    CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope, MaintenanceMemoryEnvelopeDenial,
};
pub use scrub_workflow::PhysicalIntegrityScrubWorkflow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceWorkClass {
    SnapshotRefresh,
    Compaction,
    Reclaim,
    ReplicationPreparation,
    TierMovement,
    PhysicalIntegrityScrub,
}
