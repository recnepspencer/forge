use super::shell_face_rehome_support::{
    parse_shell_face_rehome_program, resolve_single_face_two_face_shell_split_program,
};
use super::wire_rehome_support::{parse_wire_rehome_program, resolve_wire_split_program};
use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::bindings::{
    query_entity_binding, query_entity_id_by_identity, query_incoming_relation_ids,
    query_outgoing_relation_target_identities, query_relation_binding,
};
use crate::topology_operators::TopologyDeclaredMutationSequence;

#[cfg(test)]
pub(crate) fn supports_admitted_shell_or_wire_create_program(
    bindings: &TopologyQueryBindingIndex,
    sequence: &TopologyDeclaredMutationSequence,
) -> bool {
    supports_owned_half_edge_set_wire_rehome_program(bindings, sequence)
        || resolve_wire_split_program(bindings, sequence).is_some()
        || resolve_single_face_two_face_shell_split_program(bindings, sequence).is_some()
        || supports_owned_face_set_shell_rehome_program(bindings, sequence)
}

#[cfg(test)]
fn supports_owned_half_edge_set_wire_rehome_program(
    bindings: &TopologyQueryBindingIndex,
    sequence: &TopologyDeclaredMutationSequence,
) -> bool {
    let Some(program) = parse_wire_rehome_program(sequence) else {
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
        schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge,
    ) else {
        return false;
    };
    let expected_half_edge_ids = program
        .half_edge_ids
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let outgoing_half_edge_ids = outgoing_half_edge_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(bindings, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>();
    outgoing_half_edge_targets.len() == program.half_edge_ids.len()
        && outgoing_half_edge_ids.is_some_and(|ids| {
            ids.into_iter().collect::<std::collections::BTreeSet<_>>() == expected_half_edge_ids
        })
}
<<<<<<< HEAD
=======

#[cfg(test)]
fn supports_owned_face_set_shell_rehome_program(
    bindings: &TopologyQueryBindingIndex,
    sequence: &TopologyDeclaredMutationSequence,
) -> bool {
    let Some(program) = parse_shell_face_rehome_program(sequence) else {
        return false;
    };
    let Some(retired_shell_binding) = query_entity_binding(bindings, program.retired_shell_id)
        .ok()
        .flatten()
    else {
        return false;
    };
    let Ok(incoming_region_ids) = query_incoming_relation_ids(
        bindings,
        &retired_shell_binding.query_identity,
        schema::facade::platform::relations::TopologyRelationKind::RegionOwnsShell,
    ) else {
        return false;
    };
    let Ok(outgoing_face_targets) = query_outgoing_relation_target_identities(
        bindings,
        &retired_shell_binding.query_identity,
        schema::facade::platform::relations::TopologyRelationKind::ShellOwnsFace,
    ) else {
        return false;
    };
    let [incoming_region_relation_id] = incoming_region_ids.as_slice() else {
        return false;
    };
    let Some(incoming_region_relation) =
        query_relation_binding(bindings, *incoming_region_relation_id)
            .ok()
            .flatten()
    else {
        return false;
    };
    let expected_face_ids = program
        .face_ids
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let outgoing_face_ids = outgoing_face_targets
        .iter()
        .map(|identity| {
            query_entity_id_by_identity(bindings, identity)
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>();
    query_entity_id_by_identity(bindings, &incoming_region_relation.source_query_identity)
        .ok()
        .flatten()
        .is_some_and(|owned_region_id| owned_region_id == program.region_id)
        && outgoing_face_targets.len() == program.face_ids.len()
        && outgoing_face_ids.is_some_and(|ids| {
            ids.into_iter().collect::<std::collections::BTreeSet<_>>() == expected_face_ids
        })
}
>>>>>>> origin/master
