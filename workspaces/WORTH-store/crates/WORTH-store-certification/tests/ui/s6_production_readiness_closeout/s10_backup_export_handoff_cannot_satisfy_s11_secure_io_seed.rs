use worth_store_certification::{
    S6ClosedS10BackupExportAdmissionSeed, S6ClosedS11SecureIoFoundationAdmissionSeed,
};

fn requires_s11_secure_io_seed(_: S6ClosedS11SecureIoFoundationAdmissionSeed) {}

fn main() {
    let backup_export: S6ClosedS10BackupExportAdmissionSeed = todo!();
    requires_s11_secure_io_seed(backup_export);
}
