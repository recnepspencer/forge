use super::super::*;

pub(super) fn entity_fragment(
    read_view: &RelationalReadView,
    packet: &PlannedQueryPacket,
    ordinal: u64,
    target_count: usize,
    touched_partitions: usize,
    scratch: &mut QueryFragmentScratch,
    include: impl Fn(&EntityReadRecord) -> bool,
) -> crate::query::data::QueryWorkerFragment {
    let mut entities = scratch.entity_buffer();
    for record in read_view.entities() {
        if include(record) {
            entities.push(record.clone());
        }
    }
    let unmasked_entity_records_emitted = entities.len();
    scratch.remember_entity_capacity(unmasked_entity_records_emitted);
    crate::query::data::QueryWorkerFragment {
        plan_key: packet.plan_key,
        fragment_key: crate::query::data::deterministic_query_fragment_key(
            packet.plan_key,
            ordinal,
        ),
        ordering: packet.ordering,
        counters: crate::query::data::QueryFragmentCounters {
            target_count,
            unmasked_entity_records_emitted,
            unmasked_relation_records_emitted: 0,
            touched_partitions: usize::from(unmasked_entity_records_emitted > 0)
                * touched_partitions,
        },
        entities,
        relations: Vec::new(),
        traversal_basis: None,
    }
}

pub(super) fn relation_fragment(
    read_view: &RelationalReadView,
    packet: &PlannedQueryPacket,
    ordinal: u64,
    target_count: usize,
    touched_partitions: usize,
    scratch: &mut QueryFragmentScratch,
    include: impl Fn(&RelationReadRecord) -> bool,
) -> crate::query::data::QueryWorkerFragment {
    let mut relations = scratch.relation_buffer();
    for record in read_view.relations() {
        if include(record) {
            relations.push(record.clone());
        }
    }
    let unmasked_relation_records_emitted = relations.len();
    scratch.remember_relation_capacity(unmasked_relation_records_emitted);
    crate::query::data::QueryWorkerFragment {
        plan_key: packet.plan_key,
        fragment_key: crate::query::data::deterministic_query_fragment_key(
            packet.plan_key,
            ordinal,
        ),
        ordering: packet.ordering,
        counters: crate::query::data::QueryFragmentCounters {
            target_count,
            unmasked_entity_records_emitted: 0,
            unmasked_relation_records_emitted,
            touched_partitions: usize::from(unmasked_relation_records_emitted > 0)
                * touched_partitions,
        },
        entities: Vec::new(),
        relations,
        traversal_basis: None,
    }
}
