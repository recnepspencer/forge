use forge_relational::facade::identity::RelationId;
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{
<<<<<<< HEAD:crates/worth-topo/src/projection/runtime_boundary/query_runtime/tests/edit_execution/core.rs
    created_ref, seed_milestone_one_primitive, seed_minimal_topology, MilestoneOnePrimitiveCase,
=======
    seed_milestone_one_primitive, seed_minimal_topology, MilestoneOnePrimitiveCase,
>>>>>>> origin/master:crates/worth-topo/src/projection/runtime_boundary/query_runtime/tests/mutation_application/core.rs
};

use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyCreateInnerLoopOnExistingFaceDeclaration,
    TopologyCreateTopologyEntityDeclaration, TopologyDetachBoundaryMembershipDeclaration,
    TopologyDetachRadialAdjacencyDeclaration, TopologyMutationFamily,
    TopologyMutationNamingOutcome,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{
    ForgeQueryExistingTruthAssertionMode, ForgeQueryGraphCompositionProgramStepKind,
};

#[test]
fn current_head_runtime_executes_create_topology_entity_through_topology_mutation_application() {
    let runtime = build_milestone_one_runtime().expect(" runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-mutation").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "query-mutation-runtime.added_vertex",
        TopologyEntityKind::Vertex,
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("create topology entity should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![TopologyMutationFamily::CreateTopologyEntity]
    );
    assert_eq!(execution.topology_mutation_digest.mutation_record_count, 1);
    assert_eq!(execution.topology_mutation_digest.family_count, 1);
    assert_eq!(execution.naming_continuity_matrix.rows.len(), 1);
    assert_eq!(execution.naming_continuity_matrix.preserved_count, 1);
    assert_eq!(execution.naming_continuity_matrix.ambiguous_count, 0);
    assert_eq!(execution.naming_continuity_matrix.rejected_count, 0);
    assert_eq!(
        execution.naming_continuity_matrix.rows[0].outcome,
        TopologyMutationNamingOutcome::Preserved
    );
    assert_eq!(execution.naming_report.rows.len(), 1);
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&surfaces.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert!(execution
        .materialized
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.label == "query-mutation-runtime.added_vertex"));
}

#[test]
fn current_head_runtime_executes_retire_topology_entity_through_topology_mutation_application() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-mutation-runtime-retire")
        .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-mutation-retire").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = crate::topology_operators::TopologyRetireTopologyEntityDeclaration::new(
        seeded.vertex,
        TopologyEntityKind::Vertex,
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("retire topology entity should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![TopologyMutationFamily::RetireTopologyEntity]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&surfaces.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert_eq!(
        execution.inspection.component_operations()[0]
            .existing_truth_assertion_evidence()
            .expect("retire receipt should retain backend verification evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(
        execution
            .mutation_evidence()
            .backend_verified_delete_count(),
        1
    );
    assert!(!execution
        .materialized
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.entity_id == seeded.vertex));
}

#[test]
fn current_head_runtime_executes_detach_boundary_membership_through_topology_mutation_application()
{
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-mutation-runtime-detach")
        .expect("seed topology");
    let loop_owns_half_edge_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        TopologyRelationKind::LoopOwnsHalfEdge,
    );
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-mutation-detach").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyDetachBoundaryMembershipDeclaration::new(
        loop_owns_half_edge_relation,
        BoundaryMembershipKind::LoopOwnsHalfEdge,
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("detach boundary membership should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![TopologyMutationFamily::DetachBoundaryMembership]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&surfaces.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert_eq!(
        execution.inspection.component_operations()[0]
            .existing_truth_assertion_evidence()
            .expect("detach receipt should retain backend verification evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(
        execution
            .mutation_evidence()
            .backend_verified_delete_count(),
        1
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
fn current_head_runtime_executes_create_inner_loop_on_existing_face_program() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-mutation-runtime-attach-boundary",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("seed topology");
    let face_id = runtime
        .read_truth()
        .read_snapshot(verified.read_basis().snapshot())
        .expect("seeded snapshot should remain readable")
        .entities()
        .iter()
        .find(|record| {
            record.kind.kind_id == EntityKind::Topology(TopologyEntityKind::Face).kind_id()
        })
        .map(|record| record.entity_id)
        .expect("seeded primitive should contain a face");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.query-mutation-attach-boundary")
        .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let loop_key = CreateKey::new("query-mutation-runtime-attach-boundary.inner_loop");
    let declaration = TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
        loop_key.as_str(),
        "query-mutation-runtime-attach-boundary.face-inner-loop",
        face_id,
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("create-inner-loop program should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![
            TopologyMutationFamily::CreateTopologyEntity,
            TopologyMutationFamily::AttachBoundaryMembership,
        ]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&surfaces.materialized().name().to_string()));
    assert_eq!(
        execution
            .receipt
            .graph_composition_program()
            .expect("inner-loop program should expose composed program")
            .steps()
            .iter()
            .map(|step| step.kind())
            .collect::<Vec<_>>(),
        vec![
            ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration,
            ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration,
        ]
    );
    let face = execution
        .materialized
        .topology()
        .faces
        .iter()
        .find(|face| face.entity_id == face_id)
        .expect("seeded face should remain present");
    assert!(
        !face.inner_loop_ids.is_empty(),
        "face should gain an inner loop after admitted program"
    );
}

#[test]
fn current_head_runtime_executes_detach_radial_adjacency_through_topology_mutation_application() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-mutation-runtime-detach-radial")
        .expect("seed topology");
    let radial_relation = seeded_relation_id(
        &runtime,
        &seeded.snapshot,
        TopologyRelationKind::HalfEdgeRadialNext,
    );
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.query-mutation-detach-radial")
        .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyDetachRadialAdjacencyDeclaration::new(radial_relation);
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("detach radial adjacency should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![TopologyMutationFamily::DetachRadialAdjacency]
    );
    assert!(execution
        .receipt
        .affected_derived_view_ids()
        .contains(&surfaces.materialized().name().to_string()));
    assert_eq!(
        execution.inspection.affected_derived_view_ids(),
        execution.receipt.affected_derived_view_ids()
    );
    assert_eq!(
        execution.inspection.component_operations()[0]
            .existing_truth_assertion_evidence()
            .expect("detach receipt should retain backend verification evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    assert_eq!(
        execution
            .mutation_evidence()
            .backend_verified_delete_count(),
        1
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

fn seeded_relation_id(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    snapshot: &forge_relational::facade::snapshots::SnapshotHandle,
    kind: TopologyRelationKind,
) -> RelationId {
    runtime
        .read_truth()
        .read_snapshot(snapshot)
        .expect("seeded snapshot should remain readable")
        .relations()
        .iter()
        .find(|record| record.kind.kind_id == RelationKind::Topology(kind).kind_id())
        .map(|record| record.relation_id)
        .expect("seeded topology should contain requested relation kind")
}
