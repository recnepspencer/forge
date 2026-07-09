mod authority;
mod bulk_execution;
mod bulk_planning;
mod bulk_resume;
mod certification;
mod delta;
mod durable_runtime;
mod layout_reads;
mod layout_support;
mod live_query;
mod maintenance;
mod snapshots;
mod subscription_support;
mod support;
mod tiering;

use crate::backend::{StoreBackend, StoreBackendMode};
use crate::failure::StoreError;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct WORTHStoreBuilder {
    backend_mode: StoreBackendMode,
}

impl Default for WORTHStoreBuilder {
    fn default() -> Self {
        Self {
            backend_mode: StoreBackendMode::InMemory,
        }
    }
}

impl WORTHStoreBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_memory(mut self) -> Self {
        self.backend_mode = StoreBackendMode::InMemory;
        self
    }

    pub fn local_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.backend_mode = StoreBackendMode::LocalFile(path.into());
        self
    }

    pub fn sqlite_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.backend_mode = StoreBackendMode::SqliteFile(path.into());
        self
    }

    pub fn build(self) -> Result<WORTHStore, StoreError> {
        Ok(WORTHStore {
            backend: StoreBackend::open(self.backend_mode)?,
        })
    }

    pub(crate) fn build_for_durable_recovery(self) -> Result<WORTHStore, StoreError> {
        Ok(WORTHStore {
            backend: StoreBackend::open_for_durable_recovery(self.backend_mode)?,
        })
    }

    pub fn embedded_mode(self) -> crate::EmbeddedModeBuilder {
        crate::EmbeddedModeBuilder::new(self)
    }

    pub fn durable_mode(
        self,
        runtime: worth_relational::facade::runtime::RelationalRuntime,
    ) -> crate::DurableModeBuilder {
        crate::DurableModeBuilder::new(self, runtime)
    }
}

#[derive(Debug)]
pub struct WORTHStore {
    backend: StoreBackend,
}
