#![cfg(test)]

use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::identity::EntityId;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::platform::relations::TopologyRelationKind::*;

use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryBindingIndex, TopologyQueryEditFamilySupportStatus, TopologyRuntimeSupport,
};
use crate::topology_operators::application::bindings::{
    query_entity_binding, query_entity_id_by_identity, query_incoming_relation_ids,
    query_incoming_relation_source_identities, query_outgoing_relation_target_identities,
    query_relation_binding,
};
use crate::topology_operators::application::TopologyDeclarationContractPayload;
use crate::topology_operators::local_rewrites::boundary_wiring::supports_admitted_relation_create_program;
use crate::topology_operators::local_rewrites::sheet_wire_laminar::supports_admitted_shell_or_wire_create_program;
use crate::topology_operators::{
    topology_edit_families_for_contracts, LoopSuccessorKind, TopologyEditAction,
    TopologyEditContract, TopologyEditFamily,
};

pub(crate) fn unsupported_families(
    support: &TopologyRuntimeSupport,
    bindings: &TopologyQueryBindingIndex,
    families: &[TopologyEditFamily],
    contracts: &[TopologyEditContract],
) -> Vec<TopologyEditFamily> {
    let admitted_relation_create_program = supports_admitted_relation_create_program(contracts);
    let admitted_shell_or_wire_create_program =
        supports_admitted_shell_or_wire_create_program(bindings, contracts);
    let admitted_loop_successor_program =
        supports_admitted_loop_successor_program(bindings, contracts);

    let mut unsupported = Vec::new();
    for family in families.iter().copied() {
        let supported = match support.query_edit_family_support_status(family) {
            TopologyQueryEditFamilySupportStatus::Admitted => true,
            TopologyQueryEditFamilySupportStatus::Denied => false,
            TopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane => {
                (family == TopologyEditFamily::AttachBoundaryMembership
                    && admitted_relation_create_program)
                    || (family == TopologyEditFamily::AttachShellOrWireMembership
                        && admitted_shell_or_wire_create_program)
                    || (family == TopologyEditFamily::RewireLoopSuccessor
                        && admitted_loop_successor_program)
            }
        };
        if !supported && !unsupported.contains(&family) {
            unsupported.push(family);
        }
    }
    unsupported
}

pub(crate) fn unsupported_declaration_families<D>(
    support: &TopologyRuntimeSupport,
    bindings: &TopologyQueryBindingIndex,
    declaration: &D,
) -> Vec<TopologyEditFamily>
where
    D: TopologyDeclarationContractPayload,
{
    let contracts = declaration.clone().into_contracts();
    let families = topology_edit_families_for_contracts(&contracts);
    unsupported_families(support, bindings, &families, &contracts)
}

fn supports_admitted_loop_successor_program(
    bindings: &TopologyQueryBindingIndex,
    contracts: &[TopologyEditContract],
) -> bool {
    let Some(program) = desired_successor_program(contracts) else {
        return false;
    };
    single_half_edge_candidates(&program)
        .into_iter()
        .any(|moved_half_edge_id| {
            matches_admitted_single_half_edge_relocation_program(
                bindings,
                &program,
                moved_half_edge_id,
            )
        })
        || contiguous_span_candidates(bindings, &program)
            .into_iter()
            .any(|candidate| {
                matches_admitted_contiguous_span_relocation_program(bindings, &program, candidate)
            })
}

fn desired_successor_program(
    contracts: &[TopologyEditContract],
) -> Option<DesiredLoopSuccessorProgram> {
    if contracts.len() != 6 {
        return None;
    }
    let mut next = BTreeMap::new();
    let mut prev = BTreeMap::new();
    for contract in contracts {
        let TopologyEditAction::RewireLoopSuccessor {
            relation_id,
            kind,
            half_edge_id,
            successor_half_edge_id,
        } = contract.action
        else {
            return None;
        };
        let desired = DesiredLoopSuccessorRewire {
            relation_id,
            target_half_edge_id: successor_half_edge_id,
        };
        match kind {
            LoopSuccessorKind::Next => {
                next.insert(half_edge_id, desired);
            }
            LoopSuccessorKind::Prev => {
                prev.insert(half_edge_id, desired);
            }
        }
    }
    (next.len() == 3 && prev.len() == 3).then_some(DesiredLoopSuccessorProgram { next, prev })
}

fn single_half_edge_candidates(program: &DesiredLoopSuccessorProgram) -> Vec<EntityId> {
    program
        .next
        .keys()
        .copied()
        .filter(|entity_id| program.prev.contains_key(entity_id))
        .collect()
}

fn matches_admitted_single_half_edge_relocation_program(
    bindings: &TopologyQueryBindingIndex,
    program: &DesiredLoopSuccessorProgram,
    moved_half_edge_id: EntityId,
) -> bool {
    let Some(moved_next) = program.next.get(&moved_half_edge_id) else {
        return false;
    };
    let Some(moved_prev) = program.prev.get(&moved_half_edge_id) else {
        return false;
    };
    let Some(live_next) = live_relation_for_source(bindings, moved_half_edge_id, HalfEdgeNext)
    else {
        return false;
    };
    let Some(live_prev) = live_relation_for_source(bindings, moved_half_edge_id, HalfEdgePrev)
    else {
        return false;
    };
    if moved_next.relation_id != live_next.relation_id
        || moved_prev.relation_id != live_prev.relation_id
    {
        return false;
    }
    if !same_loop(bindings, moved_half_edge_id, moved_next.target_half_edge_id)
        || !same_loop(bindings, moved_half_edge_id, moved_prev.target_half_edge_id)
    {
        return false;
    }

    matches_expected_rewire(
        bindings,
        program.next.get(&live_prev.target_half_edge_id),
        live_prev.target_half_edge_id,
        HalfEdgeNext,
        live_next.target_half_edge_id,
    ) && matches_expected_rewire(
        bindings,
        program.prev.get(&live_next.target_half_edge_id),
        live_next.target_half_edge_id,
        HalfEdgePrev,
        live_prev.target_half_edge_id,
    ) && matches_expected_rewire(
        bindings,
        program.next.get(&moved_prev.target_half_edge_id),
        moved_prev.target_half_edge_id,
        HalfEdgeNext,
        moved_half_edge_id,
    ) && matches_expected_rewire(
        bindings,
        program.prev.get(&moved_next.target_half_edge_id),
        moved_next.target_half_edge_id,
        HalfEdgePrev,
        moved_half_edge_id,
    )
}

fn contiguous_span_candidates(
    bindings: &TopologyQueryBindingIndex,
    program: &DesiredLoopSuccessorProgram,
) -> Vec<ContiguousSpanCandidate> {
    program
        .prev
        .keys()
        .copied()
        .filter(|start_half_edge_id| !program.next.contains_key(start_half_edge_id))
        .filter_map(|start_half_edge_id| {
            let mut span_half_edge_ids = vec![start_half_edge_id];
            let mut seen = BTreeSet::from([start_half_edge_id]);
            let mut current_half_edge_id = start_half_edge_id;
            loop {
                let live_next =
                    live_relation_for_source(bindings, current_half_edge_id, HalfEdgeNext)?;
                if program.prev.contains_key(&live_next.target_half_edge_id) {
                    return None;
                }
                let live_prev = live_relation_for_source(
                    bindings,
                    live_next.target_half_edge_id,
                    HalfEdgePrev,
                )?;
                if live_prev.target_half_edge_id != current_half_edge_id {
                    return None;
                }
                if program.next.contains_key(&live_next.target_half_edge_id) {
                    return Some(ContiguousSpanCandidate {
                        start_half_edge_id,
                        end_half_edge_id: live_next.target_half_edge_id,
                        span_half_edge_ids,
                    });
                }
                if !seen.insert(live_next.target_half_edge_id) {
                    return None;
                }
                span_half_edge_ids.push(live_next.target_half_edge_id);
                current_half_edge_id = live_next.target_half_edge_id;
            }
        })
        .collect()
}

fn matches_admitted_contiguous_span_relocation_program(
    bindings: &TopologyQueryBindingIndex,
    program: &DesiredLoopSuccessorProgram,
    candidate: ContiguousSpanCandidate,
) -> bool {
    let Some(start_prev) = program.prev.get(&candidate.start_half_edge_id) else {
        return false;
    };
    let Some(end_next) = program.next.get(&candidate.end_half_edge_id) else {
        return false;
    };
    let Some(live_start_prev) =
        live_relation_for_source(bindings, candidate.start_half_edge_id, HalfEdgePrev)
    else {
        return false;
    };
    let Some(live_end_next) =
        live_relation_for_source(bindings, candidate.end_half_edge_id, HalfEdgeNext)
    else {
        return false;
    };
    if start_prev.relation_id != live_start_prev.relation_id
        || end_next.relation_id != live_end_next.relation_id
    {
        return false;
    }

    let old_predecessor_half_edge_id = live_start_prev.target_half_edge_id;
    let old_successor_half_edge_id = live_end_next.target_half_edge_id;
    let new_predecessor_half_edge_id = start_prev.target_half_edge_id;
    let new_successor_half_edge_id = end_next.target_half_edge_id;
    let span_half_edge_ids = candidate
        .span_half_edge_ids
        .iter()
        .copied()
        .chain(std::iter::once(candidate.end_half_edge_id))
        .collect::<BTreeSet<_>>();
    if span_half_edge_ids.contains(&old_predecessor_half_edge_id)
        || span_half_edge_ids.contains(&old_successor_half_edge_id)
        || span_half_edge_ids.contains(&new_predecessor_half_edge_id)
        || span_half_edge_ids.contains(&new_successor_half_edge_id)
    {
        return false;
    }
    if !same_loop(
        bindings,
        candidate.start_half_edge_id,
        candidate.end_half_edge_id,
    ) || !same_loop(
        bindings,
        candidate.start_half_edge_id,
        new_predecessor_half_edge_id,
    ) || !same_loop(
        bindings,
        candidate.start_half_edge_id,
        new_successor_half_edge_id,
    ) {
        return false;
    }

    matches_expected_rewire(
        bindings,
        program.next.get(&old_predecessor_half_edge_id),
        old_predecessor_half_edge_id,
        HalfEdgeNext,
        old_successor_half_edge_id,
    ) && matches_expected_rewire(
        bindings,
        program.prev.get(&old_successor_half_edge_id),
        old_successor_half_edge_id,
        HalfEdgePrev,
        old_predecessor_half_edge_id,
    ) && matches_expected_rewire(
        bindings,
        program.next.get(&new_predecessor_half_edge_id),
        new_predecessor_half_edge_id,
        HalfEdgeNext,
        candidate.start_half_edge_id,
    ) && matches_expected_rewire(
        bindings,
        program.prev.get(&new_successor_half_edge_id),
        new_successor_half_edge_id,
        HalfEdgePrev,
        candidate.end_half_edge_id,
    )
}

fn matches_expected_rewire(
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

fn live_relation_for_source(
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
    let incoming_relation_ids =
        query_incoming_relation_ids(bindings, &targets[0], relation_kind).ok()?;
    let relation_id = incoming_relation_ids.into_iter().find(|relation_id| {
        query_relation_binding(bindings, *relation_id)
            .ok()
            .flatten()
            .is_some_and(|binding| {
                binding.kind == relation_kind
                    && binding.source_query_identity == source_binding.query_identity
                    && binding.target_query_identity == targets[0]
            })
    });
    let Some(relation_id) = relation_id else {
        return None;
    };
    Some(LiveLoopSuccessorRelation {
        relation_id,
        target_half_edge_id,
    })
}

fn same_loop(
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

struct DesiredLoopSuccessorProgram {
    next: BTreeMap<EntityId, DesiredLoopSuccessorRewire>,
    prev: BTreeMap<EntityId, DesiredLoopSuccessorRewire>,
}

struct DesiredLoopSuccessorRewire {
    relation_id: forge_relational::facade::identity::RelationId,
    target_half_edge_id: EntityId,
}

struct LiveLoopSuccessorRelation {
    relation_id: forge_relational::facade::identity::RelationId,
    target_half_edge_id: EntityId,
}

struct ContiguousSpanCandidate {
    start_half_edge_id: EntityId,
    end_half_edge_id: EntityId,
    span_half_edge_ids: Vec<EntityId>,
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::platform::authority::{CreateKey, EntityReference};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::*;

    fn entity_id(slot: u64) -> EntityId {
        EntityId::new(PartitionId::main(), slot, 1)
    }

    #[test]
    fn snapshot_support_denies_authoritative_create_even_without_bindings() {
        let support = TopologyRuntimeSupport::snapshot_read_only();
        let bindings = TopologyQueryBindingIndex::default();
        let contracts = [TopologyEditContract::create_topology_entity(
            "new-loop",
            TopologyEntityKind::Loop,
        )];
        let unsupported = unsupported_families(
            &support,
            &bindings,
            &[TopologyEditFamily::CreateTopologyEntity],
            &contracts,
        );

        assert_eq!(unsupported, vec![TopologyEditFamily::CreateTopologyEntity]);
    }

    #[test]
    fn current_head_support_admits_face_inner_loop_lane_from_contract_shape() {
        let support = TopologyRuntimeSupport::current_head_authoritative();
        let bindings = TopologyQueryBindingIndex::default();
        let contracts = [
            TopologyEditContract::create_topology_entity("new-loop", TopologyEntityKind::Loop),
            TopologyEditContract::attach_boundary_membership(
                "attach-inner-loop",
                crate::topology_operators::BoundaryMembershipKind::FaceInnerLoop,
                EntityReference::Existing(entity_id(1)),
                EntityReference::Created(CreateKey::new("new-loop")),
            ),
        ];
        let unsupported = unsupported_families(
            &support,
            &bindings,
            &[TopologyEditFamily::AttachBoundaryMembership],
            &contracts,
        );

        assert!(unsupported.is_empty());
    }

    #[test]
    fn current_head_support_keeps_shell_membership_lane_denied_without_bindings() {
        let support = TopologyRuntimeSupport::current_head_authoritative();
        let bindings = TopologyQueryBindingIndex::default();
        let contracts = [
            TopologyEditContract::create_topology_entity("new-wire", TopologyEntityKind::Wire),
            TopologyEditContract::attach_shell_or_wire_membership(
                "attach-wire-half-edge",
                crate::topology_operators::ShellOrWireMembershipKind::WireOwnsHalfEdge,
                EntityReference::Created(CreateKey::new("new-wire")),
                EntityReference::Existing(entity_id(2)),
            ),
        ];
        let unsupported = unsupported_families(
            &support,
            &bindings,
            &[TopologyEditFamily::AttachShellOrWireMembership],
            &contracts,
        );

        assert_eq!(
            unsupported,
            vec![TopologyEditFamily::AttachShellOrWireMembership]
        );
    }
}
