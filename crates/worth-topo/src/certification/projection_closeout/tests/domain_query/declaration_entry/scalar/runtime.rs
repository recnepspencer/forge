use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive, seed_minimal_topology, MilestoneOnePrimitiveCase,
};

use super::super::super::support::execute_current_head_topology_declaration;
use super::support::{endpoint_rewire_fixture, radial_splice_fixture, seeded_relation_id};
use crate::facade::{
    topology_runtime, BoundaryMembershipKind, LoopEndpointKind, ShellOrWireMembershipKind,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologyRuntimeAdapters,
    TopologySpliceRadialAdjacencyDeclaration,
};
use crate::topology_operators::TopologyOperatorExecutionError;
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_remaining_scalar_batches_through_declaration_entry() {
    retire_runtime_case();
    detach_boundary_runtime_case();
    rewire_endpoint_runtime_case();
    detach_shell_or_wire_runtime_case();
    splice_radial_runtime_case();
    detach_radial_runtime_case();
}

#[test]
fn current_head_runtime_keeps_invalid_scalar_splice_on_typed_denial_boundary() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "query-native.scalar.splice-radial-denial",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.scalar.splice-radial-denial").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let (relation_id, half_edge_id, radial_next_half_edge_id) =
        radial_splice_fixture(&mut workspace, &surfaces);
    let error = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        TopologySpliceRadialAdjacencyDeclaration::new(
            relation_id,
            radial_next_half_edge_id,
            half_edge_id,
        ),
    )
    .expect_err("mismatched radial splice should fail typed and early");

    assert!(matches!(
        error,
        TopologyOperatorExecutionError::ExistingRelationSourceMismatch { .. }
    ));
}

fn retire_runtime_case() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-native.scalar.retire").expect("seed");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.scalar.retire").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        TopologyRetireTopologyEntityDeclaration::new(seeded.vertex, TopologyEntityKind::Vertex),
    )
    .expect("retire batch should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.retire_topology_entity"
    );
    assert!(!execution
        .materialized
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.entity_id == seeded.vertex));
}

fn detach_boundary_runtime_case() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "query-native.scalar.detach-boundary").expect("seed");
    let relation_id = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        TopologyRelationKind::LoopOwnsHalfEdge,
    );
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.scalar.detach-boundary").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        TopologyDetachBoundaryMembershipDeclaration::new(
            relation_id,
            BoundaryMembershipKind::LoopOwnsHalfEdge,
        ),
    )
    .expect("detach boundary batch should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.detach_boundary_membership"
    );
    let loop_record = execution
        .materialized
        .topology()
        .loops
        .iter()
        .find(|loop_record| loop_record.entity_id == seeded.outer_loop)
        .expect("seeded loop should remain present");
    assert!(loop_record.half_edge_ids.is_empty());
}

fn rewire_endpoint_runtime_case() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "query-native.scalar.rewire-endpoint",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.scalar.rewire-endpoint").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let (relation_id, half_edge_id, target_vertex_id) =
        endpoint_rewire_fixture(&mut workspace, &surfaces);
    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        TopologyRewireLoopEndpointDeclaration::new(
            relation_id,
            LoopEndpointKind::End,
            half_edge_id,
            target_vertex_id,
        ),
    )
    .expect("rewire endpoint batch should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.rewire_loop_endpoint"
    );
    let half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == half_edge_id)
        .expect("rewired half-edge should remain present");
    assert_eq!(half_edge.target_vertex_id, Some(target_vertex_id));
}

fn detach_shell_or_wire_runtime_case() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "query-native.scalar.detach-wire").expect("seed");
    let relation_id = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        TopologyRelationKind::WireOwnsHalfEdge,
    );
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.scalar.detach-wire").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        TopologyDetachShellOrWireMembershipDeclaration::new(
            relation_id,
            ShellOrWireMembershipKind::WireOwnsHalfEdge,
        ),
    )
    .expect("detach shell-or-wire batch should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.detach_shell_or_wire_membership"
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

fn splice_radial_runtime_case() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "query-native.scalar.splice-radial",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.scalar.splice-radial").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let (relation_id, half_edge_id, radial_next_half_edge_id) =
        radial_splice_fixture(&mut workspace, &surfaces);
    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        TopologySpliceRadialAdjacencyDeclaration::new(
            relation_id,
            half_edge_id,
            radial_next_half_edge_id,
        ),
    )
    .expect("splice radial batch should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.splice_radial_adjacency"
    );
    let half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == half_edge_id)
        .expect("rewired half-edge should remain present");
    assert_eq!(
        half_edge.radial_next_half_edge_id,
        Some(radial_next_half_edge_id)
    );
}

fn detach_radial_runtime_case() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded =
        seed_minimal_topology(&mut runtime, "query-native.scalar.detach-radial").expect("seed");
    let relation_id = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        TopologyRelationKind::HalfEdgeRadialNext,
    );
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.scalar.detach-radial").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        TopologyDetachRadialAdjacencyDeclaration::new(relation_id),
    )
    .expect("detach radial batch should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.detach_radial_adjacency"
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
