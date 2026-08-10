use crate::{failure::StoreError, SubscriptionSupportAccessStructure, SubscriptionSupportCatalog};

use super::super::super::super::records::StoreState;
use super::super::super::helpers::{deserialize_json, sqlite_error};
use rusqlite::Connection;

pub(super) fn load_access_state(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    let (access_structures_verified, access_structure_debts) = connection
        .query_row(
            "SELECT verified, debted_json FROM subscription_support_access_structure_state WHERE state_id = 'first_ship'",
            [],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    deserialize_json::<Vec<SubscriptionSupportAccessStructure>>(row.get(1)?)?,
                ))
            },
        )
        .or_else(|error| match error {
            rusqlite::Error::QueryReturnedNoRows => Ok((1, Vec::new())),
            other => Err(other),
        })
        .and_then(|(value, debts)| match value {
            0 => Ok((false, normalize_access_structure_debts(debts))),
            1 => Ok((true, Vec::new())),
            other => Err(rusqlite::Error::IntegralValueOutOfRange(0, other)),
        })
        .map_err(sqlite_error)?;
    state.subscription_support_access_structures_verified = access_structures_verified;
    state.subscription_support_access_structure_debts =
        if !access_structures_verified && access_structure_debts.is_empty() {
            SubscriptionSupportCatalog::first_ship()
                .access_structures()
                .required()
                .to_vec()
        } else {
            access_structure_debts
        };
    Ok(())
}

fn normalize_access_structure_debts(
    mut debts: Vec<SubscriptionSupportAccessStructure>,
) -> Vec<SubscriptionSupportAccessStructure> {
    debts.sort();
    debts.dedup();
    debts
}
