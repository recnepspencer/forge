use crate::{
    failure::StoreError,
    media::{DurabilityBarrierClass, DurableBackendFamily, DurableMediaReport},
    AuthoritativeExportBundle,
};
use std::path::PathBuf;

use super::{
    engine::{StateBackedStoreBackend, StatePersistence},
    persistence::{load_state, persist_state_atomic},
    records::StoreState,
};

#[derive(Debug, Clone)]
pub enum EmbeddedBackendMode {
    InMemory,
    LocalFile(PathBuf),
}

#[derive(Debug)]
pub struct EmbeddedPersistence {
    mode: EmbeddedBackendMode,
}

pub type EmbeddedStoreBackend = StateBackedStoreBackend<EmbeddedPersistence>;

impl EmbeddedStoreBackend {
    pub fn open(mode: EmbeddedBackendMode) -> Result<Self, StoreError> {
        StateBackedStoreBackend::open_with_persistence(EmbeddedPersistence { mode })
    }

    pub fn from_export_bundle(bundle: AuthoritativeExportBundle) -> Result<Self, StoreError> {
        StateBackedStoreBackend::from_export_bundle_with_persistence(
            EmbeddedPersistence {
                mode: EmbeddedBackendMode::InMemory,
            },
            bundle,
        )
    }
}

impl StatePersistence for EmbeddedPersistence {
    fn load_state(&mut self) -> Result<StoreState, StoreError> {
        match &self.mode {
            EmbeddedBackendMode::InMemory => Ok(StoreState::default()),
            EmbeddedBackendMode::LocalFile(path) => load_state(path),
        }
    }

    fn persist_state(&mut self, state: &StoreState) -> Result<DurableMediaReport, StoreError> {
        match &self.mode {
            EmbeddedBackendMode::InMemory => Ok(self.durable_media_report()),
            EmbeddedBackendMode::LocalFile(path) => {
                persist_state_atomic(path, state)?;
                Ok(self.durable_media_report())
            }
        }
    }

    fn durable_media_report(&self) -> DurableMediaReport {
        match &self.mode {
            EmbeddedBackendMode::InMemory => DurableMediaReport::new(
                DurableBackendFamily::InMemory,
                DurabilityBarrierClass::ProcessBufferOnly,
                DurabilityBarrierClass::ProcessBufferOnly,
                DurabilityBarrierClass::ProcessBufferOnly,
            ),
            EmbeddedBackendMode::LocalFile(_) => DurableMediaReport::new(
                DurableBackendFamily::LocalFileAtomicRewrite,
                DurabilityBarrierClass::FileContentDurable,
                DurabilityBarrierClass::RenameOrPublicationMarkerDurable,
                DurabilityBarrierClass::FileContentDurable,
            ),
        }
    }
}
