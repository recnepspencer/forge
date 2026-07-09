use crate::authority::AuthoritativeExportBundle;
use crate::failure::StoreError;

use super::{StoreBackend, StoreBackendMode};
use crate::backend::embedded::{EmbeddedBackendMode, EmbeddedStoreBackend};
use crate::backend::sqlite::SqliteStoreBackend;

impl StoreBackend {
    pub fn open(mode: StoreBackendMode) -> Result<Self, StoreError> {
        match mode {
            StoreBackendMode::InMemory => Ok(Self::Embedded(EmbeddedStoreBackend::open(
                EmbeddedBackendMode::InMemory,
            )?)),
            StoreBackendMode::LocalFile(path) => Ok(Self::Embedded(EmbeddedStoreBackend::open(
                EmbeddedBackendMode::LocalFile(path),
            )?)),
            StoreBackendMode::SqliteFile(path) => Ok(Self::Sqlite(SqliteStoreBackend::open(path)?)),
        }
    }

    pub fn open_for_durable_recovery(mode: StoreBackendMode) -> Result<Self, StoreError> {
        match mode {
            StoreBackendMode::InMemory => Ok(Self::Embedded(
                EmbeddedStoreBackend::open_for_durable_recovery(EmbeddedBackendMode::InMemory)?,
            )),
            StoreBackendMode::LocalFile(path) => Ok(Self::Embedded(
                EmbeddedStoreBackend::open_for_durable_recovery(EmbeddedBackendMode::LocalFile(
                    path,
                ))?,
            )),
            StoreBackendMode::SqliteFile(path) => Ok(Self::Sqlite(
                SqliteStoreBackend::open_for_durable_recovery(path)?,
            )),
        }
    }

    pub fn from_export_bundle_with_compatibility(
        bundle: AuthoritativeExportBundle,
    ) -> Result<(Self, crate::CompatibilityRestoreExecutionOutcome), StoreError> {
        let execution =
            crate::backend::engine::compatibility_runtime::execute_authoritative_export_restore(
                &bundle,
            )?;
        let outcome = crate::CompatibilityRestoreExecutionOutcome::new(
            execution.receipts().len(),
            execution.receipts().len(),
            crate::Milestone12AdmissionReport::from_admission_counters(execution.counters()),
        );
        Ok((
            Self::Embedded(EmbeddedStoreBackend::from_export_bundle(bundle)?),
            outcome,
        ))
    }

    pub fn from_export_bundle(bundle: AuthoritativeExportBundle) -> Result<Self, StoreError> {
        Ok(Self::from_export_bundle_with_compatibility(bundle)?.0)
    }
}
