use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::super::query_runtime_support::QueryRuntimeSupport;
use super::span_batch::{successor_span_relocation_batch, two_half_edge_span_relocation_batch};
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    TopologyEditApplicationMode, TopologyEditFamily, TopologyOperatorExecutionError,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_denies_cross_loop_two_half_edge_span_relocation_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-edit-rewire-successor-span-cross-loop",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-edit-rewire-successor-span-cross-loop",
    )
    .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = QueryRuntimeSupport::load(&mut workspace, &assembly);
    let (moved_start_identity, new_successor_identity) =
        support.half_edge_identities_for_different_loops();
    let batch = two_half_edge_span_relocation_batch(
        &mut workspace,
        &support,
        &moved_start_identity,
        &new_successor_identity,
    );

    let error = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect_err("cross-loop two-halfedge span relocation must fail closed");

    assert!(matches!(
        error,
        TopologyOperatorExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == TopologyEditFamily::RewireLoopSuccessor)
    ));
}

#[test]
fn current_head_runtime_denies_degenerate_two_half_edge_span_relocation_before_current_successor() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-edit-rewire-successor-span-degenerate",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-edit-rewire-successor-span-degenerate",
    )
    .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = QueryRuntimeSupport::load(&mut workspace, &assembly);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        schema::facade::TopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 3);
    let old_successor_identity = cycle[2].as_str();
    let batch = two_half_edge_span_relocation_batch(
        &mut workspace,
        &support,
        &moved_start_identity,
        old_successor_identity,
    );

    let error = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect_err("degenerate same-loop two-halfedge span relocation must fail closed");

    assert!(matches!(
        error,
        TopologyOperatorExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == TopologyEditFamily::RewireLoopSuccessor)
    ));
}

#[test]
fn current_head_runtime_denies_three_half_edge_span_relocation_before_internal_member() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-edit-rewire-successor-three-span-internal",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 6 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-edit-rewire-successor-three-span-internal",
    )
    .expect("workspace");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = QueryRuntimeSupport::load(&mut workspace, &assembly);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        schema::facade::TopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 3);
    let internal_successor_identity = cycle[1].as_str();
    let batch = successor_span_relocation_batch(
        &mut workspace,
        &support,
        &moved_start_identity,
        internal_successor_identity,
        3,
    );

    let error = assembly
        .apply_edit(&mut workspace, batch, TopologyEditApplicationMode::Mainline)
        .expect_err("span relocation before an internal member must fail closed");

    assert!(matches!(
        error,
        TopologyOperatorExecutionError::UnsupportedFamilies(families)
            if families.iter().all(|family| *family == TopologyEditFamily::RewireLoopSuccessor)
    ));
}
