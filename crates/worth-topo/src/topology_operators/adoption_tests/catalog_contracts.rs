use std::collections::BTreeSet;

use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportStatus,
};

use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

use super::super::{
    topology_operator_graph_obligation_catalog,
    topology_operator_graph_obligation_registration_declaration,
    topology_operator_graph_obligation_selector_coverage,
    topology_operator_graph_obligation_support_matrix,
    topology_operator_graph_obligation_support_pin, topology_operator_relation_touch_descriptor,
    TopologyOperatorGraphObligationAdoptionStatus, TopologyOperatorGraphObligationLoweringPath,
    TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY,
};

#[test]
fn operator_catalog_accounts_for_phase_seventeen_targets() {
    let catalog = topology_operator_graph_obligation_catalog();
    let covered = catalog.covered_rows().collect::<Vec<_>>();
    let residue = catalog.residue_rows().collect::<Vec<_>>();

    assert_eq!(catalog.rows().len(), 8);
    assert_eq!(covered.len(), 2);
    assert_eq!(residue.len(), 6);
    assert!(covered.iter().all(|row| {
        row.adoption_status() == TopologyOperatorGraphObligationAdoptionStatus::Covered
    }));

    let covered_paths = covered
        .iter()
        .map(|row| row.lowering_path().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        covered_paths,
        BTreeSet::from(["contribution-orchestration", "graph-composition"])
    );

    let residue_classes = residue
        .iter()
        .map(|row| row.residue_class().expect("residue row names its class"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        residue_classes,
        BTreeSet::from([
            "face-inner-loop-command-batch-operator",
            "existing-entity-incoming-relation-count-mismatch-guards",
            "milestone-one-reference-integrity-pack",
            "scalar-topology-mutation-fronts",
            "shell-membership-command-batch-operator",
            "wire-rehome-command-batch-operator",
        ])
    );
}

#[test]
fn operator_catalog_registration_declaration_is_selector_covered() {
    let declaration = topology_operator_graph_obligation_registration_declaration()
        .expect("operator catalog registration declaration should build");
    let coverage = topology_operator_graph_obligation_selector_coverage();

    assert_eq!(
        declaration.family(),
        TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY
    );
    assert_eq!(declaration.registrations().len(), 2);
    assert_eq!(coverage.row_count(), 2);
    assert!(coverage.covers_registration_declaration(&declaration));
    assert!(declaration
        .registrations()
        .iter()
        .all(|registration| { !registration.touch_selector().selector_digest().is_empty() }));
}

#[test]
fn operator_catalog_support_pin_matches_declared_matrix() {
    let declaration = topology_operator_graph_obligation_registration_declaration()
        .expect("operator catalog registration declaration should build");
    let matrix = topology_operator_graph_obligation_support_matrix();
    let pin = topology_operator_graph_obligation_support_pin();

    pin.evaluate_for_registrations(&matrix, declaration.registrations())
        .expect("operator catalog support pin must match the declared support matrix");
    assert_eq!(pin.row_count(), 2);
}

#[test]
fn operator_catalog_support_matrix_does_not_overstate_non_advisory_execution() {
    let matrix = topology_operator_graph_obligation_support_matrix();

    let non_advisory_graph_composition_statuses = ForgeQueryGraphObligationKind::ALL
        .into_iter()
        .filter(|kind| *kind != ForgeQueryGraphObligationKind::AdvisoryObligation)
        .map(|kind| {
            matrix
                .rows_for_kind(kind)
                .find(|row| {
                    row.support_lane() == ForgeQueryGraphObligationSupportLane::GraphComposition
                })
                .expect("matrix should explicitly classify graph composition lane")
                .status()
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(
        non_advisory_graph_composition_statuses,
        BTreeSet::from([ForgeQueryGraphObligationSupportStatus::NotApplicable])
    );
}

#[test]
fn runtime_operator_catalog_only_registers_graph_composition_rows() {
    let relational_runtime = build_milestone_one_runtime().expect("milestone one runtime");
    let workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(relational_runtime),
        "operator-catalog.runtime-inspection",
    )
    .expect("topology runtime should assemble");
    let runtime = workspace.into_runtime();
    let catalog_rows = runtime
        .graph_obligation_registration_catalog()
        .registrations()
        .iter()
        .filter(|registration| {
            registration.rule_identity().namespace() == "worth-topo.topology-operator"
                && registration.rule_identity().name()
                    == "topology.rewire_loop_successor_program.graph-obligation"
        })
        .collect::<Vec<_>>();

    assert_eq!(catalog_rows.len(), 1);
    let row = catalog_rows[0];
    assert_eq!(
        row.support_posture().lane(),
        ForgeQueryGraphObligationSupportLane::GraphComposition
    );
    assert_eq!(
        row.support_posture().status(),
        ForgeQueryGraphObligationSupportStatus::DiagnosticOnly
    );

    let selection = runtime.select_graph_obligations_for_touch(
        &topology_operator_relation_touch_descriptor()
            .expect("topology operator touch descriptor should build"),
        &ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle(),
    );
    assert_eq!(selection.matched_obligation_count(), 1);
    assert_eq!(
        selection.matched_registrations()[0].registration_digest(),
        row.registration_digest()
    );
}

#[test]
fn catalog_distinguishes_covered_rows_from_explicit_residue_paths() {
    let catalog = topology_operator_graph_obligation_catalog();

    assert!(catalog
        .covered_rows()
        .all(|row| row.registration().is_some()));
    assert!(catalog
        .residue_rows()
        .all(|row| row.registration().is_none()));
    assert!(catalog.residue_rows().any(|row| {
        row.lowering_path() == TopologyOperatorGraphObligationLoweringPath::ScalarMutation
    }));
}
