use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::facade::{
    BoundaryMembershipKind, LoopEndpointKind, ShellOrWireMembershipKind,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopEndpointDeclaration, TopologySpliceRadialAdjacencyDeclaration,
};
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::{query_entity_id_from_row, query_relation_id_from_row};
use forge_query::facade::ForgeQueryWorkspace;

pub(super) fn retire_declaration() -> TopologyRetireTopologyEntityDeclaration {
    TopologyRetireTopologyEntityDeclaration::new(
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            7,
            1,
        ),
        TopologyEntityKind::Vertex,
    )
}

pub(super) fn detach_boundary_declaration() -> TopologyDetachBoundaryMembershipDeclaration {
    TopologyDetachBoundaryMembershipDeclaration::new(
        RelationId::new(
            forge_relational::facade::identity::PartitionId::main(),
            7,
            1,
        ),
        BoundaryMembershipKind::LoopOwnsHalfEdge,
    )
}

pub(super) fn rewire_endpoint_declaration() -> TopologyRewireLoopEndpointDeclaration {
    TopologyRewireLoopEndpointDeclaration::new(
        RelationId::new(
            forge_relational::facade::identity::PartitionId::main(),
            7,
            1,
        ),
        LoopEndpointKind::End,
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            8,
            1,
        ),
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            9,
            1,
        ),
    )
}

pub(super) fn detach_shell_or_wire_declaration() -> TopologyDetachShellOrWireMembershipDeclaration {
    TopologyDetachShellOrWireMembershipDeclaration::new(
        RelationId::new(
            forge_relational::facade::identity::PartitionId::main(),
            7,
            1,
        ),
        ShellOrWireMembershipKind::WireOwnsHalfEdge,
    )
}

pub(super) fn splice_radial_declaration() -> TopologySpliceRadialAdjacencyDeclaration {
    TopologySpliceRadialAdjacencyDeclaration::new(
        RelationId::new(
            forge_relational::facade::identity::PartitionId::main(),
            7,
            1,
        ),
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            8,
            1,
        ),
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            9,
            1,
        ),
    )
}

pub(super) fn detach_radial_declaration() -> TopologyDetachRadialAdjacencyDeclaration {
    TopologyDetachRadialAdjacencyDeclaration::new(RelationId::new(
        forge_relational::facade::identity::PartitionId::main(),
        7,
        1,
    ))
}

pub(super) fn seeded_relation_id(
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

pub(super) fn endpoint_rewire_fixture(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> (RelationId, EntityId, EntityId) {
    let relation_rows = workspace.read(surfaces.relations());
    let entity_rows = workspace.read(surfaces.entities());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name == TopologyRelationKind::HalfEdgeEndsAtVertex.kind_name()
                })
        })
        .expect("seeded topology should contain an endpoint relation");
    let current_target_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
        .expect("endpoint relation should expose topology.target_identity");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("endpoint relation should expose topology.source_identity");
    let target_vertex_id = entity_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == ".vertex")
                && row.identity != current_target_identity
        })
        .map(|row| query_entity_id_from_row(row).expect("entity id should decode"))
        .expect("seeded disk should provide an alternate vertex");
    let half_edge_id = entity_rows
        .iter()
        .find(|row| row.identity == source_identity)
        .map(|row| query_entity_id_from_row(row).expect("entity id should decode"))
        .expect("endpoint source identity should resolve to a half-edge");
    (
        query_relation_id_from_row(relation).expect("relation id should decode"),
        half_edge_id,
        target_vertex_id,
    )
}

pub(super) fn radial_splice_fixture(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> (RelationId, EntityId, EntityId) {
    let domain_query = TopologyReadProofHarness::new();
    let relation_rows = workspace.read(surfaces.relations());
    let entity_rows = workspace.read(surfaces.entities());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name == TopologyRelationKind::HalfEdgeRadialNext.kind_name()
                })
        })
        .expect("seeded topology should contain a radial relation");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("radial relation should expose topology.source_identity");
    let current_target_identity = domain_query
        .radial_half_edge_neighborhood(workspace, source_identity)
        .expect("seeded topology should expose radial neighborhood")
        .current_target_half_edge_identity;
    let alternate_identity = domain_query
        .radial_half_edge_neighborhood(workspace, source_identity)
        .expect("seeded topology should expose radial neighborhood")
        .same_edge_half_edge_identities
        .into_iter()
        .find(|identity| identity != source_identity && identity != &current_target_identity)
        .expect("seeded edge fan should provide an alternate same-edge half-edge");
    let half_edge_id = entity_rows
        .iter()
        .find(|row| row.identity == source_identity)
        .map(|row| query_entity_id_from_row(row).expect("entity id should decode"))
        .expect("source identity should resolve to a half-edge");
    let alternate_half_edge_id = entity_rows
        .iter()
        .find(|row| row.identity == alternate_identity)
        .map(|row| query_entity_id_from_row(row).expect("entity id should decode"))
        .expect("alternate identity should resolve to a half-edge");
    (
        query_relation_id_from_row(relation).expect("relation id should decode"),
        half_edge_id,
        alternate_half_edge_id,
    )
}
