use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::{WorthTopologyRelationKind, WorthTopologyRelationKind::*};

use super::bindings::{
    query_entity_binding, query_entity_id_by_identity, query_incoming_relation_source_identities,
    query_outgoing_relation_target_identities, query_relation_binding,
};

pub(super) fn matches_expected_rewire(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    desired: Option<&DesiredLoopSuccessorRewire>,
    source_half_edge_id: EntityId,
    relation_kind: WorthTopologyRelationKind,
    expected_target_half_edge_id: EntityId,
) -> bool {
    let Some(desired) = desired else {
        return false;
    };
    let Some(live) = live_relation_for_source(
        entity_rows,
        relation_rows,
        source_half_edge_id,
        relation_kind,
    ) else {
        return false;
    };
    desired.relation_id == live.relation_id
        && desired.target_half_edge_id == expected_target_half_edge_id
}

pub(super) fn live_relation_for_source(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    source_half_edge_id: EntityId,
    relation_kind: WorthTopologyRelationKind,
) -> Option<LiveLoopSuccessorRelation> {
    let source_binding = query_entity_binding(entity_rows, source_half_edge_id).ok()??;
    let targets = query_outgoing_relation_target_identities(
        relation_rows,
        &source_binding.query_identity,
        relation_kind,
    )
    .ok()?;
    if targets.len() != 1 {
        return None;
    }
    let target_half_edge_id = query_entity_id_by_identity(entity_rows, &targets[0]).ok()??;
    let relation = relation_rows.iter().find_map(|row| {
        let binding = query_relation_binding(relation_rows, relation_id_from_row(row)).ok()??;
        (binding.kind == relation_kind
            && binding.source_query_identity == source_binding.query_identity
            && binding.target_query_identity == targets[0])
            .then_some(LiveLoopSuccessorRelation {
                relation_id: relation_id_from_row(row),
                target_half_edge_id,
            })
    })?;
    Some(relation)
}

pub(super) fn same_loop(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    left_half_edge_id: EntityId,
    right_half_edge_id: EntityId,
) -> bool {
    let Some(left_binding) = query_entity_binding(entity_rows, left_half_edge_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Some(right_binding) = query_entity_binding(entity_rows, right_half_edge_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Ok(left_loops) = query_incoming_relation_source_identities(
        relation_rows,
        &left_binding.query_identity,
        LoopOwnsHalfEdge,
    ) else {
        return false;
    };
    let Ok(right_loops) = query_incoming_relation_source_identities(
        relation_rows,
        &right_binding.query_identity,
        LoopOwnsHalfEdge,
    ) else {
        return false;
    };
    left_loops.len() == 1 && right_loops.len() == 1 && left_loops[0] == right_loops[0]
}

fn relation_id_from_row(row: &ForgeQueryEntity) -> RelationId {
    serde_json::from_value(row.payload["lineage"]["provenance"].clone())
        .expect("query relation provenance should decode")
}

pub(super) struct DesiredLoopSuccessorWorkflow {
    pub(super) next: std::collections::BTreeMap<EntityId, DesiredLoopSuccessorRewire>,
    pub(super) prev: std::collections::BTreeMap<EntityId, DesiredLoopSuccessorRewire>,
}

pub(super) struct DesiredLoopSuccessorRewire {
    pub(super) relation_id: RelationId,
    pub(super) target_half_edge_id: EntityId,
}

pub(super) struct LiveLoopSuccessorRelation {
    pub(super) relation_id: RelationId,
    pub(super) target_half_edge_id: EntityId,
}

pub(super) struct ContiguousSpanCandidate {
    pub(super) start_half_edge_id: EntityId,
    pub(super) end_half_edge_id: EntityId,
    pub(super) span_half_edge_ids: Vec<EntityId>,
}
