use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::super::support::{
    current_head_unsupported_declaration_families, execute_current_head_topology_declaration,
};
use super::successor_runtime_support::{
    cross_loop_successor_declaration, find_half_edge, single_successor_fixture,
    two_half_edge_span_fixture,
};
use crate::facade::{topology_runtime, TopologyOperatorExecutionPath, TopologyRuntimeAdapters};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_single_successor_program_batch_through_declaration_entry() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded = seed_milestone_one_primitive(
        &mut runtime,
        "query-native.successor.runtime",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
    )
    .expect("seed topology");
    let fixture = single_successor_fixture(&runtime, seeded.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.successor.runtime").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, fixture.declaration)
            .expect("single successor relocation should execute through declaration entry");

    assert_eq!(
        execution.path,
        TopologyOperatorExecutionPath::DeclarationEntry {
            semantic_family_key: "topology.rewire_loop_successor_program",
        }
    );
    let moved_half_edge = find_half_edge(&execution, fixture.moved_half_edge_id);
    assert_eq!(
        moved_half_edge.prev_half_edge_id,
        Some(fixture.new_predecessor_id)
    );
    assert_eq!(
        moved_half_edge.next_half_edge_id,
        Some(fixture.new_successor_id)
    );
    let old_predecessor = find_half_edge(&execution, fixture.old_predecessor_id);
    assert_eq!(
        old_predecessor.next_half_edge_id,
        Some(fixture.old_successor_id)
    );
}

#[test]
fn current_head_runtime_executes_two_half_edge_span_successor_program_through_declaration_entry() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded = seed_milestone_one_primitive(
        &mut runtime,
        "query-native.successor-span.runtime",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed topology");
    let fixture = two_half_edge_span_fixture(&runtime, seeded.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.successor-span.runtime").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, fixture.declaration)
            .expect("two-half-edge span relocation should execute through declaration entry");

    assert_eq!(
        execution.path,
        TopologyOperatorExecutionPath::DeclarationEntry {
            semantic_family_key: "topology.rewire_loop_successor_program",
        }
    );
    let moved_start = find_half_edge(&execution, fixture.moved_start_id);
    assert_eq!(
        moved_start.prev_half_edge_id,
        Some(fixture.new_predecessor_id)
    );
    assert_eq!(moved_start.next_half_edge_id, Some(fixture.moved_end_id));
    let moved_end = find_half_edge(&execution, fixture.moved_end_id);
    assert_eq!(moved_end.prev_half_edge_id, Some(fixture.moved_start_id));
    assert_eq!(moved_end.next_half_edge_id, Some(fixture.new_successor_id));
    let old_predecessor = find_half_edge(&execution, fixture.old_predecessor_id);
    assert_eq!(
        old_predecessor.next_half_edge_id,
        Some(fixture.old_successor_id)
    );
}

#[test]
fn current_head_runtime_rejects_cross_loop_successor_program_before_any_declaration_entry_execution(
) {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded = seed_milestone_one_primitive(
        &mut runtime,
        "query-native.successor-cross-loop.runtime",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let declaration = cross_loop_successor_declaration(&runtime, seeded.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.successor-cross-loop.runtime").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");

    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![crate::facade::TopologyEditFamily::RewireLoopSuccessor]
    );
}
