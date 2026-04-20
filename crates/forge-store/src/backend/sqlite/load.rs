#[path = "load/authority_primary.rs"]
mod authority_primary;
#[path = "load/authority_support.rs"]
mod authority_support;
#[path = "load/bulk.rs"]
mod bulk;
#[path = "load/delta.rs"]
mod delta;
#[path = "load/layout.rs"]
mod layout;
#[path = "load/meta.rs"]
mod meta;
#[path = "load/retention.rs"]
mod retention;
#[path = "load/snapshot.rs"]
mod snapshot;

use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::records::StoreState;

pub(super) fn load_state(connection: &Connection) -> Result<StoreState, StoreError> {
    let mut state = StoreState::default();
    meta::load_canonicalization_version(connection, &mut state)?;
    authority_primary::load_authority_primary(connection, &mut state)?;
    authority_support::load_authority_support(connection, &mut state)?;
    retention::load_retention(connection, &mut state)?;
    delta::load_delta(connection, &mut state)?;
    layout::load_layout(connection, &mut state)?;
    bulk::load_bulk(connection, &mut state)?;
    snapshot::load_snapshot(connection, &mut state)?;
    meta::finalize_sequences(connection, &mut state)?;
    Ok(state)
}
