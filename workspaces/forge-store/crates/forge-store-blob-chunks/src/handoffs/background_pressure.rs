use forge_store_contracts::{
    IoPressureBackgroundPressureDeclaration, IoPressureBackgroundPressureKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlobBackgroundPressureKind {
    Ingest,
    Migration,
    Compaction,
}

pub const fn blob_ingest_background_pressure_shape(
    bytes: u64,
) -> IoPressureBackgroundPressureDeclaration {
    IoPressureBackgroundPressureDeclaration::blob_ingest_pressure(bytes)
}

pub const fn blob_migration_background_pressure_shape(
    bytes: u64,
) -> IoPressureBackgroundPressureDeclaration {
    IoPressureBackgroundPressureDeclaration::blob_migration_pressure(bytes)
}

pub const fn blob_compaction_background_pressure_shape() -> IoPressureBackgroundPressureDeclaration
{
    IoPressureBackgroundPressureDeclaration::compaction_rewrite()
}

pub const fn blob_background_pressure_kind(
    declaration: IoPressureBackgroundPressureDeclaration,
) -> Option<BlobBackgroundPressureKind> {
    match declaration.kind() {
        IoPressureBackgroundPressureKind::CompactionRewrite => {
            Some(BlobBackgroundPressureKind::Compaction)
        }
        IoPressureBackgroundPressureKind::BlobIngestPressure => {
            Some(BlobBackgroundPressureKind::Ingest)
        }
        IoPressureBackgroundPressureKind::BlobMigrationPressure => {
            Some(BlobBackgroundPressureKind::Migration)
        }
        _ => None,
    }
}
