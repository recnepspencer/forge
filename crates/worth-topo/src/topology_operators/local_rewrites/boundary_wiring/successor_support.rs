use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::platform::relations::{TopologyRelationKind, TopologyRelationKind::*};

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::bindings::{
    query_entity_binding, query_entity_id_by_identity, query_incoming_relation_source_identities,
    query_outgoing_relation_ids, query_outgoing_relation_target_identities, query_relation_binding,
};

pub(super) fn matches_expected_rewire(
    bindings: &TopologyQueryBindingIndex,
    desired: Option<&DesiredLoopSuccessorRewire>,
    source_half_edge_id: EntityId,
    relation_kind: TopologyRelationKind,
    expected_target_half_edge_id: EntityId,
) -> bool {
    let Some(desired) = desired else {
        return false;
    };
    let Some(live) = live_relation_for_source(bindings, source_half_edge_id, relation_kind) else {
        return false;
    };
    desired.relation_id == live.relation_id
        && desired.target_half_edge_id == expected_target_half_edge_id
}

pub(super) fn live_relation_for_source(
    bindings: &TopologyQueryBindingIndex,
    source_half_edge_id: EntityId,
    relation_kind: TopologyRelationKind,
) -> Option<LiveLoopSuccessorRelation> {
    let source_binding = query_entity_binding(bindings, source_half_edge_id).ok()??;
    let targets = query_outgoing_relation_target_identities(
        bindings,
        &source_binding.query_identity,
        relation_kind,
    )
    .ok()?;
    if targets.len() != 1 {
        return None;
    }
    let target_half_edge_id = query_entity_id_by_identity(bindings, &targets[0]).ok()??;
    let relation_ids =
        query_outgoing_relation_ids(bindings, &source_binding.query_identity, relation_kind)
            .ok()?;
    let [relation_id] = relation_ids.as_slice() else {
        return None;
    };
    let binding = query_relation_binding(bindings, *relation_id).ok()??;
    (binding.kind == relation_kind
        && binding.source_query_identity == source_binding.query_identity
        && binding.target_query_identity == targets[0])
        .then_some(LiveLoopSuccessorRelation {
            relation_id: *relation_id,
            target_half_edge_id,
        })
}

pub(super) fn same_loop(
    bindings: &TopologyQueryBindingIndex,
    left_half_edge_id: EntityId,
    right_half_edge_id: EntityId,
) -> bool {
    let Some(left_binding) = query_entity_binding(bindings, left_half_edge_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Some(right_binding) = query_entity_binding(bindings, right_half_edge_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Ok(left_loops) = query_incoming_relation_source_identities(
        bindings,
        &left_binding.query_identity,
        LoopOwnsHalfEdge,
    ) else {
        return false;
    };
    let Ok(right_loops) = query_incoming_relation_source_identities(
        bindings,
        &right_binding.query_identity,
        LoopOwnsHalfEdge,
    ) else {
        return false;
    };
    left_loops.len() == 1 && right_loops.len() == 1 && left_loops[0] == right_loops[0]
}

pub(super) struct DesiredLoopSuccessorProgram {
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




