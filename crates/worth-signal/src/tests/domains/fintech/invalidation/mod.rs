mod boundary_inventory;
mod boundary_inventory_m13;
mod comparator_world;
mod heterogeneous_consumer_comparators;
mod lifecycle_scenarios;
mod locality_red_controls;
mod producer_local_factor_slot_collision;
mod quote_to_risk_aspect_translation;
mod tolerance_suppressed_repricing;

pub(in crate::tests::domains::fintech) use comparator_world::{
    run_comparator_scenario, ComparatorScenarioOutcome,
};
