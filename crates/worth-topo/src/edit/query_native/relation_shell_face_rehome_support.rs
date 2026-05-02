use std::collections::BTreeSet;

use forge_query::facade::ForgeQueryEntity;
use worth_schema::facade::{WorthEntityReference, WorthTopologyEntityKind};

use super::bindings::{
    query_entity_binding, query_entity_id_by_identity, query_incoming_relation_ids,
    query_outgoing_relation_target_identities,
};
use super::WorthTopologyEditContract;
use crate::edit::{WorthShellOrWireMembershipKind, WorthTopologyEditAction};

pub(super) struct ShellFaceRehomeWorkflow {
    pub(super) create_key: String,
    pub(super) region_id: forge_relational::facade::identity::EntityId,
    pub(super) face_ids: Vec<forge_relational::facade::identity::EntityId>,
    pub(super) retired_shell_id: forge_relational::facade::identity::EntityId,
}

pub(super) struct ShellFaceSplitWorkflow {
    pub(super) create_key: String,
    pub(super) region_id: forge_relational::facade::identity::EntityId,
    pub(super) face_id: forge_relational::facade::identity::EntityId,
    pub(super) retained_shell_id: Option<forge_relational::facade::identity::EntityId>,
}

pub(super) fn parse_shell_face_rehome_workflow(
    contracts: &[WorthTopologyEditContract],
) -> Option<ShellFaceRehomeWorkflow> {
    let [create, attach_region, face_attaches @ .., retire] = contracts else {
        return None;
    };
    let (
        WorthTopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: WorthTopologyEntityKind::Shell,
            ..
        },
        WorthTopologyEditAction::AttachShellOrWireMembership {
            kind: WorthShellOrWireMembershipKind::RegionOwnsShell,
            owner: WorthEntityReference::Existing(region_id),
            member: WorthEntityReference::Created(member_key),
            ..
        },
        WorthTopologyEditAction::RetireTopologyEntity {
            entity_id: retired_shell_id,
            kind: WorthTopologyEntityKind::Shell,
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
        let WorthTopologyEditAction::AttachShellOrWireMembership {
            kind: WorthShellOrWireMembershipKind::ShellOwnsFace,
            owner: WorthEntityReference::Created(owner_key),
            member: WorthEntityReference::Existing(face_id),
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
    Some(ShellFaceRehomeWorkflow {
        create_key: create_key.as_str().to_string(),
        region_id: *region_id,
        face_ids,
        retired_shell_id: *retired_shell_id,
    })
}

pub(super) fn parse_shell_face_split_workflow(
    contracts: &[WorthTopologyEditContract],
) -> Option<ShellFaceSplitWorkflow> {
    let [create, attach_region, attach_face] = contracts else {
        return None;
    };
    let (
        WorthTopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: WorthTopologyEntityKind::Shell,
            ..
        },
        WorthTopologyEditAction::AttachShellOrWireMembership {
            kind: WorthShellOrWireMembershipKind::RegionOwnsShell,
            owner: WorthEntityReference::Existing(region_id),
            member: WorthEntityReference::Created(member_key),
            ..
        },
        WorthTopologyEditAction::AttachShellOrWireMembership {
            kind: WorthShellOrWireMembershipKind::ShellOwnsFace,
            owner: WorthEntityReference::Created(owner_key),
            member: WorthEntityReference::Existing(face_id),
            ..
        },
    ) = (&create.action, &attach_region.action, &attach_face.action)
    else {
        return None;
    };
    if create_key.as_str() != member_key.as_str() || create_key.as_str() != owner_key.as_str() {
        return None;
    }
    Some(ShellFaceSplitWorkflow {
        create_key: create_key.as_str().to_string(),
        region_id: *region_id,
        face_id: *face_id,
        retained_shell_id: None,
    })
}

pub(super) fn supports_owned_face_set_shell_rehome_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[WorthTopologyEditContract],
) -> bool {
    let Some(workflow) = parse_shell_face_rehome_workflow(contracts) else {
        return false;
    };
    let Some(retired_shell_binding) = query_entity_binding(entity_rows, workflow.retired_shell_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Ok(incoming_region_ids) = query_incoming_relation_ids(
        relation_rows,
        &retired_shell_binding.query_identity,
        worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell,
    ) else {
        return false;
    };
    let Ok(outgoing_face_targets) = query_outgoing_relation_target_identities(
        relation_rows,
        &retired_shell_binding.query_identity,
        worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
    ) else {
        return false;
    };
    let [incoming_region_relation_id] = incoming_region_ids.as_slice() else {
        return false;
    };
    let Some(incoming_region_relation) =
        super::bindings::query_relation_binding(relation_rows, *incoming_region_relation_id)
            .ok()
            .flatten()
    else {
        return false;
    };
    let expected_face_ids = workflow.face_ids.iter().copied().collect::<BTreeSet<_>>();
    let outgoing_face_ids = outgoing_face_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(entity_rows, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>();
    query_entity_id_by_identity(entity_rows, &incoming_region_relation.source_query_identity)
        .ok()
        .flatten()
        .is_some_and(|owned_region_id| owned_region_id == workflow.region_id)
        && outgoing_face_targets.len() == workflow.face_ids.len()
        && outgoing_face_ids
            .is_some_and(|ids| ids.into_iter().collect::<BTreeSet<_>>() == expected_face_ids)
}

pub(super) fn resolve_single_face_two_face_shell_split_workflow(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    contracts: &[WorthTopologyEditContract],
) -> Option<ShellFaceSplitWorkflow> {
    let mut workflow = parse_shell_face_split_workflow(contracts)?;
    let face_binding = query_entity_binding(entity_rows, workflow.face_id)
        .ok()
        .flatten()?;
    let incoming_shell_ids = query_incoming_relation_ids(
        relation_rows,
        &face_binding.query_identity,
        worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
    )
    .ok()?;
    let [shell_owns_face_relation_id] = incoming_shell_ids.as_slice() else {
        return None;
    };
    let shell_owns_face_relation =
        super::bindings::query_relation_binding(relation_rows, *shell_owns_face_relation_id)
            .ok()
            .flatten()?;
    let retained_shell_id =
        query_entity_id_by_identity(entity_rows, &shell_owns_face_relation.source_query_identity)
            .ok()
            .flatten()?;
    let retained_shell_binding = query_entity_binding(entity_rows, retained_shell_id)
        .ok()
        .flatten()?;
    let incoming_region_ids = query_incoming_relation_ids(
        relation_rows,
        &retained_shell_binding.query_identity,
        worth_schema::facade::WorthTopologyRelationKind::RegionOwnsShell,
    )
    .ok()?;
    let [region_owns_shell_relation_id] = incoming_region_ids.as_slice() else {
        return None;
    };
    let region_owns_shell_relation =
        super::bindings::query_relation_binding(relation_rows, *region_owns_shell_relation_id)
            .ok()
            .flatten()?;
    let owned_region_id = query_entity_id_by_identity(
        entity_rows,
        &region_owns_shell_relation.source_query_identity,
    )
    .ok()
    .flatten()?;
    if owned_region_id != workflow.region_id {
        return None;
    }
    let outgoing_face_targets = query_outgoing_relation_target_identities(
        relation_rows,
        &retained_shell_binding.query_identity,
        worth_schema::facade::WorthTopologyRelationKind::ShellOwnsFace,
    )
    .ok()?;
    if outgoing_face_targets.len() != 2 {
        return None;
    }
    let outgoing_face_ids = outgoing_face_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(entity_rows, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>()?;
    if !outgoing_face_ids.contains(&workflow.face_id) {
        return None;
    }
    workflow.retained_shell_id = Some(retained_shell_id);
    Some(workflow)
}
