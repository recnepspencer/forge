use crate::diagnostics::data::DiagnosticCode;
use crate::validation::data::{InvariantClass, InvariantViolation, InvariantViolationFields};

use super::planned_successors::{planned_successor_count, PlannedSuccessorMap};

#[derive(Debug, Clone, Copy)]
pub(super) struct RelationTraversalBudget {
    max_relation_scans: usize,
    relation_scans: usize,
    visited_entities: usize,
}

impl RelationTraversalBudget {
    pub(super) fn for_planned_successors(
        budget: &crate::config::data::RelationIntegrityScopeBudget,
        planned_successors: &PlannedSuccessorMap,
    ) -> Self {
        let planned_edge_count = planned_successor_count(planned_successors);
        Self {
            max_relation_scans: budget
                .max_scanned_relations
                .saturating_add(planned_edge_count),
            relation_scans: 0,
            visited_entities: 0,
        }
    }

    pub(super) fn record_relation_scan(&mut self) -> Result<(), ()> {
        self.relation_scans = self.relation_scans.saturating_add(1);
        if self.relation_scans > self.max_relation_scans {
            return Err(());
        }
        Ok(())
    }

    pub(super) fn record_entity_visit(&mut self) -> Result<(), ()> {
        self.visited_entities = self.visited_entities.saturating_add(1);
        let max_visited_entities = self.max_relation_scans.saturating_add(1);
        if self.visited_entities > max_visited_entities {
            return Err(());
        }
        Ok(())
    }
}

pub(super) fn traversal_budget_exceeded_violation(
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    traversal_budget: RelationTraversalBudget,
    planned_successors: &PlannedSuccessorMap,
) -> InvariantViolation {
    InvariantViolation {
        class,
        code: DiagnosticCode::InvariantViolation,
        detail: format!(
            "relation contract '{}' exceeded evaluator traversal budget for relation kind {:?}",
            contract_id, relation_kind_id
        ),
        fields: InvariantViolationFields::RelationIntegrityScopeBudgetExceeded {
            limit_name: "max_scanned_relations".to_string(),
            limit: traversal_budget.max_relation_scans,
            observed: traversal_budget.relation_scans,
            relation_kind_count: 1,
            touched_entity_count: traversal_budget.visited_entities,
            deleted_entity_count: 0,
            scanned_relation_count: traversal_budget.relation_scans,
            planned_edge_count: planned_successor_count(planned_successors),
        },
    }
}
