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
    let authoritative_entity_records_emitted = entities.len();
    scratch.remember_entity_capacity(authoritative_entity_records_emitted);
    crate::query::data::QueryWorkerFragment {
        plan_key: packet.plan_key,
        fragment_key: crate::query::data::deterministic_query_fragment_key(
            packet.plan_key,
            ordinal,
        ),
        ordering: packet.ordering,
        counters: crate::query::data::QueryFragmentCounters {
            target_count,
            authoritative_entity_records_emitted,
            authoritative_relation_records_emitted: 0,
            touched_partitions: usize::from(authoritative_entity_records_emitted > 0)
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
    let authoritative_relation_records_emitted = relations.len();
    scratch.remember_relation_capacity(authoritative_relation_records_emitted);
    crate::query::data::QueryWorkerFragment {
        plan_key: packet.plan_key,
        fragment_key: crate::query::data::deterministic_query_fragment_key(
            packet.plan_key,
            ordinal,
        ),
        ordering: packet.ordering,
        counters: crate::query::data::QueryFragmentCounters {
            target_count,
            authoritative_entity_records_emitted: 0,
            authoritative_relation_records_emitted,
            touched_partitions: usize::from(authoritative_relation_records_emitted > 0)
                * touched_partitions,
        },
        entities: Vec::new(),
        relations,
        traversal_basis: None,
    }
}
