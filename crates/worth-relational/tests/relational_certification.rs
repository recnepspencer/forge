#[path = "relational_certification/baseline_layers.rs"]
mod baseline_layers;
#[path = "relational_certification/comparison_contracts.rs"]
mod comparison_contracts;
#[path = "relational_certification/delta_contracts.rs"]
mod delta_contracts;
#[path = "relational_certification/field_contracts.rs"]
mod field_contracts;
#[path = "relational_certification/field_values.rs"]
mod field_values;
#[path = "relational_certification/observation_contracts.rs"]
mod observation_contracts;
#[path = "relational_certification/oracle_ancestry.rs"]
mod oracle_ancestry;
#[path = "relational_certification/oracle_application.rs"]
mod oracle_application;
#[path = "relational_certification/phase4_compatibility.rs"]
mod phase4_compatibility;
#[path = "relational_certification/phase4_cost.rs"]
mod phase4_cost;
#[path = "relational_certification/phase4_fork.rs"]
mod phase4_fork;
#[path = "relational_certification/phase4_fork_evidence.rs"]
mod phase4_fork_evidence;
#[path = "relational_certification/phase4_owner_binding.rs"]
mod phase4_owner_binding;
#[path = "relational_certification/production_failures.rs"]
mod production_failures;
#[path = "relational_certification/production_snapshot_failures.rs"]
mod production_snapshot_failures;
#[path = "relational_certification/production_world.rs"]
mod production_world;
#[path = "relational_certification/profile_contracts.rs"]
mod profile_contracts;
#[path = "relational_certification/read_contracts.rs"]
mod read_contracts;
#[path = "relational_certification/schema_contracts.rs"]
mod schema_contracts;
#[path = "relational_certification/trace_replay.rs"]
mod trace_replay;
#[path = "relational_certification/world/mod.rs"]
mod world;

#[test]
fn supply_chain_semantic_world_target_is_present() {
    let scale = world::supply_chain::SupplyChainScale::court();
    let baseline = world::supply_chain::SupplyChainBaseline::operating(scale);
    assert_eq!(baseline.scale.name, world::supply_chain::ScaleName::Court);
    assert_eq!(baseline.definition.entities.len(), 244);
}
