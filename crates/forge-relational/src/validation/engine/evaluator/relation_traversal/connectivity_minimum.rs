use std::collections::BTreeSet;

use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::LoweredConnectivityMinimumContract;
use crate::transactions::data::EntityReference;
use crate::validation::data::{InvariantClass, InvariantViolation, InvariantViolationFields};

use super::super::super::context::InvariantExecutionContext;
use super::super::common::{contract_candidate_kind_matches, entity_reference_kind};
use super::planned_successors::{planned_successor_map, PlannedSuccessorMap};
use super::relation_successors::relation_kind_successors;
use super::traversal_budget::{traversal_budget_exceeded_violation, RelationTraversalBudget};
use super::visible_entities::visible_entities_of_kinds;

pub(in crate::validation::engine::evaluator) fn evaluate_connectivity_minimum_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredConnectivityMinimumContract,
) -> Option<InvariantViolation> {
    context.metrics().count_relation_contracts_evaluated(1);
    let source_entities = visible_entities_of_kinds(context, &contract.source_kind_ids);
    if source_entities.is_empty() {
        return None;
    }

    let planned_successors = context
        .relation_integrity_scope(contract.relation_kind_id)
        .map(|scope| planned_successor_map(&scope.planned_edges))
        .unwrap_or_default();
    for source in source_entities {
        let reachable_target_count = match reachable_target_count_for_connectivity(
            context,
            class,
            &contract.contract_id,
            contract.relation_kind_id,
            source.clone(),
            &contract.target_kind_ids,
            &planned_successors,
        ) {
            Ok(count) => count,
            Err(violation) => return Some(violation),
        };
        if reachable_target_count < contract.minimum_reachable_targets as usize {
            return Some(InvariantViolation {
                class,
                code: DiagnosticCode::InvariantViolation,
                detail: format!(
                    "connectivity minimum contract '{}' requires at least {} reachable target(s) for {:?}",
                    contract.contract_id,
                    contract.minimum_reachable_targets,
                    source
                ),
                fields: InvariantViolationFields::ConnectivityMinimum {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source,
                    reachable_target_count,
                    minimum_reachable_targets: contract.minimum_reachable_targets,
                },
            });
        }
    }
    None
}

fn reachable_target_count_for_connectivity(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    source: EntityReference,
    target_kind_ids: &[crate::identity::data::KindId],
    planned_successors: &PlannedSuccessorMap,
) -> Result<usize, InvariantViolation> {
    let mut visited = BTreeSet::new();
    let mut frontier = vec![source.clone()];
    let mut reachable_targets = BTreeSet::new();
    let mut traversal_budget = RelationTraversalBudget::for_planned_successors(
        context.relation_integrity_scope_budget(),
        planned_successors,
    );

    visited.insert(source);
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
            if let Some(kind_id) = entity_reference_kind(context, class, &next)? {
                if contract_candidate_kind_matches(kind_id, target_kind_ids) {
                    reachable_targets.insert(next.clone());
                }
            }
            frontier.push(next);
        }
    }

    Ok(reachable_targets.len())
}
