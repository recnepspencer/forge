use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

pub(super) fn query_relation_id_from_row(
    row: &forge_query::facade::ForgeQueryEntity,
) -> RelationId {
    serde_json::from_value(row.payload["lineage"]["provenance"].clone())
        .expect("query relation provenance should decode")
}

pub(super) fn query_entity_id_from_row(row: &forge_query::facade::ForgeQueryEntity) -> EntityId {
    serde_json::from_value(row.payload["lineage"]["provenance"].clone())
        .expect("query entity provenance should decode")
}

pub(super) fn find_entity_id_by_identity(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    identity: &str,
) -> EntityId {
    entity_rows
        .iter()
        .find(|row| row.identity == identity)
        .map(query_entity_id_from_row)
        .expect("query identity should resolve to one entity")
}

pub(super) fn alternate_same_edge_half_edge_id(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    source_identity: &str,
    current_target_identity: &str,
) -> EntityId {
    let source_edge_identity = outgoing_target_identity(
        relation_rows,
        source_identity,
        WorthTopologyRelationKind::HalfEdgeUsesEdge,
    );
    entity_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == WorthTopologyEntityKind::HalfEdge.kind_name())
                && row.identity != source_identity
                && row.identity != current_target_identity
                && outgoing_target_identity(
                    relation_rows,
                    row.identity.as_str(),
                    WorthTopologyRelationKind::HalfEdgeUsesEdge,
                ) == source_edge_identity
        })
        .map(query_entity_id_from_row)
        .expect("seeded edge fan should provide an alternate halfedge on the same edge")
}

pub(super) fn different_edge_half_edge_id(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    source_identity: &str,
) -> EntityId {
    let source_edge_identity = outgoing_target_identity(
        relation_rows,
        source_identity,
        WorthTopologyRelationKind::HalfEdgeUsesEdge,
    );
    entity_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == WorthTopologyEntityKind::HalfEdge.kind_name())
                && row.identity != source_identity
                && outgoing_target_identity(
                    relation_rows,
                    row.identity.as_str(),
                    WorthTopologyRelationKind::HalfEdgeUsesEdge,
                ) != source_edge_identity
        })
        .map(query_entity_id_from_row)
        .expect("seeded edge fan should provide a halfedge on a different edge")
}

pub(super) fn relation_id_for_source_kind(
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    source_identity: &str,
    relation_kind: WorthTopologyRelationKind,
) -> RelationId {
    relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == relation_kind.kind_name())
                && row
                    .payload
                    .get("topology")
                    .and_then(|value| value.get("source_identity"))
                    .and_then(|value| value.as_str())
                    .is_some_and(|value| value == source_identity)
        })
        .map(query_relation_id_from_row)
        .expect("seeded topology should expose requested source/kind relation")
}

pub(super) fn next_target_half_edge_id(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    source_identity: &str,
) -> EntityId {
    find_entity_id_by_identity(
        entity_rows,
        &outgoing_target_identity(
            relation_rows,
            source_identity,
            WorthTopologyRelationKind::HalfEdgeNext,
        ),
    )
}

pub(super) fn prev_target_half_edge_id(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    source_identity: &str,
) -> EntityId {
    find_entity_id_by_identity(
        entity_rows,
        &outgoing_target_identity(
            relation_rows,
            source_identity,
            WorthTopologyRelationKind::HalfEdgePrev,
        ),
    )
}

pub(super) fn half_edge_identities_for_different_loops(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
) -> (String, String) {
    let half_edges = entity_rows
        .iter()
        .filter(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == WorthTopologyEntityKind::HalfEdge.kind_name())
        })
        .collect::<Vec<_>>();
    for left in &half_edges {
        let left_loop = incoming_source_identity(
            relation_rows,
            left.identity.as_str(),
            WorthTopologyRelationKind::LoopOwnsHalfEdge,
        );
        for right in &half_edges {
            if left.identity == right.identity {
                continue;
            }
            let right_loop = incoming_source_identity(
                relation_rows,
                right.identity.as_str(),
                WorthTopologyRelationKind::LoopOwnsHalfEdge,
            );
            if left_loop != right_loop {
                return (left.identity.clone(), right.identity.clone());
            }
        }
    }
    panic!("seeded topology should expose halfedges on different loops");
}

pub(super) fn successor_cycle_identities(
    entity_rows: &[forge_query::facade::ForgeQueryEntity],
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    start_identity: &str,
    count: usize,
) -> Vec<String> {
    let mut identities = Vec::with_capacity(count);
    let mut current_identity = start_identity.to_string();
    for _ in 0..count {
        identities.push(current_identity.clone());
        current_identity = outgoing_target_identity(
            relation_rows,
            &current_identity,
            WorthTopologyRelationKind::HalfEdgeNext,
        );
        let _ = find_entity_id_by_identity(entity_rows, &current_identity);
    }
    identities
}

fn outgoing_target_identity(
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    source_identity: &str,
    relation_kind: WorthTopologyRelationKind,
) -> String {
    relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == relation_kind.kind_name())
                && row
                    .payload
                    .get("topology")
                    .and_then(|value| value.get("source_identity"))
                    .and_then(|value| value.as_str())
                    .is_some_and(|value| value == source_identity)
        })
        .and_then(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("target_identity"))
                .and_then(|value| value.as_str())
        })
        .map(str::to_string)
        .expect("seeded topology should expose target identity for requested relation")
}

fn incoming_source_identity(
    relation_rows: &[forge_query::facade::ForgeQueryEntity],
    target_identity: &str,
    relation_kind: WorthTopologyRelationKind,
) -> String {
    relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == relation_kind.kind_name())
                && row
                    .payload
                    .get("topology")
                    .and_then(|value| value.get("target_identity"))
                    .and_then(|value| value.as_str())
                    .is_some_and(|value| value == target_identity)
        })
        .and_then(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("source_identity"))
                .and_then(|value| value.as_str())
        })
        .map(str::to_string)
        .expect("seeded topology should expose source identity for requested incoming relation")
}
