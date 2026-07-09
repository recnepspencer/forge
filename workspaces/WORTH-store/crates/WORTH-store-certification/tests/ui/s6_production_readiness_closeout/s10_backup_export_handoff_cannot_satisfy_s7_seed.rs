use worth_store_certification::{
    S6ClosedS10BackupExportAdmissionSeed, S6ClosedS7PlacementAdmissionSeed,
};

fn requires_s7_seed(_: S6ClosedS7PlacementAdmissionSeed) {}

fn main() {
    let backup_export: S6ClosedS10BackupExportAdmissionSeed = todo!();
    requires_s7_seed(backup_export);
}
