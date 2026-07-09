use worth_store_certification::{
    S6ClosedS10BackupExportAdmissionSeed, S6ClosedS7PlacementAdmissionSeed,
};

fn requires_backup_export_seed(_: S6ClosedS10BackupExportAdmissionSeed) {}

fn main() {
    let placement: S6ClosedS7PlacementAdmissionSeed = todo!();
    requires_backup_export_seed(placement);
}
