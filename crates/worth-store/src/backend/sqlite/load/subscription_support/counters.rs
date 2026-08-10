use crate::{failure::StoreError, SubscriptionSupportCounterSnapshot};

use super::super::super::super::records::StoreState;
use super::super::super::helpers::{deserialize_json, sqlite_error};
use rusqlite::Connection;

pub(super) fn load_counter_snapshot(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    state.subscription_support_counter_snapshot = connection
        .query_row(
            "SELECT payload_json FROM subscription_support_counter_snapshot WHERE counter_id = 'first_ship'",
            [],
            |row| deserialize_json::<SubscriptionSupportCounterSnapshot>(row.get(0)?),
        )
        .or_else(|error| match error {
            rusqlite::Error::QueryReturnedNoRows => {
                Ok(SubscriptionSupportCounterSnapshot::default())
            }
            other => Err(other),
        })
        .map_err(sqlite_error)?;
    Ok(())
}
