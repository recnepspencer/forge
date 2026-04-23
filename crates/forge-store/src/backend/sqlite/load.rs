#[path = "load/authority_primary.rs"]
mod authority_primary;
#[path = "load/authority_support.rs"]
mod authority_support;
#[path = "load/bulk.rs"]
mod bulk;
#[path = "load/compatibility.rs"]
mod compatibility;
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
#[path = "load/subscription_support.rs"]
mod subscription_support;
#[path = "load/tiering.rs"]
mod tiering;

use crate::backend::maintenance::summaries;
use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::records::StoreState;

pub(super) fn load_state(connection: &Connection) -> Result<StoreState, StoreError> {
    let mut state = StoreState::default();
    meta::load_canonicalization_version(connection, &mut state)?;
    authority_primary::load_authority_primary(connection, &mut state)?;
    compatibility::load_compatibility(connection, &mut state)?;
    subscription_support::load_subscription_support(connection, &mut state)?;
    authority_support::load_authority_support(connection, &mut state)?;
    retention::load_retention(connection, &mut state)?;
    delta::load_delta(connection, &mut state)?;
    layout::load_layout(connection, &mut state)?;
    bulk::load_bulk(connection, &mut state)?;
    snapshot::load_snapshot(connection, &mut state)?;
    tiering::load_tiering(connection, &mut state)?;
    meta::finalize_sequences(connection, &mut state)?;
    summaries::record_scheduler_boot_state(&mut state);
    summaries::backfill_scheduler_summaries_if_missing(&mut state);
    Ok(state)
}
