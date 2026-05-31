use super::*;

#[derive(Debug, Clone, Copy)]
pub(super) enum TraversalMode {
    OutgoingNeighborhood,
    IncomingNeighborhood,
    ConnectivityTraversal { max_depth: Option<u32> },
}

pub(super) fn aspect_filter_matches_entity(
    record: &EntityReadRecord,
    aspect_filter: &ProjectionAspectFilter,
) -> bool {
    aspect_filter.matches_authoritative_state(record.authoritative_aspect_state.as_ref())
}

pub(super) fn aspect_filter_matches_relation(
    record: &RelationReadRecord,
    aspect_filter: &ProjectionAspectFilter,
) -> bool {
    aspect_filter.matches_authoritative_state(record.authoritative_aspect_state.as_ref())
}

pub(super) fn traversal_fragment(
    runtime: &RelationalRuntime,
    state: &impl PartitionAccess,
    version_id: crate::identity::data::VersionId,
    packet: &PlannedQueryPacket,
    seeds: &[crate::identity::data::EntityId],
    relation_kind_scope: Option<&[crate::identity::data::KindId]>,
    ordinal: u64,
    mode: TraversalMode,
    scratch: &mut QueryFragmentScratch,
) -> Option<crate::query::data::QueryWorkerFragment> {
    if packet.ordering != QueryOrderingContract::CanonicalTraversalOrder {
        return None;
    }

    let relation_kind_scope =
        relation_kind_scope.map(|scope| scope.iter().copied().collect::<BTreeSet<_>>());
    scratch.reset_traversal();
    let mut entities = scratch.entity_buffer();
    let mut relations = scratch.relation_buffer();
    let mut entity_visit_keys = scratch.entity_visit_key_buffer();
    let mut relation_visit_keys = scratch.relation_visit_key_buffer();

    for seed in seeds.iter().copied() {
        if scratch.visited_entities_mut().insert(seed) {
            scratch.frontier_mut().push_back((seed, 0u32, seed, None));
        }
    }

    while let Some((entity_id, depth, root_seed, via_relation)) = scratch.frontier_mut().pop_front()
    {
        let Some(entity_record) = runtime
            .read_truth()
            .unmasked_entity_record_for_id_at_version(state, entity_id, version_id)
        else {
            continue;
        };
        entity_visit_keys.push(crate::query::data::TraversalEntityVisitKey {
            depth,
            root_seed,
            via_relation,
            entity_id,
        });
        entities.push(entity_record);

        let relation_ids = relation_ids_for_traversal(
            runtime,
            state,
            version_id,
            entity_id,
            &mode,
            relation_kind_scope.as_ref(),
        );
        let allow_expansion = match mode {
            TraversalMode::OutgoingNeighborhood | TraversalMode::IncomingNeighborhood => depth == 0,
            TraversalMode::ConnectivityTraversal { max_depth } => {
                max_depth.is_none_or(|max_depth| depth < max_depth)
            }
        };
        if !allow_expansion {
            continue;
        }

        for relation_id in relation_ids {
            let Some(relation_record) = runtime
                .read_truth()
                .unmasked_relation_record_for_id_at_version(state, relation_id, version_id)
            else {
                continue;
            };
            if scratch
                .emitted_relations_mut()
                .insert(relation_record.relation_id)
            {
                relation_visit_keys.push(crate::query::data::TraversalRelationVisitKey {
                    depth,
                    root_seed,
                    relation_id: relation_record.relation_id,
                });
                relations.push(relation_record.clone());
            }

            let neighbor = match mode {
                TraversalMode::OutgoingNeighborhood
                | TraversalMode::ConnectivityTraversal { .. } => relation_record.target,
                TraversalMode::IncomingNeighborhood => relation_record.source,
            };
            if scratch.visited_entities_mut().insert(neighbor) {
                scratch.frontier_mut().push_back((
                    neighbor,
                    depth + 1,
                    root_seed,
                    Some(relation_record.relation_id),
                ));
            }
        }
    }
    scratch.remember_entity_capacity(entities.len());
    scratch.remember_relation_capacity(relations.len());
    scratch.remember_entity_visit_key_capacity(entity_visit_keys.len());
    scratch.remember_relation_visit_key_capacity(relation_visit_keys.len());
    let touched_partitions = entities
        .iter()
        .map(|record| record.entity_id.partition_id)
        .collect::<BTreeSet<_>>()
        .len();

    Some(crate::query::data::QueryWorkerFragment {
        plan_key: packet.plan_key,
        fragment_key: crate::query::data::deterministic_query_fragment_key(
            packet.plan_key,
            ordinal,
        ),
        ordering: packet.ordering,
        counters: crate::query::data::QueryFragmentCounters {
            target_count: seeds.len(),
            unmasked_entity_records_emitted: entities.len(),
            unmasked_relation_records_emitted: relations.len(),
            touched_partitions,
        },
        entities,
        relations,
        traversal_basis: Some(crate::query::data::TraversalReductionBasis {
            entity_visit_keys,
            relation_visit_keys,
        }),
    })
}

fn relation_ids_for_traversal(
    runtime: &RelationalRuntime,
    state: &impl PartitionAccess,
    version_id: crate::identity::data::VersionId,
    entity_id: crate::identity::data::EntityId,
    mode: &TraversalMode,
    relation_kind_scope: Option<&BTreeSet<crate::identity::data::KindId>>,
) -> Vec<crate::identity::data::RelationId> {
    let storage = runtime.storage_access();
    let mut relation_ids = match mode {
        TraversalMode::OutgoingNeighborhood | TraversalMode::ConnectivityTraversal { .. } => {
            storage.outgoing_relations_for_entity(entity_id, version_id)
        }
        TraversalMode::IncomingNeighborhood => {
            storage.incoming_relations_for_entity(entity_id, version_id)
        }
    };
    relation_ids.sort();
    relation_ids.retain(|relation_id| {
        let Some(relation_record) = runtime
            .read_truth()
            .unmasked_relation_record_for_id_at_version(state, *relation_id, version_id)
        else {
            return false;
        };
        relation_kind_scope.is_none_or(|scope| scope.contains(&relation_record.kind.kind_id))
    });
    relation_ids
}
