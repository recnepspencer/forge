use rusqlite::Connection;

use crate::failure::StoreError;

use super::super::super::records::StoreState;

mod access_state;
mod actions;
mod counters;
mod maintenance_debt;
mod maintenance_descriptors;
mod stored_record_sets;

pub(super) fn load_subscription_support(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    clear_subscription_support_state(state);
    stored_record_sets::load_stored_record_sets(connection, state)?;
    actions::load_action_records(connection, state)?;
    maintenance_descriptors::load_maintenance_descriptor_records(connection, state)?;
    maintenance_debt::load_maintenance_debt_records(connection, state)?;
    counters::load_counter_snapshot(connection, state)?;
    access_state::load_access_state(connection, state)?;
    Ok(())
}

fn clear_subscription_support_state(state: &mut StoreState) {
    state.subscription_support_record_sets.clear();
    state.subscription_support_action_records.clear();
    state
        .subscription_support_maintenance_descriptor_records
        .clear();
    state.subscription_support_maintenance_debt_records.clear();
}
