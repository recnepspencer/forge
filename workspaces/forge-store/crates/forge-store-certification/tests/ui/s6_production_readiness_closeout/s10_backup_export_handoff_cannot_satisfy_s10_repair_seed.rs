use forge_store_certification::{
    S6ClosedS10BackupExportAdmissionSeed, S6ClosedS10RepairAdmissionSeed,
};

fn requires_s10_repair_seed(_: S6ClosedS10RepairAdmissionSeed) {}

fn main() {
    let backup_export: S6ClosedS10BackupExportAdmissionSeed = todo!();
    requires_s10_repair_seed(backup_export);
}
