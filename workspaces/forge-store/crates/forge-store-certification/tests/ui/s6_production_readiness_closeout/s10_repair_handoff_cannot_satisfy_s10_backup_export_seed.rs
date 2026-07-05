use forge_store_certification::{
    S6ClosedS10BackupExportAdmissionSeed, S6ClosedS10RepairAdmissionSeed,
};

fn requires_s10_backup_export_seed(_: S6ClosedS10BackupExportAdmissionSeed) {}

fn main() {
    let repair: S6ClosedS10RepairAdmissionSeed = todo!();
    requires_s10_backup_export_seed(repair);
}
