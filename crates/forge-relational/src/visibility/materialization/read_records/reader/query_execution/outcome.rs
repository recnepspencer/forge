use crate::logic::runtime::RelationalRuntime;
use crate::query::data::{
    reduce_query_fragments, QueryComplexitySummary, QueryExecutionOutcome, SnapshotPinnedQueryPlan,
};

pub(in crate::visibility::materialization::read_records::reader) fn query_execution_outcome(
    runtime: &RelationalRuntime,
    plan: SnapshotPinnedQueryPlan,
    packet_count: usize,
    touched_partitions: usize,
    target_count: usize,
    fragments: Vec<crate::query::data::QueryWorkerFragment>,
) -> QueryExecutionOutcome {
    let unmasked_entity_records_emitted = fragments
        .iter()
        .map(|fragment| fragment.counters.unmasked_entity_records_emitted)
        .sum();
    let unmasked_relation_records_emitted = fragments
        .iter()
        .map(|fragment| fragment.counters.unmasked_relation_records_emitted)
        .sum();
    let complexity = QueryComplexitySummary {
        packet_count,
        fragment_count: packet_count,
        touched_partitions,
        target_count,
        unmasked_entity_records_emitted,
        unmasked_relation_records_emitted,
    };
    let result =
        reduce_query_fragments(plan.packet.execution_shape, plan.packet.ordering, fragments);
    runtime
        .performance_access()
        .count_query_emissions(result.entities.len(), result.relations.len());

    QueryExecutionOutcome {
        plan,
        result,
        complexity,
    }
}
