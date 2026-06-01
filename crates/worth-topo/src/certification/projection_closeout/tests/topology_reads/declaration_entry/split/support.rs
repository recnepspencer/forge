use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;

use crate::facade::{
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration, TopologyWireSplitHalfEdgeMember,
};

pub(super) struct WireSplitFixture {
    pub(super) retained_wire_id: EntityId,
    pub(super) moved_half_edge_ids: Vec<EntityId>,
    pub(super) retained_half_edge_ids: Vec<EntityId>,
    pub(super) disconnected_half_edge_ids: Vec<EntityId>,
}

pub(super) struct ShellSplitFixture {
    pub(super) region_id: EntityId,
    pub(super) retained_shell_id: EntityId,
    pub(super) moved_face_id: EntityId,
    pub(super) retained_face_id: EntityId,
}

pub(super) fn wire_split_declaration() -> TopologySplitConnectedHalfEdgeSetToNewWireDeclaration {
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(
        "query-native.split-wire.handle.new-wire",
        vec![TopologyWireSplitHalfEdgeMember::new(
            "query-native.split-wire.handle.member-1",
            EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                8,
                1,
            ),
        )],
    )
}

pub(super) fn shell_split_declaration(
) -> TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration {
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::new(
        "query-native.split-shell.handle.new-shell",
        "query-native.split-shell.handle.region-member",
        "query-native.split-shell.handle.face-member",
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            2,
            1,
        ),
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            8,
            1,
        ),
    )
}

pub(super) fn wire_split_fixture(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> WireSplitFixture {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .expect("seeded snapshot should remain readable");
    let retained_wire_id = read_view
        .entities()
        .iter()
        .find(|record| {
            schema::facade::platform::entities::EntityKind::from_kind_id(record.kind.kind_id)
                == Some(schema::facade::platform::entities::EntityKind::Topology(
                    TopologyEntityKind::Wire,
                ))
        })
        .map(|record| record.entity_id)
        .into_iter()
        .next()
        .expect("seeded wire primitive should contain one wire");
    let half_edge_ids = read_view
        .relations()
        .iter()
        .filter(|record| {
            record.source == retained_wire_id
                && matches!(
                    schema::facade::platform::relations::RelationKind::from_kind_id(
                        record.kind.kind_id
                    ),
                    Some(schema::facade::platform::relations::RelationKind::Topology(
                        TopologyRelationKind::WireOwnsHalfEdge
                    ))
                )
        })
        .map(|record| record.target)
        .collect::<Vec<_>>();
    let mut vertices_by_half_edge = BTreeMap::new();
    for half_edge_id in &half_edge_ids {
        let mut vertices = BTreeSet::new();
        for relation_kind in [
            TopologyRelationKind::HalfEdgeStartsAtVertex,
            TopologyRelationKind::HalfEdgeEndsAtVertex,
        ] {
            vertices.extend(read_view.relations().iter().filter_map(|record| {
                (record.source == *half_edge_id
                    && matches!(
                        schema::facade::platform::relations::RelationKind::from_kind_id(
                            record.kind.kind_id
                        ),
                        Some(schema::facade::platform::relations::RelationKind::Topology(kind))
                            if kind == relation_kind
                    ))
                .then_some(record.target)
            }));
        }
        vertices_by_half_edge.insert(*half_edge_id, vertices);
    }
    let mut adjacency = BTreeMap::new();
    for half_edge_id in &half_edge_ids {
        let neighbors = half_edge_ids
            .iter()
            .copied()
            .filter(|candidate_id| candidate_id != half_edge_id)
            .filter(|candidate_id| {
                vertices_by_half_edge[half_edge_id]
                    .iter()
                    .any(|vertex_id| vertices_by_half_edge[candidate_id].contains(vertex_id))
            })
            .collect::<Vec<_>>();
        adjacency.insert(*half_edge_id, neighbors);
    }
    let start_id = adjacency
        .iter()
        .find(|(_, neighbors)| neighbors.len() == 1)
        .map(|(half_edge_id, _)| *half_edge_id)
        .expect("open wire should expose an endpoint half-edge");
    let mut ordered_half_edge_ids = Vec::with_capacity(half_edge_ids.len());
    let mut visited = BTreeSet::new();
    let mut previous_id = None;
    let mut current_id = start_id;
    loop {
        ordered_half_edge_ids.push(current_id);
        visited.insert(current_id);
        let next_id = adjacency[&current_id].iter().copied().find(|neighbor_id| {
            Some(*neighbor_id) != previous_id && !visited.contains(neighbor_id)
        });
        let Some(next_id) = next_id else {
            break;
        };
        previous_id = Some(current_id);
        current_id = next_id;
    }
    assert_eq!(ordered_half_edge_ids.len(), half_edge_ids.len());

    WireSplitFixture {
        retained_wire_id,
        moved_half_edge_ids: ordered_half_edge_ids[..2].to_vec(),
        retained_half_edge_ids: ordered_half_edge_ids[2..].to_vec(),
        disconnected_half_edge_ids: vec![ordered_half_edge_ids[0], ordered_half_edge_ids[2]],
    }
}

pub(super) fn shell_split_fixture(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> ShellSplitFixture {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .expect("seeded snapshot should remain readable");
    let entity_ids_for = |kind: TopologyEntityKind| {
        read_view
            .entities()
            .iter()
            .filter(|record| {
                schema::facade::platform::entities::EntityKind::from_kind_id(record.kind.kind_id)
                    == Some(schema::facade::platform::entities::EntityKind::Topology(
                        kind,
                    ))
            })
            .map(|record| record.entity_id)
            .collect::<Vec<_>>()
    };
    let mut face_ids = entity_ids_for(TopologyEntityKind::Face);
    face_ids.sort();

    ShellSplitFixture {
        region_id: entity_ids_for(TopologyEntityKind::Region)
            .into_iter()
            .next()
            .expect("seeded sheet patch should contain one region"),
        retained_shell_id: entity_ids_for(TopologyEntityKind::Shell)
            .into_iter()
            .next()
            .expect("seeded sheet patch should contain one shell"),
        moved_face_id: face_ids[0],
        retained_face_id: face_ids[1],
    }
}

pub(super) fn wire_split_declaration_for_fixture(
    label: &str,
    moved_half_edge_ids: &[EntityId],
) -> TopologySplitConnectedHalfEdgeSetToNewWireDeclaration {
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(
        format!("{label}.new-wire"),
        moved_half_edge_ids
            .iter()
            .enumerate()
            .map(|(index, half_edge_id)| {
                TopologyWireSplitHalfEdgeMember::new(
                    format!("{label}.member-{}", index + 1),
                    *half_edge_id,
                )
            })
            .collect(),
    )
}

pub(super) fn shell_split_declaration_for_fixture(
    label: &str,
    region_id: EntityId,
    face_id: EntityId,
) -> TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration {
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::new(
        format!("{label}.new-shell"),
        format!("{label}.region-member"),
        format!("{label}.face-member"),
        region_id,
        face_id,
    )
}
