use std::collections::{BTreeMap, BTreeSet, VecDeque};

use schema::facade::platform::authority::EntityReference;
use schema::facade::platform::entities::TopologyEntityKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::bindings::{
    query_entity_binding, query_entity_id_by_identity, query_incoming_relation_ids,
    query_outgoing_relation_target_identities, query_relation_binding,
};
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyDeclaredMutationActionRef, TopologyDeclaredMutationSequence,
};

pub(crate) struct WireRehomeProgram {
    pub(super) create_key: String,
    pub(super) half_edge_ids: Vec<forge_relational::facade::identity::EntityId>,
    pub(super) retired_wire_id: forge_relational::facade::identity::EntityId,
}

pub(crate) fn parse_wire_rehome_program(
    sequence: &TopologyDeclaredMutationSequence,
) -> Option<WireRehomeProgram> {
    let members = sequence.members().collect::<Vec<_>>();
    let [create, attaches @ .., retire] = members.as_slice() else {
        return None;
    };
    let (
        TopologyDeclaredMutationActionRef::CreateTopologyEntity {
            create_key,
            kind: TopologyEntityKind::Wire,
        },
        TopologyDeclaredMutationActionRef::RetireTopologyEntity {
            entity_id: retired_wire_id,
            kind: TopologyEntityKind::Wire,
        },
    ) = (create.action_ref(), retire.action_ref())
    else {
        return None;
    };
    if attaches.is_empty() {
        return None;
    }
    let mut half_edge_ids = Vec::with_capacity(attaches.len());
    let mut seen_half_edge_ids = BTreeSet::new();
    for attach in attaches {
        let TopologyDeclaredMutationActionRef::AttachShellOrWireMembership {
            kind: ShellOrWireMembershipKind::WireOwnsHalfEdge,
            owner: EntityReference::Created(owner_key),
            member: EntityReference::Existing(half_edge_id),
        } = attach.action_ref()
        else {
            return None;
        };
        if owner_key.as_str() != create_key || !seen_half_edge_ids.insert(*half_edge_id) {
            return None;
        }
        half_edge_ids.push(*half_edge_id);
    }
    Some(WireRehomeProgram {
        create_key: create_key.to_string(),
        half_edge_ids,
        retired_wire_id,
    })
}

pub(crate) struct WireSplitProgram {
    pub(super) create_key: String,
    pub(super) half_edge_ids: Vec<forge_relational::facade::identity::EntityId>,
    pub(super) retained_wire_id: Option<forge_relational::facade::identity::EntityId>,
}

pub(crate) fn parse_wire_split_program(
    sequence: &TopologyDeclaredMutationSequence,
) -> Option<WireSplitProgram> {
    let members = sequence.members().collect::<Vec<_>>();
    let [create, attaches @ ..] = members.as_slice() else {
        return None;
    };
    let TopologyDeclaredMutationActionRef::CreateTopologyEntity {
        create_key,
        kind: TopologyEntityKind::Wire,
    } = create.action_ref()
    else {
        return None;
    };
    if attaches.is_empty() {
        return None;
    }
    let mut half_edge_ids = Vec::with_capacity(attaches.len());
    let mut seen_half_edge_ids = BTreeSet::new();
    for attach in attaches {
        let TopologyDeclaredMutationActionRef::AttachShellOrWireMembership {
            kind: ShellOrWireMembershipKind::WireOwnsHalfEdge,
            owner: EntityReference::Created(owner_key),
            member: EntityReference::Existing(half_edge_id),
        } = attach.action_ref()
        else {
            return None;
        };
        if owner_key.as_str() != create_key || !seen_half_edge_ids.insert(*half_edge_id) {
            return None;
        }
        half_edge_ids.push(*half_edge_id);
    }
    Some(WireSplitProgram {
        create_key: create_key.to_string(),
        half_edge_ids,
        retained_wire_id: None,
    })
}

pub(crate) fn resolve_wire_split_program(
    bindings: &TopologyQueryBindingIndex,
    sequence: &TopologyDeclaredMutationSequence,
) -> Option<WireSplitProgram> {
    let mut program = parse_wire_split_program(sequence)?;
    let retained_wire_id = shared_existing_wire_owner_id(bindings, &program.half_edge_ids)?;
    program.retained_wire_id = Some(retained_wire_id);
    let retained_wire_binding = query_entity_binding(bindings, retained_wire_id)
        .ok()
        .flatten()?;
    let outgoing_half_edge_targets = query_outgoing_relation_target_identities(
        bindings,
        &retained_wire_binding.query_identity_label,
        schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
    )
    .ok()?;
    let outgoing_half_edge_ids = outgoing_half_edge_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(bindings, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>()?;
    let moved_half_edge_ids = program
        .half_edge_ids
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let retained_half_edge_ids = outgoing_half_edge_ids
        .into_iter()
        .filter(|half_edge_id| !moved_half_edge_ids.contains(half_edge_id))
        .collect::<BTreeSet<_>>();
    (!retained_half_edge_ids.is_empty()
        && connected_by_incident_vertices(bindings, &moved_half_edge_ids)
        && connected_by_incident_vertices(bindings, &retained_half_edge_ids))
    .then_some(program)
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
            &half_edge_binding.query_identity_label,
            schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
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
            schema::facade::platform::relations::TopologyRelationKind::HalfEdgeStartsAtVertex,
            schema::facade::platform::relations::TopologyRelationKind::HalfEdgeEndsAtVertex,
        ] {
            let Ok(target_identities) = query_outgoing_relation_target_identities(
                bindings,
                &half_edge_binding.query_identity_label,
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
