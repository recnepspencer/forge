use crate::authority::commit::preparation::planning::strategy::{
    strategy_for_parallel_packets, PreparationStrategySelection,
};
use crate::query::data::{
    QueryParallelLegality, QueryParallelProfitability, SnapshotPinnedQueryPlan,
};

use super::super::VisibilityReadContext;

pub(in crate::visibility::materialization::read_records::reader) fn query_execution_strategy(
    reader: &VisibilityReadContext<'_>,
    plan: &SnapshotPinnedQueryPlan,
    packet_count: usize,
) -> PreparationStrategySelection {
    if !matches!(plan.legality, QueryParallelLegality::LegalReadOnlySnapshot) {
        return PreparationStrategySelection::Serial;
    }
    if !matches!(plan.profitability, QueryParallelProfitability::Profitable) {
        return PreparationStrategySelection::Serial;
    }

    strategy_for_parallel_packets(
        reader.runtime().config.execution.execution_model,
        packet_count,
    )
    .selected_mode
}
