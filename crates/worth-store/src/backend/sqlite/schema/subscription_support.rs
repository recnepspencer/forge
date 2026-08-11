use crate::failure::StoreError;
use rusqlite::Connection;

#[path = "subscription_support/access.rs"]
mod access;
#[path = "subscription_support/indexes.rs"]
mod indexes;
#[path = "subscription_support/migrations.rs"]
mod migrations;
#[path = "subscription_support/tables.rs"]
mod tables;

pub(super) fn create_subscription_support_schema(
    connection: &Connection,
) -> Result<(), StoreError> {
    let access_gap = access::capture_subscription_support_access_gap(connection)?;
    tables::create_subscription_support_tables(connection)?;
    migrations::migrate_subscription_support_columns(connection)?;
    indexes::create_subscription_support_lookup_indexes(connection)?;
    access::mark_subscription_support_access_debt(connection, access_gap)?;
    Ok(())
}
