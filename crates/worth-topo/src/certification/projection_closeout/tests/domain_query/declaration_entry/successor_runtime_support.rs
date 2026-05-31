use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::facade::LoopSuccessorKind;
use crate::topology_operators::application::TopologyDeclaredMutationArtifact;
use crate::topology_operators::{
    TopologyLoopSuccessorRewireMember, TopologyRewireLoopSuccessorProgramDeclaration,
};

pub(super) struct SingleSuccessorFixture {
    pub(super) declaration: TopologyRewireLoopSuccessorProgramDeclaration,
    pub(super) moved_half_edge_id: EntityId,
    pub(super) old_predecessor_id: EntityId,
    pub(super) old_successor_id: EntityId,
    pub(super) new_predecessor_id: EntityId,
    pub(super) new_successor_id: EntityId,
}

pub(super) struct TwoHalfEdgeSpanFixture {
    pub(super) declaration: TopologyRewireLoopSuccessorProgramDeclaration,
    pub(super) moved_start_id: EntityId,
    pub(super) moved_end_id: EntityId,
    pub(super) old_predecessor_id: EntityId,
    pub(super) old_successor_id: EntityId,
    pub(super) new_predecessor_id: EntityId,
    pub(super) new_successor_id: EntityId,
}

pub(super) fn single_successor_fixture(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> SingleSuccessorFixture {
    let read_view = runtime_snapshot(runtime, read_basis);
    let moved_half_edge_id = read_view
        .entities()
        .iter()
        .find(|record| {
            schema::facade::platform::entities::EntityKind::from_kind_id(record.kind.kind_id)
                == Some(schema::facade::platform::entities::EntityKind::Topology(
                    TopologyEntityKind::HalfEdge,
                ))
        })
        .map(|record| record.entity_id)
        .expect("seeded topology should expose a half-edge");
    let old_successor_id = outgoing_target_id(
        &read_view,
        moved_half_edge_id,
        TopologyRelationKind::HalfEdgeNext,
    );
    let old_predecessor_id = outgoing_target_id(
        &read_view,
        moved_half_edge_id,
        TopologyRelationKind::HalfEdgePrev,
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
    let new_predecessor_id = outgoing_target_id(
        &read_view,
        new_successor_id,
        TopologyRelationKind::HalfEdgePrev,
    );
    SingleSuccessorFixture {
        declaration: successor_relocation_declaration(
            &read_view,
            moved_half_edge_id,
            new_successor_id,
        ),
        moved_half_edge_id,
        old_predecessor_id,
        old_successor_id,
        new_predecessor_id,
        new_successor_id,
    }
}

pub(super) fn two_half_edge_span_fixture(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> TwoHalfEdgeSpanFixture {
    let read_view = runtime_snapshot(runtime, read_basis);
    let moved_start_id = first_loop_half_edge_id(&read_view);
    let cycle = successor_cycle_ids(&read_view, moved_start_id, 5);
    let moved_end_id = cycle[1];
    let old_successor_id = cycle[2];
    let new_predecessor_id = cycle[3];
    let new_successor_id = cycle[4];
    let old_predecessor_id = outgoing_target_id(
        &read_view,
        moved_start_id,
        TopologyRelationKind::HalfEdgePrev,
    );
    TwoHalfEdgeSpanFixture {
        declaration: two_half_edge_span_relocation_declaration(
            &read_view,
            moved_start_id,
            new_successor_id,
        ),
        moved_start_id,
        moved_end_id,
        old_predecessor_id,
        old_successor_id,
        new_predecessor_id,
        new_successor_id,
    }
}

pub(super) fn cross_loop_successor_declaration(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> TopologyRewireLoopSuccessorProgramDeclaration {
    let read_view = runtime_snapshot(runtime, read_basis);
    let loops = read_view
        .entities()
        .iter()
        .filter(|record| {
            schema::facade::platform::entities::EntityKind::from_kind_id(record.kind.kind_id)
                == Some(schema::facade::platform::entities::EntityKind::Topology(
                    TopologyEntityKind::Loop,
                ))
        })
        .map(|record| record.entity_id)
        .collect::<Vec<_>>();
    let moved_half_edge_id = first_loop_member_half_edge_id(
        &read_view,
        loops[0],
        TopologyRelationKind::LoopOwnsHalfEdge,
    );
    let foreign_successor_id = first_loop_member_half_edge_id(
        &read_view,
        loops[1],
        TopologyRelationKind::LoopOwnsHalfEdge,
    );
    successor_relocation_declaration(&read_view, moved_half_edge_id, foreign_successor_id)
}

pub(super) fn find_half_edge(
    execution: &TopologyDeclaredMutationArtifact,
    half_edge_id: EntityId,
) -> &crate::facade::TopologyHalfEdge {
    execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == half_edge_id)
        .expect("half-edge should remain present")
}

fn runtime_snapshot<'a>(
    runtime: &'a RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> forge_relational::facade::runtime::RelationalReadView {
    runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .expect("seeded snapshot should remain readable")
}

fn successor_relocation_declaration(
    read_view: &forge_relational::facade::runtime::RelationalReadView,
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
            LoopSuccessorKind::Next,
            new_successor_id,
        ),
        successor_rewire_member(
            read_view,
            moved_half_edge_id,
            TopologyRelationKind::HalfEdgePrev,
            LoopSuccessorKind::Prev,
            new_predecessor_id,
        ),
        successor_rewire_member(
            read_view,
            old_predecessor_id,
            TopologyRelationKind::HalfEdgeNext,
            LoopSuccessorKind::Next,
            old_successor_id,
        ),
        successor_rewire_member(
            read_view,
            old_successor_id,
            TopologyRelationKind::HalfEdgePrev,
            LoopSuccessorKind::Prev,
            old_predecessor_id,
        ),
        successor_rewire_member(
            read_view,
            new_predecessor_id,
            TopologyRelationKind::HalfEdgeNext,
            LoopSuccessorKind::Next,
            moved_half_edge_id,
        ),
        successor_rewire_member(
            read_view,
            new_successor_id,
            TopologyRelationKind::HalfEdgePrev,
            LoopSuccessorKind::Prev,
            moved_half_edge_id,
        ),
    ])
}

fn two_half_edge_span_relocation_declaration(
    read_view: &forge_relational::facade::runtime::RelationalReadView,
    moved_start_id: EntityId,
    new_successor_id: EntityId,
) -> TopologyRewireLoopSuccessorProgramDeclaration {
    let moved_end_id = outgoing_target_id(
        read_view,
        moved_start_id,
        TopologyRelationKind::HalfEdgeNext,
    );
    let old_predecessor_id = outgoing_target_id(
        read_view,
        moved_start_id,
        TopologyRelationKind::HalfEdgePrev,
    );
    let old_successor_id =
        outgoing_target_id(read_view, moved_end_id, TopologyRelationKind::HalfEdgeNext);
    let new_predecessor_id = outgoing_target_id(
        read_view,
        new_successor_id,
        TopologyRelationKind::HalfEdgePrev,
    );
    TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        successor_rewire_member(
            read_view,
            moved_start_id,
            TopologyRelationKind::HalfEdgePrev,
            LoopSuccessorKind::Prev,
            new_predecessor_id,
        ),
        successor_rewire_member(
            read_view,
            moved_end_id,
            TopologyRelationKind::HalfEdgeNext,
            LoopSuccessorKind::Next,
            new_successor_id,
        ),
        successor_rewire_member(
            read_view,
            old_predecessor_id,
            TopologyRelationKind::HalfEdgeNext,
            LoopSuccessorKind::Next,
            old_successor_id,
        ),
        successor_rewire_member(
            read_view,
            old_successor_id,
            TopologyRelationKind::HalfEdgePrev,
            LoopSuccessorKind::Prev,
            old_predecessor_id,
        ),
        successor_rewire_member(
            read_view,
            new_predecessor_id,
            TopologyRelationKind::HalfEdgeNext,
            LoopSuccessorKind::Next,
            moved_start_id,
        ),
        successor_rewire_member(
            read_view,
            new_successor_id,
            TopologyRelationKind::HalfEdgePrev,
            LoopSuccessorKind::Prev,
            moved_end_id,
        ),
    ])
}

fn successor_rewire_member(
    read_view: &forge_relational::facade::runtime::RelationalReadView,
    source_half_edge_id: EntityId,
    relation_kind: TopologyRelationKind,
    successor_kind: LoopSuccessorKind,
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
    read_view: &forge_relational::facade::runtime::RelationalReadView,
    source_id: EntityId,
    relation_kind: TopologyRelationKind,
) -> forge_relational::facade::identity::RelationId {
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
    read_view: &forge_relational::facade::runtime::RelationalReadView,
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

fn first_loop_half_edge_id(
    read_view: &forge_relational::facade::runtime::RelationalReadView,
) -> EntityId {
    let loop_id = read_view
        .entities()
        .iter()
        .find(|record| {
            schema::facade::platform::entities::EntityKind::from_kind_id(record.kind.kind_id)
                == Some(schema::facade::platform::entities::EntityKind::Topology(
                    TopologyEntityKind::Loop,
                ))
        })
        .map(|record| record.entity_id)
        .expect("seeded topology should contain a loop");
    first_loop_member_half_edge_id(read_view, loop_id, TopologyRelationKind::LoopOwnsHalfEdge)
}

fn first_loop_member_half_edge_id(
    read_view: &forge_relational::facade::runtime::RelationalReadView,
    loop_id: EntityId,
    relation_kind: TopologyRelationKind,
) -> EntityId {
    read_view
        .relations()
        .iter()
        .find(|record| {
            record.source == loop_id
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(relation_kind))
        })
        .map(|record| record.target)
        .expect("loop should own a half-edge")
}

fn successor_cycle_ids(
    read_view: &forge_relational::facade::runtime::RelationalReadView,
    start_id: EntityId,
    count: usize,
) -> Vec<EntityId> {
    let mut current_id = start_id;
    let mut ids = Vec::with_capacity(count);
    for _ in 0..count {
        ids.push(current_id);
        current_id = outgoing_target_id(read_view, current_id, TopologyRelationKind::HalfEdgeNext);
    }
    ids
}
