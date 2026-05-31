use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::support::{
    shell_split_declaration_for_fixture, shell_split_fixture, wire_split_declaration_for_fixture,
    wire_split_fixture,
};
use crate::certification::projection_closeout::tests::domain_query::support::current_head_unsupported_declaration_families;
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_rejects_disconnected_wire_split_batch_before_any_declaration_entry_execution(
) {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native.denied-split-wire.runtime",
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed topology");
    let fixture = wire_split_fixture(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.denied-split-wire.runtime.workspace")
            .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    let declaration = wire_split_declaration_for_fixture(
        "query-native.denied-split-wire.runtime",
        &fixture.disconnected_half_edge_ids,
    );
    assert!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration)
            .contains(&crate::facade::TopologyEditFamily::AttachShellOrWireMembership)
    );
}

#[test]
fn current_head_runtime_rejects_three_face_shell_split_batch_before_any_declaration_entry_execution(
) {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native.denied-split-shell.runtime",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 3 },
    )
    .expect("seed topology");
    let fixture = shell_split_fixture(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        "query-native.denied-split-shell.runtime.workspace",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    let declaration = shell_split_declaration_for_fixture(
        "query-native.denied-split-shell.runtime",
        fixture.region_id,
        fixture.moved_face_id,
    );
    assert!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration)
            .contains(&crate::facade::TopologyEditFamily::AttachShellOrWireMembership)
    );
}
