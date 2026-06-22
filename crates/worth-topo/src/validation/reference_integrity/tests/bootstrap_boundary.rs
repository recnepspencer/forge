use super::*;
use forge_relational::facade::runtime::InvariantExecutionPoint;

#[test]
fn milestone_one_invariant_registrations_cover_the_runtime_pack() {
    let registrations =
        milestone_one_invariant_registrations().expect("runtime invariant registrations");
    let mut rule_ids = registrations
        .iter()
        .map(|registration| registration.rule_id().as_str().to_string())
        .collect::<Vec<_>>();
    rule_ids.sort();

    rule_ids.dedup();

    assert_eq!(rule_ids, milestone_one_rule_ids());
    assert_eq!(registrations.len(), milestone_one_rule_ids().len() * 2);
}

#[test]
fn milestone_one_invariant_registrations_share_identity_across_graph_composition_and_commit_backstop(
) {
    let registrations =
        milestone_one_invariant_registrations().expect("runtime invariant registrations");

    for rule_id in milestone_one_rule_ids() {
        let mut execution_points = registrations
            .iter()
            .filter(|registration| registration.rule_id().as_str() == rule_id)
            .map(|registration| registration.descriptor().operational.execution_point)
            .collect::<Vec<_>>();
        execution_points.sort();
        execution_points.dedup();

        assert_eq!(
            execution_points,
            vec![
                InvariantExecutionPoint::CommitBoundary,
                InvariantExecutionPoint::GraphComposition,
            ],
            "rule {rule_id} must be one semantic identity with graph-composition execution and commit backstop"
        );
    }
}

#[test]
fn milestone_one_invariant_registrations_do_not_create_execution_point_specific_rule_id_clones() {
    let registrations =
        milestone_one_invariant_registrations().expect("runtime invariant registrations");

    for registration in &registrations {
        let rule_id = registration.rule_id().as_str();
        assert!(
            !rule_id.contains("graph_composition")
                && !rule_id.contains("graph-composition")
                && !rule_id.contains("commit_boundary")
                && !rule_id.contains("commit-boundary"),
            "rule id {rule_id} encodes execution point instead of semantic identity"
        );
    }

    for rule_id in milestone_one_rule_ids() {
        let matching_count = registrations
            .iter()
            .filter(|registration| registration.rule_id().as_str() == rule_id)
            .count();
        assert_eq!(
            matching_count, 2,
            "rule {rule_id} should have exactly graph-composition and commit-backstop registrations"
        );
    }
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

    let seeded =
        crate::test_support::schema_topology_authoring_boundary::seed_minimal_topology_through_schema_execution(
            &mut runtime,
            "runtime-invariant-seed",
        )
        .expect("seeded milestone-one topology should commit through runtime invariants");

    let read = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot should remain readable");
    assert!(read.get_entity(seeded.model).is_some());
    assert!(read.get_entity(seeded.shell).is_some());
}

fn milestone_one_rule_ids() -> Vec<String> {
    [
        ".m1.naming.coverage",
        ".m1.topology.loop_wiring",
        ".m1.topology.ownership_surface",
        ".m1.topology.radial_surface",
        ".m1.topology.shell_closure",
        ".m1.topology.vertex_disks",
        ".m1.topology.wire_connectivity",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}
