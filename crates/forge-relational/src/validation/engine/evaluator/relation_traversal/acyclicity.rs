use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::LoweredAcyclicityContract;
use crate::transactions::data::EntityReference;
use crate::validation::data::{InvariantClass, InvariantViolation, InvariantViolationFields};

use super::super::super::context::InvariantExecutionContext;
use super::planned_successors::{planned_successor_map, PlannedSuccessorMap};
use super::relation_successors::relation_kind_successors;
use super::traversal_budget::{traversal_budget_exceeded_violation, RelationTraversalBudget};

pub(in crate::validation::engine::evaluator) fn evaluate_acyclicity_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredAcyclicityContract,
) -> Option<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return None;
    };
    if scope.planned_edges.is_empty() {
        return None;
    }

    context.metrics().count_relation_contracts_evaluated(1);
    let planned_successors = planned_successor_map(&scope.planned_edges);
    for edge in &scope.planned_edges {
        context.metrics().count_relation_slot_scans(1);
        let reaches_cycle = if edge.source == edge.target {
            Ok(true)
        } else {
            relation_kind_reaches(
                context,
                class,
                &contract.contract_id,
                contract.relation_kind_id,
                edge.target.clone(),
                edge.source.clone(),
                &planned_successors,
            )
        };
        match reaches_cycle {
            Ok(true) => {
                return Some(InvariantViolation {
                    class,
                    code: DiagnosticCode::InvariantViolation,
                    detail: format!(
                        "acyclicity contract '{}' detected a cycle for relation kind {:?}",
                        contract.contract_id, contract.relation_kind_id
                    ),
                    fields: InvariantViolationFields::Acyclicity {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        source: edge.source.clone(),
                        target: edge.target.clone(),
                    },
                });
            }
            Ok(false) => {}
            Err(violation) => return Some(violation),
        }
    }
    None
}

fn relation_kind_reaches(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    start: EntityReference,
    target: EntityReference,
    planned_successors: &PlannedSuccessorMap,
) -> Result<bool, InvariantViolation> {
    let mut visited = std::collections::BTreeSet::new();
    let mut frontier = vec![start.clone()];
    let mut traversal_budget = RelationTraversalBudget::for_planned_successors(
        context.relation_integrity_scope_budget(),
        planned_successors,
    );

    visited.insert(start);
    traversal_budget.record_entity_visit().map_err(|_| {
        traversal_budget_exceeded_violation(
            class,
            contract_id,
            relation_kind_id,
            traversal_budget,
            planned_successors,
        )
    })?;

    while let Some(entity_id) = frontier.pop() {
        for next in relation_kind_successors(
            context,
            class,
            contract_id,
            relation_kind_id,
            &entity_id,
            planned_successors,
            &mut traversal_budget,
        )? {
            if next == target {
                return Ok(true);
            }
            if !visited.insert(next.clone()) {
                continue;
            }
            traversal_budget.record_entity_visit().map_err(|_| {
                traversal_budget_exceeded_violation(
                    class,
                    contract_id,
                    relation_kind_id,
                    traversal_budget,
                    planned_successors,
                )
            })?;
            frontier.push(next);
        }
    }

    Ok(false)
}
