use super::*;

#[test]
fn milestone_one_invariant_registrations_cover_the_runtime_pack() {
    let registrations =
        milestone_one_invariant_registrations().expect("runtime invariant registrations");
    let mut rule_ids = registrations
        .iter()
        .map(|registration| registration.rule_id().as_str().to_string())
        .collect::<Vec<_>>();
    rule_ids.sort();

    assert_eq!(
        rule_ids,
        vec![
            ".m1.naming.coverage",
            ".m1.topology.loop_wiring",
            ".m1.topology.ownership_surface",
            ".m1.topology.radial_surface",
            ".m1.topology.shell_closure",
            ".m1.topology.vertex_disks",
            ".m1.topology.wire_connectivity",
        ]
    );
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
