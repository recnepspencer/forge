use forge_store_certification::{
    S6ClosedS10BackupExportAdmissionSeed, S6ClosedS11SecureIoFoundationAdmissionSeed,
};

fn requires_s10_backup_export_seed(_: S6ClosedS10BackupExportAdmissionSeed) {}

fn main() {
    let secure_io: S6ClosedS11SecureIoFoundationAdmissionSeed = todo!();
    requires_s10_backup_export_seed(secure_io);
}
