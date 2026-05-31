use std::collections::BTreeSet;

use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::super::super::support::execute_current_head_topology_declaration;
use super::support::{
    shell_split_declaration_for_fixture, shell_split_fixture, wire_split_declaration_for_fixture,
    wire_split_fixture,
};
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_canonical_wire_split_batch_through_declaration_entry() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native.split-wire.runtime",
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed topology");
    let fixture = wire_split_fixture(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.split-wire.runtime.workspace").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        wire_split_declaration_for_fixture(
            "query-native.split-wire.runtime",
            &fixture.moved_half_edge_ids,
        ),
    )
    .expect("canonical wire split batch should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.split_connected_half_edge_set_to_new_wire"
    );
    let topology = execution.materialized.topology();
    let new_wire = topology
        .wires
        .iter()
        .find(|wire| wire.label == "query-native.split-wire.runtime.new-wire")
        .expect("wire split should materialize the replacement wire");
    let retained_wire = topology
        .wires
        .iter()
        .find(|wire| wire.entity_id == fixture.retained_wire_id)
        .expect("wire split should keep the retained wire");
    assert_eq!(
        new_wire
            .half_edge_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        fixture
            .moved_half_edge_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
    );
    assert_eq!(
        retained_wire
            .half_edge_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        fixture
            .retained_half_edge_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
    );
}

#[test]
fn current_head_runtime_executes_canonical_shell_split_batch_through_declaration_entry() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native.split-shell.runtime",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let fixture = shell_split_fixture(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, "query-native.split-shell.runtime.workspace")
        .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        shell_split_declaration_for_fixture(
            "query-native.split-shell.runtime",
            fixture.region_id,
            fixture.moved_face_id,
        ),
    )
    .expect("canonical shell split batch should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.split_single_face_from_two_face_shell_to_new_shell"
    );
    let topology = execution.materialized.topology();
    let new_shell = topology
        .shells
        .iter()
        .find(|shell| shell.label == "query-native.split-shell.runtime.new-shell")
        .expect("shell split should materialize the replacement shell");
    let retained_shell = topology
        .shells
        .iter()
        .find(|shell| shell.entity_id == fixture.retained_shell_id)
        .expect("shell split should keep the retained shell");
    assert_eq!(new_shell.region_id, Some(fixture.region_id));
    assert_eq!(new_shell.face_ids, vec![fixture.moved_face_id]);
    assert_eq!(retained_shell.face_ids, vec![fixture.retained_face_id]);
}
