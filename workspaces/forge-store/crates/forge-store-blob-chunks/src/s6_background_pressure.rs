use forge_store_contracts::{S6BackgroundPressureDeclaration, S6BackgroundPressureKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlobBackgroundPressureKind {
    Ingest,
    Migration,
}

pub const fn blob_ingest_background_pressure_shape(bytes: u64) -> S6BackgroundPressureDeclaration {
    S6BackgroundPressureDeclaration::blob_ingest_pressure(bytes)
}

pub const fn blob_migration_background_pressure_shape(
    bytes: u64,
) -> S6BackgroundPressureDeclaration {
    S6BackgroundPressureDeclaration::blob_migration_pressure(bytes)
}

pub const fn blob_background_pressure_kind(
    declaration: S6BackgroundPressureDeclaration,
) -> Option<BlobBackgroundPressureKind> {
    match declaration.kind() {
        S6BackgroundPressureKind::BlobIngestPressure => Some(BlobBackgroundPressureKind::Ingest),
        S6BackgroundPressureKind::BlobMigrationPressure => {
            Some(BlobBackgroundPressureKind::Migration)
        }
        _ => None,
    }
}
