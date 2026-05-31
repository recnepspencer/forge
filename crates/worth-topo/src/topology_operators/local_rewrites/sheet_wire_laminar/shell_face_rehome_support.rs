use std::collections::BTreeSet;

use schema::facade::platform::authority::EntityReference;
use schema::facade::platform::entities::TopologyEntityKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::bindings::{
    query_entity_binding, query_entity_id_by_identity, query_incoming_relation_ids,
    query_outgoing_relation_target_identities,
};
use crate::topology_operators::TopologyEditContract;
use crate::topology_operators::{ShellOrWireMembershipKind, TopologyEditAction};

pub(crate) struct ShellFaceRehomeProgram {
    pub(super) create_key: String,
    pub(super) region_id: forge_relational::facade::identity::EntityId,
    pub(super) face_ids: Vec<forge_relational::facade::identity::EntityId>,
    pub(super) retired_shell_id: forge_relational::facade::identity::EntityId,
}

pub(crate) struct ShellFaceSplitProgram {
    pub(super) create_key: String,
    pub(super) region_id: forge_relational::facade::identity::EntityId,
    pub(super) face_id: forge_relational::facade::identity::EntityId,
    pub(super) retained_shell_id: Option<forge_relational::facade::identity::EntityId>,
}

pub(crate) fn parse_shell_face_rehome_program(
    contracts: &[TopologyEditContract],
) -> Option<ShellFaceRehomeProgram> {
    let [create, attach_region, face_attaches @ .., retire] = contracts else {
        return None;
    };
    let (
        TopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: TopologyEntityKind::Shell,
            ..
        },
        TopologyEditAction::AttachShellOrWireMembership {
            kind: ShellOrWireMembershipKind::RegionOwnsShell,
            owner: EntityReference::Existing(region_id),
            member: EntityReference::Created(member_key),
            ..
        },
        TopologyEditAction::RetireTopologyEntity {
            entity_id: retired_shell_id,
            kind: TopologyEntityKind::Shell,
        },
    ) = (&create.action, &attach_region.action, &retire.action)
    else {
        return None;
    };
    if create_key.as_str() != member_key.as_str() || face_attaches.is_empty() {
        return None;
    }
    let mut face_ids = Vec::with_capacity(face_attaches.len());
    let mut seen_face_ids = BTreeSet::new();
    for attach in face_attaches {
        let TopologyEditAction::AttachShellOrWireMembership {
            kind: ShellOrWireMembershipKind::ShellOwnsFace,
            owner: EntityReference::Created(owner_key),
            member: EntityReference::Existing(face_id),
            ..
        } = &attach.action
        else {
            return None;
        };
        if owner_key.as_str() != create_key.as_str() || !seen_face_ids.insert(*face_id) {
            return None;
        }
        face_ids.push(*face_id);
    }
    Some(ShellFaceRehomeProgram {
        create_key: create_key.as_str().to_string(),
        region_id: *region_id,
        face_ids,
        retired_shell_id: *retired_shell_id,
    })
}

pub(crate) fn parse_shell_face_split_program(
    contracts: &[TopologyEditContract],
) -> Option<ShellFaceSplitProgram> {
    let [create, attach_region, attach_face] = contracts else {
        return None;
    };
    let (
        TopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: TopologyEntityKind::Shell,
            ..
        },
        TopologyEditAction::AttachShellOrWireMembership {
            kind: ShellOrWireMembershipKind::RegionOwnsShell,
            owner: EntityReference::Existing(region_id),
            member: EntityReference::Created(member_key),
            ..
        },
        TopologyEditAction::AttachShellOrWireMembership {
            kind: ShellOrWireMembershipKind::ShellOwnsFace,
            owner: EntityReference::Created(owner_key),
            member: EntityReference::Existing(face_id),
            ..
        },
    ) = (&create.action, &attach_region.action, &attach_face.action)
    else {
        return None;
    };
    if create_key.as_str() != member_key.as_str() || create_key.as_str() != owner_key.as_str() {
        return None;
    }
    Some(ShellFaceSplitProgram {
        create_key: create_key.as_str().to_string(),
        region_id: *region_id,
        face_id: *face_id,
        retained_shell_id: None,
    })
}

pub(crate) fn resolve_single_face_two_face_shell_split_program(
    bindings: &TopologyQueryBindingIndex,
    contracts: &[TopologyEditContract],
) -> Option<ShellFaceSplitProgram> {
    let mut program = parse_shell_face_split_program(contracts)?;
    let face_binding = query_entity_binding(bindings, program.face_id)
        .ok()
        .flatten()?;
    let incoming_shell_ids = query_incoming_relation_ids(
        bindings,
        &face_binding.query_identity,
        schema::facade::platform::relations::TopologyRelationKind::ShellOwnsFace,
    )
    .ok()?;
    let [shell_owns_face_relation_id] = incoming_shell_ids.as_slice() else {
        return None;
    };
    let shell_owns_face_relation =
        crate::topology_operators::application::bindings::query_relation_binding(
            bindings,
            *shell_owns_face_relation_id,
        )
        .ok()
        .flatten()?;
    let retained_shell_id =
        query_entity_id_by_identity(bindings, &shell_owns_face_relation.source_query_identity)
            .ok()
            .flatten()?;
    let retained_shell_binding = query_entity_binding(bindings, retained_shell_id)
        .ok()
        .flatten()?;
    let incoming_region_ids = query_incoming_relation_ids(
        bindings,
        &retained_shell_binding.query_identity,
        schema::facade::platform::relations::TopologyRelationKind::RegionOwnsShell,
    )
    .ok()?;
    let [region_owns_shell_relation_id] = incoming_region_ids.as_slice() else {
        return None;
    };
    let region_owns_shell_relation =
        crate::topology_operators::application::bindings::query_relation_binding(
            bindings,
            *region_owns_shell_relation_id,
        )
        .ok()
        .flatten()?;
    let owned_region_id =
        query_entity_id_by_identity(bindings, &region_owns_shell_relation.source_query_identity)
            .ok()
            .flatten()?;
    if owned_region_id != program.region_id {
        return None;
    }
    let outgoing_face_targets = query_outgoing_relation_target_identities(
        bindings,
        &retained_shell_binding.query_identity,
        schema::facade::platform::relations::TopologyRelationKind::ShellOwnsFace,
    )
    .ok()?;
    if outgoing_face_targets.len() != 2 {
        return None;
    }
    let outgoing_face_ids = outgoing_face_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(bindings, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>()?;
    if !outgoing_face_ids.contains(&program.face_id) {
        return None;
    }
    program.retained_shell_id = Some(retained_shell_id);
    Some(program)
}
