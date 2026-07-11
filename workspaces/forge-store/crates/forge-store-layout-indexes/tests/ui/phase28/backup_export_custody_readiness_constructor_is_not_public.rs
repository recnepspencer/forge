use forge_store_operations::{BackupExportCustodyCounterSnapshot, BackupExportCustodyMode, BackupExportCustodyReadiness};

fn main() {
    let readiness = todo!();
    let counters = BackupExportCustodyCounterSnapshot::for_declaration(BackupExportCustodyMode::Backup);
    let _ = BackupExportCustodyReadiness::from_admitted_readiness(
        readiness,
        Some(BackupExportCustodyMode::Backup),
        counters,
    );
}
