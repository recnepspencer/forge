use forge_store_contracts::{
    BackgroundPressureDeclaration, BackgroundPressureKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlobBackgroundPressureKind {
    Ingest,
    Migration,
    Compaction,
}

pub const fn blob_ingest_background_pressure_shape(
    bytes: u64,
) -> BackgroundPressureDeclaration {
    BackgroundPressureDeclaration::blob_ingest_pressure(bytes)
}

pub const fn blob_migration_background_pressure_shape(
    bytes: u64,
) -> BackgroundPressureDeclaration {
    BackgroundPressureDeclaration::blob_migration_pressure(bytes)
}

pub const fn blob_compaction_background_pressure_shape() -> BackgroundPressureDeclaration
{
    BackgroundPressureDeclaration::compaction_rewrite()
}

pub const fn blob_background_pressure_kind(
    declaration: BackgroundPressureDeclaration,
) -> Option<BlobBackgroundPressureKind> {
    match declaration.kind() {
        BackgroundPressureKind::CompactionRewrite => {
            Some(BlobBackgroundPressureKind::Compaction)
        }
        BackgroundPressureKind::BlobIngestPressure => {
            Some(BlobBackgroundPressureKind::Ingest)
        }
        BackgroundPressureKind::BlobMigrationPressure => {
            Some(BlobBackgroundPressureKind::Migration)
        }
        _ => None,
    }
}
