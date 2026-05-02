use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_query::facade::ForgeQueryEntity;
use worth_schema::facade::{WorthEntityReference, WorthTopologyEntityKind};

use super::bindings::{
    query_entity_binding, query_entity_id_by_identity, query_incoming_relation_ids,
    query_outgoing_relation_target_identities, query_relation_binding,
};
use super::WorthTopologyEditContract;
use crate::edit::{WorthShellOrWireMembershipKind, WorthTopologyEditAction};

pub(super) struct WireRehomeWorkflow {
    pub(super) create_key: String,
    pub(super) half_edge_ids: Vec<forge_relational::facade::identity::EntityId>,
    pub(super) retired_wire_id: forge_relational::facade::identity::EntityId,
}

pub(super) fn parse_wire_rehome_workflow(
    contracts: &[WorthTopologyEditContract],
) -> Option<WireRehomeWorkflow> {
    let [create, attaches @ .., retire] = contracts else {
        return None;
    };
    let (
        WorthTopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: WorthTopologyEntityKind::Wire,
            ..
        },
        WorthTopologyEditAction::RetireTopologyEntity {
            entity_id: retired_wire_id,
            kind: WorthTopologyEntityKind::Wire,
        },
    ) = (&create.action, &retire.action)
    else {
        return None;
    };
    if attaches.is_empty() {
        return None;
    }
    let mut half_edge_ids = Vec::with_capacity(attaches.len());
    let mut seen_half_edge_ids = BTreeSet::new();
    for attach in attaches {
        let WorthTopologyEditAction::AttachShellOrWireMembership {
            kind: WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            owner: WorthEntityReference::Created(owner_key),
            member: WorthEntityReference::Existing(half_edge_id),
            ..
        } = &attach.action
        else {
            return None;
        };
        if owner_key.as_str() != create_key.as_str() || !seen_half_edge_ids.insert(*half_edge_id) {
            return None;
        }
        half_edge_ids.push(*half_edge_id);
    }
    Some(WireRehomeWorkflow {
        create_key: create_key.as_str().to_string(),
        half_edge_ids,
        retired_wire_id: *retired_wire_id,
    })
}

pub(super) fn supports_owned_half_edge_set_wire_rehome_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[WorthTopologyEditContract],
) -> bool {
    let Some(workflow) = parse_wire_rehome_workflow(contracts) else {
        return false;
    };
    let Some(retired_wire_binding) = query_entity_binding(entity_rows, workflow.retired_wire_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Ok(outgoing_half_edge_targets) = query_outgoing_relation_target_identities(
        relation_rows,
        &retired_wire_binding.query_identity,
        worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
    ) else {
        return false;
    };
    let expected_half_edge_ids = workflow
        .half_edge_ids
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let outgoing_half_edge_ids = outgoing_half_edge_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(entity_rows, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>();
    outgoing_half_edge_targets.len() == workflow.half_edge_ids.len()
        && outgoing_half_edge_ids
            .is_some_and(|ids| ids.into_iter().collect::<BTreeSet<_>>() == expected_half_edge_ids)
}

pub(super) struct WireSplitWorkflow {
    pub(super) create_key: String,
    pub(super) half_edge_ids: Vec<forge_relational::facade::identity::EntityId>,
    pub(super) retained_wire_id: Option<forge_relational::facade::identity::EntityId>,
}

pub(super) fn parse_wire_split_workflow(
    contracts: &[WorthTopologyEditContract],
) -> Option<WireSplitWorkflow> {
    let [create, attaches @ ..] = contracts else {
        return None;
    };
    let WorthTopologyEditAction::CreateTopologyEntity {
        create_key,
        kind: WorthTopologyEntityKind::Wire,
        ..
    } = &create.action
    else {
        return None;
    };
    if attaches.is_empty() {
        return None;
    }
    let mut half_edge_ids = Vec::with_capacity(attaches.len());
    let mut seen_half_edge_ids = BTreeSet::new();
    for attach in attaches {
        let WorthTopologyEditAction::AttachShellOrWireMembership {
            kind: WorthShellOrWireMembershipKind::WireOwnsHalfEdge,
            owner: WorthEntityReference::Created(owner_key),
            member: WorthEntityReference::Existing(half_edge_id),
            ..
        } = &attach.action
        else {
            return None;
        };
        if owner_key.as_str() != create_key.as_str() || !seen_half_edge_ids.insert(*half_edge_id) {
            return None;
        }
        half_edge_ids.push(*half_edge_id);
    }
    Some(WireSplitWorkflow {
        create_key: create_key.as_str().to_string(),
        half_edge_ids,
        retained_wire_id: None,
    })
}

pub(super) fn supports_connected_wire_split_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[WorthTopologyEditContract],
) -> bool {
    let Some(mut workflow) = parse_wire_split_workflow(contracts) else {
        return false;
    };
    let Some(retained_wire_id) =
        shared_existing_wire_owner_id(entity_rows, relation_rows, &workflow.half_edge_ids)
    else {
        return false;
    };
    workflow.retained_wire_id = Some(retained_wire_id);
    let Some(retained_wire_binding) = query_entity_binding(entity_rows, retained_wire_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Ok(outgoing_half_edge_targets) = query_outgoing_relation_target_identities(
        relation_rows,
        &retained_wire_binding.query_identity,
        worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
    ) else {
        return false;
    };
    let outgoing_half_edge_ids = outgoing_half_edge_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(entity_rows, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>();
    let Some(outgoing_half_edge_ids) = outgoing_half_edge_ids else {
        return false;
    };
    let moved_half_edge_ids = workflow
        .half_edge_ids
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let retained_half_edge_ids = outgoing_half_edge_ids
        .into_iter()
        .filter(|half_edge_id| !moved_half_edge_ids.contains(half_edge_id))
        .collect::<BTreeSet<_>>();
    !retained_half_edge_ids.is_empty()
        && connected_by_incident_vertices(entity_rows, relation_rows, &moved_half_edge_ids)
        && connected_by_incident_vertices(entity_rows, relation_rows, &retained_half_edge_ids)
}

pub(super) fn resolve_wire_split_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[WorthTopologyEditContract],
) -> Option<WireSplitWorkflow> {
    let mut workflow = parse_wire_split_workflow(contracts)?;
    workflow.retained_wire_id = Some(shared_existing_wire_owner_id(
        entity_rows,
        relation_rows,
        &workflow.half_edge_ids,
    )?);
    supports_connected_wire_split_workflow(entity_rows, relation_rows, contracts)
        .then_some(workflow)
}

fn shared_existing_wire_owner_id(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    half_edge_ids: &[forge_relational::facade::identity::EntityId],
) -> Option<forge_relational::facade::identity::EntityId> {
    let mut owner_id = None;
    for half_edge_id in half_edge_ids {
        let half_edge_binding = query_entity_binding(entity_rows, *half_edge_id)
            .ok()
            .flatten()?;
        let incoming_relation_ids = query_incoming_relation_ids(
            relation_rows,
            &half_edge_binding.query_identity,
            worth_schema::facade::WorthTopologyRelationKind::WireOwnsHalfEdge,
        )
        .ok()?;
        let [relation_id] = incoming_relation_ids.as_slice() else {
            return None;
        };
        let relation_binding = query_relation_binding(relation_rows, *relation_id)
            .ok()
            .flatten()?;
        let candidate_owner_id =
            query_entity_id_by_identity(entity_rows, &relation_binding.source_query_identity)
                .ok()
                .flatten()?;
        if owner_id.get_or_insert(candidate_owner_id) != &candidate_owner_id {
            return None;
        }
    }
    owner_id
}

fn connected_by_incident_vertices(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    half_edge_ids: &BTreeSet<forge_relational::facade::identity::EntityId>,
) -> bool {
    if half_edge_ids.is_empty() {
        return false;
    }
    let mut incident_vertices = BTreeMap::new();
    for half_edge_id in half_edge_ids {
        let Some(half_edge_binding) = query_entity_binding(entity_rows, *half_edge_id)
            .ok()
            .flatten()
        else {
            return false;
        };
        let mut vertices = BTreeSet::new();
        for kind in [
            worth_schema::facade::WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
            worth_schema::facade::WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
        ] {
            let Ok(target_identities) = query_outgoing_relation_target_identities(
                relation_rows,
                &half_edge_binding.query_identity,
                kind,
            ) else {
                return false;
            };
            for target_identity in target_identities {
                let Some(vertex_id) = query_entity_id_by_identity(entity_rows, &target_identity)
                    .ok()
                    .flatten()
                else {
                    return false;
                };
                vertices.insert(vertex_id);
            }
        }
        if vertices.is_empty() {
            return false;
        }
        incident_vertices.insert(*half_edge_id, vertices);
    }
    let seed_id = *half_edge_ids
        .iter()
        .next()
        .expect("non-empty set checked above");
    let mut visited = BTreeSet::from([seed_id]);
    let mut queue = VecDeque::from([seed_id]);
    while let Some(current_id) = queue.pop_front() {
        let current_vertices = &incident_vertices[&current_id];
        for neighbor_id in half_edge_ids {
            if visited.contains(neighbor_id) {
                continue;
            }
            if incident_vertices[neighbor_id]
                .iter()
                .any(|vertex_id| current_vertices.contains(vertex_id))
            {
                visited.insert(*neighbor_id);
                queue.push_back(*neighbor_id);
            }
        }
    }
    visited.len() == half_edge_ids.len()
}
