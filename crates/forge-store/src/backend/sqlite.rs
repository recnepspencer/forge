#[path = "sqlite/helpers.rs"]
mod helpers;
#[path = "sqlite/schema.rs"]
mod schema;
#[path = "sqlite/load.rs"]
mod load;
#[path = "sqlite/persist.rs"]
mod persist;

use crate::{
    failure::StoreError,
    media::{DurabilityBarrierClass, DurableBackendFamily, DurableMediaReport},
};
use rusqlite::Connection;
use std::path::PathBuf;

use super::{
    engine::{StateBackedStoreBackend, StatePersistence},
    records::StoreState,
};

#[derive(Debug)]
pub struct SqlitePersistence {
    connection: Connection,
}

pub type SqliteStoreBackend = StateBackedStoreBackend<SqlitePersistence>;

impl SqliteStoreBackend {
    pub fn open(path: PathBuf) -> Result<Self, StoreError> {
        open_backend(path, StateBackedStoreBackend::open_with_persistence)
    }

    pub fn open_for_durable_recovery(path: PathBuf) -> Result<Self, StoreError> {
        open_backend(
            path,
            StateBackedStoreBackend::open_with_persistence_for_durable_recovery,
        )
    }
}

impl StatePersistence for SqlitePersistence {
    fn load_state(&mut self) -> Result<StoreState, StoreError> {
        load::load_state(&self.connection)
    }

    fn persist_state(&mut self, state: &StoreState) -> Result<DurableMediaReport, StoreError> {
        persist::persist_state(&mut self.connection, state)?;
        Ok(self.durable_media_report())
    }

    fn durable_media_report(&self) -> DurableMediaReport {
        DurableMediaReport::new(
            DurableBackendFamily::SqliteTransactional,
            DurabilityBarrierClass::TransactionalCommitDurable,
            DurabilityBarrierClass::TransactionalCommitDurable,
            DurabilityBarrierClass::TransactionalCommitDurable,
        )
    }
}

fn open_backend(
    path: PathBuf,
    open_fn: fn(SqlitePersistence) -> Result<SqliteStoreBackend, StoreError>,
) -> Result<SqliteStoreBackend, StoreError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let connection = Connection::open(path).map_err(helpers::sqlite_error)?;
    schema::configure_connection(&connection)?;
    schema::create_schema(&connection)?;
    open_fn(SqlitePersistence { connection })
}
