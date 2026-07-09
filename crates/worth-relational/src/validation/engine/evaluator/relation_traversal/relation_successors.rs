use std::collections::BTreeSet;

use crate::transactions::data::EntityReference;
use crate::validation::data::{InvariantClass, InvariantViolation};

use super::super::super::context::InvariantExecutionContext;
use super::planned_successors::PlannedSuccessorMap;
use super::traversal_budget::{traversal_budget_exceeded_violation, RelationTraversalBudget};

pub(super) fn relation_kind_successors(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    entity_id: &EntityReference,
    planned_successors: &PlannedSuccessorMap,
    traversal_budget: &mut RelationTraversalBudget,
) -> Result<Vec<EntityReference>, InvariantViolation> {
    let mut successors = BTreeSet::new();
    collect_planned_successors(
        class,
        contract_id,
        relation_kind_id,
        entity_id,
        planned_successors,
        traversal_budget,
        &mut successors,
    )?;
    collect_visible_successors(
        context,
        class,
        contract_id,
        relation_kind_id,
        entity_id,
        planned_successors,
        traversal_budget,
        &mut successors,
    )?;
    Ok(successors.into_iter().collect())
}

fn collect_planned_successors(
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    entity_id: &EntityReference,
    planned_successors: &PlannedSuccessorMap,
    traversal_budget: &mut RelationTraversalBudget,
    successors: &mut BTreeSet<EntityReference>,
) -> Result<(), InvariantViolation> {
    let Some(edges) = planned_successors.get(entity_id) else {
        return Ok(());
    };
    for target in edges {
        traversal_budget.record_relation_scan().map_err(|_| {
            traversal_budget_exceeded_violation(
                class,
                contract_id,
                relation_kind_id,
                *traversal_budget,
                planned_successors,
            )
        })?;
        successors.insert(target.clone());
    }
    Ok(())
}

fn collect_visible_successors(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract_id: &crate::schema::data::ContractId,
    relation_kind_id: crate::identity::data::KindId,
    entity_id: &EntityReference,
    planned_successors: &PlannedSuccessorMap,
    traversal_budget: &mut RelationTraversalBudget,
    successors: &mut BTreeSet<EntityReference>,
) -> Result<(), InvariantViolation> {
    let EntityReference::Existing(entity_id) = entity_id else {
        return Ok(());
    };
    let Some(partition) = context
        .partition_access()
        .get_partition(entity_id.partition_id)
    else {
        return Ok(());
    };
    let slot = entity_id.slot_index();
    let outgoing = partition
        .adjacency
        .get(slot)
        .map(|set| set.as_slice())
        .into_iter()
        .flatten();
    for relation_id in outgoing.copied() {
        context.metrics().count_relation_slot_scans(1);
        traversal_budget.record_relation_scan().map_err(|_| {
            traversal_budget_exceeded_violation(
                class,
                contract_id,
                relation_kind_id,
                *traversal_budget,
                planned_successors,
            )
        })?;
        let Some(metadata) = context.state_view().relation_metadata(relation_id) else {
            continue;
        };
        if metadata.kind_id == relation_kind_id {
            successors.insert(EntityReference::Existing(metadata.target));
        }
    }
    Ok(())
}
