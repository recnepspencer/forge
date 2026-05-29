use super::*;

#[test]
fn runtime_invariant_pack_matches_declared_bootstrap_plan() {
    let declared = bootstrap_runtime_invariant_plan();
    let registrations =
        milestone_one_runtime_invariants().expect("runtime invariant registrations");

    assert_eq!(registrations.len(), declared.topology.len());
}

#[test]
fn runtime_builder_helper_applies_schema_and_runtime_invariants() {
    let _runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
}

#[test]
fn runtime_invariants_accept_seeded_topology_on_the_actual_authority_path() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let seeded = seed_minimal_topology(&mut runtime, "runtime-invariant-seed")
        .expect("seeded milestone-one topology should commit through runtime invariants");

    let read = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot should remain readable");
    assert!(read.get_entity(seeded.model).is_some());
    assert!(read.get_entity(seeded.shell).is_some());
}
