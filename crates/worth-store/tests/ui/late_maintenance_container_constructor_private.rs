use worth_store::{
    DerivedFamilyRebuildMaintenanceDeclaration, MaintenanceAuditMaintenanceDeclaration,
    ReplicationPreparationMaintenanceDeclaration, SnapshotRefreshMaintenanceDeclaration,
};

fn main() {
    let _ = DerivedFamilyRebuildMaintenanceDeclaration::new(
        "basis:derived-rebuild",
        "family:derived-local",
        "rebuild:derived-index",
    );
    let _ = SnapshotRefreshMaintenanceDeclaration::new(
        "snapshot_family",
        "family:snapshot-local",
        "refresh:publication-support",
    );
    let _ = ReplicationPreparationMaintenanceDeclaration::new(
        "replication_family",
        "family:replication-local",
        "prepare:capsule-handoff",
    );
    let _ = MaintenanceAuditMaintenanceDeclaration::new(
        "audit_family",
        "family:audit-local",
        "audit:queue-summary-parity",
    );
}
