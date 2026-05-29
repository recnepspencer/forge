use std::collections::{BTreeMap, BTreeSet, VecDeque};

use schema::facade::{EntityReference, TopologyEntityKind};

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::bindings::{
    query_entity_binding, query_entity_id_by_identity, query_incoming_relation_ids,
    query_outgoing_relation_target_identities, query_relation_binding,
};
use crate::topology_operators::TopologyEditContract;
use crate::topology_operators::{ShellOrWireMembershipKind, TopologyEditAction};

pub(super) struct WireRehomeProgram {
    pub(super) create_key: String,
    pub(super) half_edge_ids: Vec<forge_relational::facade::identity::EntityId>,
    pub(super) retired_wire_id: forge_relational::facade::identity::EntityId,
}

pub(super) fn parse_wire_rehome_program(
    contracts: &[TopologyEditContract],
) -> Option<WireRehomeProgram> {
    let [create, attaches @ .., retire] = contracts else {
        return None;
    };
    let (
        TopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: TopologyEntityKind::Wire,
            ..
        },
        TopologyEditAction::RetireTopologyEntity {
            entity_id: retired_wire_id,
            kind: TopologyEntityKind::Wire,
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
        let TopologyEditAction::AttachShellOrWireMembership {
            kind: ShellOrWireMembershipKind::WireOwnsHalfEdge,
            owner: EntityReference::Created(owner_key),
            member: EntityReference::Existing(half_edge_id),
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
    Some(WireRehomeProgram {
        create_key: create_key.as_str().to_string(),
        half_edge_ids,
        retired_wire_id: *retired_wire_id,
    })
}

pub(super) fn supports_owned_half_edge_set_wire_rehome_program(
    bindings: &TopologyQueryBindingIndex,
    contracts: &[TopologyEditContract],
) -> bool {
    let Some(program) = parse_wire_rehome_program(contracts) else {
        return false;
    };
    let Some(retired_wire_binding) = query_entity_binding(bindings, program.retired_wire_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Ok(outgoing_half_edge_targets) = query_outgoing_relation_target_identities(
        bindings,
        &retired_wire_binding.query_identity,
        schema::facade::TopologyRelationKind::WireOwnsHalfEdge,
    ) else {
        return false;
    };
    let expected_half_edge_ids = program
        .half_edge_ids
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let outgoing_half_edge_ids = outgoing_half_edge_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(bindings, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>();
    outgoing_half_edge_targets.len() == program.half_edge_ids.len()
        && outgoing_half_edge_ids
            .is_some_and(|ids| ids.into_iter().collect::<BTreeSet<_>>() == expected_half_edge_ids)
}

pub(super) struct WireSplitProgram {
    pub(super) create_key: String,
    pub(super) half_edge_ids: Vec<forge_relational::facade::identity::EntityId>,
    pub(super) retained_wire_id: Option<forge_relational::facade::identity::EntityId>,
}

pub(super) fn parse_wire_split_program(
    contracts: &[TopologyEditContract],
) -> Option<WireSplitProgram> {
    let [create, attaches @ ..] = contracts else {
        return None;
    };
    let TopologyEditAction::CreateTopologyEntity {
        create_key,
        kind: TopologyEntityKind::Wire,
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
        let TopologyEditAction::AttachShellOrWireMembership {
            kind: ShellOrWireMembershipKind::WireOwnsHalfEdge,
            owner: EntityReference::Created(owner_key),
            member: EntityReference::Existing(half_edge_id),
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
    Some(WireSplitProgram {
        create_key: create_key.as_str().to_string(),
        half_edge_ids,
        retained_wire_id: None,
    })
}

pub(super) fn supports_connected_wire_split_program(
    bindings: &TopologyQueryBindingIndex,
    contracts: &[TopologyEditContract],
) -> bool {
    let Some(mut program) = parse_wire_split_program(contracts) else {
        return false;
    };
    let Some(retained_wire_id) = shared_existing_wire_owner_id(bindings, &program.half_edge_ids)
    else {
        return false;
    };
    program.retained_wire_id = Some(retained_wire_id);
    let Some(retained_wire_binding) = query_entity_binding(bindings, retained_wire_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Ok(outgoing_half_edge_targets) = query_outgoing_relation_target_identities(
        bindings,
        &retained_wire_binding.query_identity,
        schema::facade::TopologyRelationKind::WireOwnsHalfEdge,
    ) else {
        return false;
    };
    let outgoing_half_edge_ids = outgoing_half_edge_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(bindings, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>();
    let Some(outgoing_half_edge_ids) = outgoing_half_edge_ids else {
        return false;
    };
    let moved_half_edge_ids = program
        .half_edge_ids
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let retained_half_edge_ids = outgoing_half_edge_ids
        .into_iter()
        .filter(|half_edge_id| !moved_half_edge_ids.contains(half_edge_id))
        .collect::<BTreeSet<_>>();
    !retained_half_edge_ids.is_empty()
        && connected_by_incident_vertices(bindings, &moved_half_edge_ids)
        && connected_by_incident_vertices(bindings, &retained_half_edge_ids)
}

pub(super) fn resolve_wire_split_program(
    bindings: &TopologyQueryBindingIndex,
    contracts: &[TopologyEditContract],
) -> Option<WireSplitProgram> {
    let mut program = parse_wire_split_program(contracts)?;
    program.retained_wire_id = Some(shared_existing_wire_owner_id(
        bindings,
        &program.half_edge_ids,
    )?);
    supports_connected_wire_split_program(bindings, contracts).then_some(program)
}

fn shared_existing_wire_owner_id(
    bindings: &TopologyQueryBindingIndex,
    half_edge_ids: &[forge_relational::facade::identity::EntityId],
) -> Option<forge_relational::facade::identity::EntityId> {
    let mut owner_id = None;
    for half_edge_id in half_edge_ids {
        let half_edge_binding = query_entity_binding(bindings, *half_edge_id)
            .ok()
            .flatten()?;
        let incoming_relation_ids = query_incoming_relation_ids(
            bindings,
            &half_edge_binding.query_identity,
            schema::facade::TopologyRelationKind::WireOwnsHalfEdge,
        )
        .ok()?;
        let [relation_id] = incoming_relation_ids.as_slice() else {
            return None;
        };
        let relation_binding = query_relation_binding(bindings, *relation_id)
            .ok()
            .flatten()?;
        let candidate_owner_id =
            query_entity_id_by_identity(bindings, &relation_binding.source_query_identity)
                .ok()
                .flatten()?;
        if owner_id.get_or_insert(candidate_owner_id) != &candidate_owner_id {
            return None;
        }
    }
    owner_id
}

fn connected_by_incident_vertices(
    bindings: &TopologyQueryBindingIndex,
    half_edge_ids: &BTreeSet<forge_relational::facade::identity::EntityId>,
) -> bool {
    if half_edge_ids.is_empty() {
        return false;
    }
    let mut incident_vertices = BTreeMap::new();
    for half_edge_id in half_edge_ids {
        let Some(half_edge_binding) = query_entity_binding(bindings, *half_edge_id).ok().flatten()
        else {
            return false;
        };
        let mut vertices = BTreeSet::new();
        for kind in [
            schema::facade::TopologyRelationKind::HalfEdgeStartsAtVertex,
            schema::facade::TopologyRelationKind::HalfEdgeEndsAtVertex,
        ] {
            let Ok(target_identities) = query_outgoing_relation_target_identities(
                bindings,
                &half_edge_binding.query_identity,
                kind,
            ) else {
                return false;
            };
            for target_identity in target_identities {
                let Some(vertex_id) = query_entity_id_by_identity(bindings, &target_identity)
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
