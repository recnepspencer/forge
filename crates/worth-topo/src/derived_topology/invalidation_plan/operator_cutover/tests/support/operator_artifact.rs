use forge_relational::facade::identity::{EntityId, RelationId};
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{DerivedTopologyReadBasis, MilestoneOnePrimitiveCase};

use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::topology_operators::application::TopologyDeclaredMutationArtifact;
use crate::topology_operators::{
    TopologyLoopSuccessorRewireMember, TopologyRewireLoopSuccessorProgramDeclaration,
};

pub(in super::super) fn real_operator_artifact() -> TopologyDeclaredMutationArtifact {
    let mut runtime = crate::validation::reference_integrity::build_milestone_one_runtime()
        .expect("milestone one runtime");
    let verified =
        crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution(
            &mut runtime,
            "phase-seven.operator-cutover",
            &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
        )
        .expect("seeded sheet disk topology");
    let declaration = real_successor_relocation_declaration(&runtime, verified.read_basis());
    let adapters = crate::facade::TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        crate::facade::topology_runtime(adapters, "phase-seven.operator-cutover.real-artifact")
            .expect("topology workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declared query surfaces");

    execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
        .expect("real topology operator artifact")
}

fn real_successor_relocation_declaration(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> TopologyRewireLoopSuccessorProgramDeclaration {
    let read_view = runtime_snapshot(runtime, read_basis);
    let moved_half_edge_id = first_entity_id(&read_view, TopologyEntityKind::HalfEdge);
    let old_successor_id = outgoing_target_id(
        &read_view,
        moved_half_edge_id,
        TopologyRelationKind::HalfEdgeNext,
    );
    let intermediate_successor_id = outgoing_target_id(
        &read_view,
        old_successor_id,
        TopologyRelationKind::HalfEdgeNext,
    );
    let second_intermediate_id = outgoing_target_id(
        &read_view,
        intermediate_successor_id,
        TopologyRelationKind::HalfEdgeNext,
    );
    let new_successor_id = outgoing_target_id(
        &read_view,
        second_intermediate_id,
        TopologyRelationKind::HalfEdgeNext,
    );
    successor_relocation_declaration(&read_view, moved_half_edge_id, new_successor_id)
}

fn runtime_snapshot(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> RelationalReadView {
    runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .expect("seeded snapshot should remain readable")
}

fn first_entity_id(read_view: &RelationalReadView, kind: TopologyEntityKind) -> EntityId {
    read_view
        .entities()
        .iter()
        .find(|record| record.kind.kind_id == EntityKind::Topology(kind).kind_id())
        .map(|record| record.entity_id)
        .expect("seeded topology should expose requested entity")
}

fn successor_relocation_declaration(
    read_view: &RelationalReadView,
    moved_half_edge_id: EntityId,
    new_successor_id: EntityId,
) -> TopologyRewireLoopSuccessorProgramDeclaration {
    let old_successor_id = outgoing_target_id(
        read_view,
        moved_half_edge_id,
        TopologyRelationKind::HalfEdgeNext,
    );
    let old_predecessor_id = outgoing_target_id(
        read_view,
        moved_half_edge_id,
        TopologyRelationKind::HalfEdgePrev,
    );
    let new_predecessor_id = outgoing_target_id(
        read_view,
        new_successor_id,
        TopologyRelationKind::HalfEdgePrev,
    );
    TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        successor_rewire_member(
            read_view,
            moved_half_edge_id,
            TopologyRelationKind::HalfEdgeNext,
            crate::facade::LoopSuccessorKind::Next,
            new_successor_id,
        ),
        successor_rewire_member(
            read_view,
            moved_half_edge_id,
            TopologyRelationKind::HalfEdgePrev,
            crate::facade::LoopSuccessorKind::Prev,
            new_predecessor_id,
        ),
        successor_rewire_member(
            read_view,
            old_predecessor_id,
            TopologyRelationKind::HalfEdgeNext,
            crate::facade::LoopSuccessorKind::Next,
            old_successor_id,
        ),
        successor_rewire_member(
            read_view,
            old_successor_id,
            TopologyRelationKind::HalfEdgePrev,
            crate::facade::LoopSuccessorKind::Prev,
            old_predecessor_id,
        ),
        successor_rewire_member(
            read_view,
            new_predecessor_id,
            TopologyRelationKind::HalfEdgeNext,
            crate::facade::LoopSuccessorKind::Next,
            moved_half_edge_id,
        ),
        successor_rewire_member(
            read_view,
            new_successor_id,
            TopologyRelationKind::HalfEdgePrev,
            crate::facade::LoopSuccessorKind::Prev,
            moved_half_edge_id,
        ),
    ])
}

fn successor_rewire_member(
    read_view: &RelationalReadView,
    source_half_edge_id: EntityId,
    relation_kind: TopologyRelationKind,
    successor_kind: crate::facade::LoopSuccessorKind,
    successor_half_edge_id: EntityId,
) -> TopologyLoopSuccessorRewireMember {
    TopologyLoopSuccessorRewireMember::new(
        relation_id_for_source_kind(read_view, source_half_edge_id, relation_kind),
        successor_kind,
        source_half_edge_id,
        successor_half_edge_id,
    )
}

fn relation_id_for_source_kind(
    read_view: &RelationalReadView,
    source_id: EntityId,
    relation_kind: TopologyRelationKind,
) -> RelationId {
    read_view
        .relations()
        .iter()
        .find(|record| {
            record.source == source_id
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(relation_kind))
        })
        .map(|record| record.relation_id)
        .expect("relation id should exist")
}

fn outgoing_target_id(
    read_view: &RelationalReadView,
    source_id: EntityId,
    relation_kind: TopologyRelationKind,
) -> EntityId {
    read_view
        .relations()
        .iter()
        .find(|record| {
            record.source == source_id
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(relation_kind))
        })
        .map(|record| record.target)
        .expect("relation target should exist")
}
