use super::super::{
    ForgeQueryAdmittedBooleanPredicateExpression, ForgeQueryPredicateSelectivityClass,
};
use super::row::ForgeQueryBooleanPredicateSelectivityRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBooleanSelectivityCounters {
    predicate_rows_normalized: usize,
    expression_nodes_visited: usize,
    admitted_references_consulted: usize,
    branches_produced: usize,
    deduplicated_predicate_count: usize,
    traversal_predicate_count: usize,
    executor_observations_consumed: usize,
    exact_predicate_count: usize,
    broad_predicate_count: usize,
    risky_predicate_count: usize,
    pre_traversal_eligible_count: usize,
}

impl ForgeQueryBooleanSelectivityCounters {
    pub fn predicate_rows_normalized(&self) -> usize {
        self.predicate_rows_normalized
    }

    pub fn expression_nodes_visited(&self) -> usize {
        self.expression_nodes_visited
    }

    pub fn admitted_references_consulted(&self) -> usize {
        self.admitted_references_consulted
    }

    pub fn branches_produced(&self) -> usize {
        self.branches_produced
    }

    pub fn deduplicated_predicate_count(&self) -> usize {
        self.deduplicated_predicate_count
    }

    pub fn traversal_predicate_count(&self) -> usize {
        self.traversal_predicate_count
    }

    pub fn executor_observations_consumed(&self) -> usize {
        self.executor_observations_consumed
    }

    pub fn exact_predicate_count(&self) -> usize {
        self.exact_predicate_count
    }

    pub fn broad_predicate_count(&self) -> usize {
        self.broad_predicate_count
    }

    pub fn risky_predicate_count(&self) -> usize {
        self.risky_predicate_count
    }

    pub fn pre_traversal_eligible_count(&self) -> usize {
        self.pre_traversal_eligible_count
    }

    pub(crate) fn from_expression(
        expression: &ForgeQueryAdmittedBooleanPredicateExpression,
        rows: &[ForgeQueryBooleanPredicateSelectivityRow],
        deduplicated_predicate_count: usize,
    ) -> Self {
        Self {
            predicate_rows_normalized: rows.len(),
            expression_nodes_visited: expression.counters().expression_nodes_visited(),
            admitted_references_consulted: expression.counters().admitted_references_consulted(),
            branches_produced: expression.branches().len(),
            deduplicated_predicate_count,
            traversal_predicate_count: rows
                .iter()
                .filter(|row| {
                    row.selectivity_class()
                        == &ForgeQueryPredicateSelectivityClass::TraversalPredicate
                })
                .count(),
            executor_observations_consumed: 0,
            exact_predicate_count: rows
                .iter()
                .filter(|row| row.selectivity_class().is_exact_anchor())
                .count(),
            broad_predicate_count: rows
                .iter()
                .filter(|row| row.selectivity_class().is_broad_or_risky())
                .count(),
            risky_predicate_count: rows
                .iter()
                .filter(|row| {
                    row.selectivity_class()
                        == &ForgeQueryPredicateSelectivityClass::UnknownPredicate
                })
                .count(),
            pre_traversal_eligible_count: rows
                .iter()
                .filter(|row| row.is_pre_traversal_eligible())
                .count(),
        }
    }
}
