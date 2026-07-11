use forge_store_operations::{BackupExportCustodyAdmission, BackupExportCustodyCounterSnapshot, BackupExportCustodyMode};

fn main() {
    let readiness = todo!();
    let counters = BackupExportCustodyCounterSnapshot::for_declaration(BackupExportCustodyMode::Backup);
    let _ = BackupExportCustodyAdmission::from_outbound_declaration(
        BackupExportCustodyMode::Backup,
        readiness,
        counters,
    );
}
