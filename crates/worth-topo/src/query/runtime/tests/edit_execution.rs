use forge_relational::facade::identity::RelationId;
use worth_schema::facade::{
    created_ref, seed_minimal_topology, WorthCreateKey, WorthRelationKind, WorthTopologyEntityKind,
    WorthTopologyRelationKind,
};

use crate::edit::{
    WorthBoundaryMembershipKind, WorthShellOrWireMembershipKind, WorthTopologyEditApplicationMode,
    WorthTopologyEditBatch, WorthTopologyEditContract, WorthTopologyEditFamily,
    WorthTopologyQueryEditRunner,
};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_create_topology_entity_through_query_native_edit_runner() {
    let runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit").expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::create_topology_entity(
            "worth-query-edit-runtime.added_vertex",
            WorthTopologyEntityKind::Vertex,
        )])
        .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("create topology entity should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::CreateTopologyEntity]
    );
    assert_eq!(execution.naming_report.rows.len(), 1);
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert!(execution
        .materialized
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.label == "worth-query-edit-runtime.added_vertex"));
}

#[test]
fn current_head_runtime_executes_retire_topology_entity_through_query_native_edit_runner() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-retire")
        .expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.current-head.query-edit-retire")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::retire_topology_entity(
            seeded.vertex,
            WorthTopologyEntityKind::Vertex,
        )])
        .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("retire topology entity should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::RetireTopologyEntity]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert!(!execution
        .materialized
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.entity_id == seeded.vertex));
}

#[test]
fn current_head_runtime_executes_detach_boundary_membership_through_query_native_edit_runner() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-detach")
        .expect("seed topology");
    let loop_owns_half_edge_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::LoopOwnsHalfEdge,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(adapters, "worth.current-head.query-edit-detach")
        .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_boundary_membership(
            loop_owns_half_edge_relation,
            WorthBoundaryMembershipKind::LoopOwnsHalfEdge,
        )])
        .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("detach boundary membership should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachBoundaryMembership]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    let loop_record = execution
        .materialized
        .topology()
        .loops
        .iter()
        .find(|loop_record| loop_record.entity_id == seeded.outer_loop)
        .expect("seeded outer loop should remain present");
    assert!(loop_record.half_edge_ids.is_empty());
}

#[test]
fn current_head_runtime_denies_create_inner_loop_on_existing_face_workflow_until_invariant_complete_subgraphs_are_admitted(
) {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-attach-boundary")
        .expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-attach-boundary")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let loop_key = WorthCreateKey::new("worth-query-edit-runtime-attach-boundary.inner_loop");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            loop_key.as_str(),
            WorthTopologyEntityKind::Loop,
        ),
        WorthTopologyEditContract::attach_boundary_membership(
            "worth-query-edit-runtime-attach-boundary.face-inner-loop",
            WorthBoundaryMembershipKind::FaceInnerLoop,
            seeded.face,
            created_ref(loop_key.as_str()),
        ),
    ])
    .expect("non-empty edit batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("create-inner-loop workflow must fail closed until the production runtime admits an invariant-complete subgraph");

    assert!(matches!(
        error,
        crate::edit::WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachBoundaryMembership]
    ));
}

#[test]
fn current_head_runtime_executes_detach_radial_adjacency_through_query_native_edit_runner() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-detach-radial")
        .expect("seed topology");
    let radial_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::HalfEdgeRadialNext,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-detach-radial")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch =
        WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::detach_radial_adjacency(
            radial_relation,
        )])
        .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("detach radial adjacency should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachRadialAdjacency]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    let half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == seeded.half_edge)
        .expect("seeded half-edge should remain present");
    assert_eq!(half_edge.radial_next_half_edge_id, None);
}

#[test]
fn current_head_runtime_executes_detach_shell_or_wire_membership_through_query_native_edit_runner()
{
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-detach-wire")
        .expect("seed topology");
    let wire_owns_half_edge_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        WorthTopologyRelationKind::WireOwnsHalfEdge,
    );
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-detach-wire")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::detach_shell_or_wire_membership(
            wire_owns_half_edge_relation,
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
        ),
    ])
    .expect("non-empty edit batch");

    let execution = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("detach shell-or-wire membership should execute through query runtime");

    assert_eq!(
        execution.families,
        vec![WorthTopologyEditFamily::DetachShellOrWireMembership]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&assembly.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    let wire = execution
        .materialized
        .topology()
        .wires
        .iter()
        .find(|wire| wire.entity_id == seeded.wire)
        .expect("seeded wire should remain present");
    assert!(wire.half_edge_ids.is_empty());
}

#[test]
fn current_head_runtime_denies_attach_shell_or_wire_membership_until_invariant_complete_subgraphs_are_admitted(
) {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    let seeded = seed_minimal_topology(&mut runtime, "worth-query-edit-runtime-attach-wire")
        .expect("seed topology");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "worth.current-head.query-edit-attach-wire")
            .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let shell_key = WorthCreateKey::new("worth-query-edit-runtime-attach-wire.inner_shell");
    let batch = WorthTopologyEditBatch::new(vec![
        WorthTopologyEditContract::create_topology_entity(
            shell_key.as_str(),
            WorthTopologyEntityKind::Shell,
        ),
        WorthTopologyEditContract::attach_shell_or_wire_membership(
            "worth-query-edit-runtime-attach-wire.region-owns-shell",
            WorthShellOrWireMembershipKind::RegionOwnsShell,
            seeded.region,
            created_ref(shell_key.as_str()),
        ),
    ])
    .expect("non-empty edit batch");

    let error = WorthTopologyQueryEditRunner::new(&mut workspace, &assembly)
        .apply(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("attach shell-or-wire membership must fail closed until invariant-complete topology subgraphs are admitted");

    assert!(matches!(
        error,
        crate::edit::WorthTopologyQueryEditExecutionError::UnsupportedFamilies(families)
            if families == vec![WorthTopologyEditFamily::AttachShellOrWireMembership]
    ));
}

fn seeded_relation_id(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    snapshot: &forge_relational::facade::snapshots::SnapshotHandle,
    kind: WorthTopologyRelationKind,
) -> RelationId {
    runtime
        .read_truth()
        .read_snapshot(snapshot)
        .expect("seeded snapshot should remain readable")
        .relations()
        .iter()
        .find(|record| record.kind.kind_id == WorthRelationKind::Topology(kind).kind_id())
        .map(|record| record.relation_id)
        .expect("seeded topology should contain requested relation kind")
}
