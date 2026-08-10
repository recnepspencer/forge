use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceDeclarationClass {
    Retention,
    Compaction,
    Reclaim,
    Rebuild,
    DerivedFamilyRebuild,
    SnapshotRefresh,
    ReplicationPreparation,
    MaintenanceAudit,
    TierPlacementProposal,
    TierMoveExecution,
}

mod compaction;
mod operational;
mod rebuild;
mod reclaim;
mod retention;

pub use compaction::CompactionMaintenanceDeclaration;
pub use operational::{
    MaintenanceAuditMaintenanceDeclaration, ReplicationPreparationMaintenanceDeclaration,
    SnapshotRefreshMaintenanceDeclaration, TierMoveMaintenanceDeclaration,
    TierPlacementMaintenanceDeclaration,
};
pub use rebuild::{DerivedFamilyRebuildMaintenanceDeclaration, RebuildMaintenanceDeclaration};
pub use reclaim::{AuthoritativeReclaimMaintenanceDeclaration, ReclaimMaintenanceDeclaration};
pub use retention::RetentionMaintenanceDeclaration;
