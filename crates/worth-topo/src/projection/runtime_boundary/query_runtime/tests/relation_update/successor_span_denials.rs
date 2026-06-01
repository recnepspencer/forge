use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::super::query_runtime_support::QueryRuntimeSupport;
use super::successor_span_declaration::{
    successor_span_relocation_declaration, two_half_edge_span_relocation_declaration,
};
use crate::certification::support::declaration_runtime::current_head_unsupported_declaration_families;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::TopologyMutationFamily;
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_denies_cross_loop_two_half_edge_span_relocation_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-mutation-rewire-successor-span-cross-loop",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-mutation-rewire-successor-span-cross-loop",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let support = QueryRuntimeSupport::load(&mut workspace, &surfaces);
    let (moved_start_identity, new_successor_identity) =
        support.half_edge_identities_for_different_loops();
    let declaration = two_half_edge_span_relocation_declaration(
        &mut workspace,
        &support,
        &moved_start_identity,
        &new_successor_identity,
    );
    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyMutationFamily::RewireLoopSuccessor]
    );
}

#[test]
fn current_head_runtime_denies_degenerate_two_half_edge_span_relocation_before_current_successor() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-mutation-rewire-successor-span-degenerate",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-mutation-rewire-successor-span-degenerate",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let support = QueryRuntimeSupport::load(&mut workspace, &surfaces);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        schema::facade::platform::relations::TopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 3);
    let old_successor_identity = cycle[2].as_str();
    let declaration = two_half_edge_span_relocation_declaration(
        &mut workspace,
        &support,
        &moved_start_identity,
        old_successor_identity,
    );
    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyMutationFamily::RewireLoopSuccessor]
    );
}

#[test]
fn current_head_runtime_denies_three_half_edge_span_relocation_before_internal_member() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-mutation-rewire-successor-three-span-internal",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-mutation-rewire-successor-three-span-internal",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let support = QueryRuntimeSupport::load(&mut workspace, &surfaces);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        schema::facade::platform::relations::TopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 3);
    let internal_successor_identity = cycle[1].as_str();
    let declaration = successor_span_relocation_declaration(
        &mut workspace,
        &support,
        &moved_start_identity,
        internal_successor_identity,
        3,
    );

    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![TopologyMutationFamily::RewireLoopSuccessor]
    );
}
