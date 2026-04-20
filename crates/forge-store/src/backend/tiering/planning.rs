mod authoritative;
mod derived;
mod read;
mod shared;

use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        tiering::{classification::summarize_window, observation},
    },
    failure::StoreError,
    tiering::{PlacementDemandSummary, PlacementObservationScopeClass},
};

pub(crate) use authoritative::plan_authoritative_tier_move;
pub(crate) use derived::plan_derived_tier_move;
pub(crate) use read::{
    plan_broadened_recall, plan_cold_recall_lease, plan_resident_read_lease,
};

pub(crate) fn summarize_placement_demand<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    scope_class: PlacementObservationScopeClass,
    scope_key: &str,
) -> Result<PlacementDemandSummary, StoreError> {
    let window = observation::observe_working_set(backend, scope_class, scope_key)?;
    let summary = summarize_window(&window);
    backend.counters().record_working_set_reclassifications(1);
    Ok(summary)
}
