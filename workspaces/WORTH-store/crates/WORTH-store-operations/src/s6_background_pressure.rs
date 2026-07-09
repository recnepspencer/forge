use worth_store_contracts::{S6BackgroundPressureDeclaration, S6BackgroundPressureKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationsBackgroundPressureKind {
    ReplicationPrep,
    BackupPrep,
    Repair,
}

pub const fn replication_prep_background_pressure_shape(
    read_ahead_pages: u64,
) -> S6BackgroundPressureDeclaration {
    S6BackgroundPressureDeclaration::replication_prep_read(read_ahead_pages)
}

pub const fn backup_prep_background_pressure_shape(
    bytes: u64,
    read_ahead_pages: u64,
) -> S6BackgroundPressureDeclaration {
    S6BackgroundPressureDeclaration::backup_prep_read(bytes, read_ahead_pages)
}

pub const fn repair_background_pressure_shape(
    read_ahead_pages: u64,
) -> S6BackgroundPressureDeclaration {
    S6BackgroundPressureDeclaration::repair_scan(read_ahead_pages)
}

pub const fn operations_background_pressure_kind(
    declaration: S6BackgroundPressureDeclaration,
) -> Option<OperationsBackgroundPressureKind> {
    match declaration.kind() {
        S6BackgroundPressureKind::ReplicationPrepRead => {
            Some(OperationsBackgroundPressureKind::ReplicationPrep)
        }
        S6BackgroundPressureKind::BackupPrepRead => {
            Some(OperationsBackgroundPressureKind::BackupPrep)
        }
        S6BackgroundPressureKind::RepairScan => Some(OperationsBackgroundPressureKind::Repair),
        _ => None,
    }
}
