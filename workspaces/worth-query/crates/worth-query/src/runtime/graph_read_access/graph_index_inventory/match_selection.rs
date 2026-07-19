use super::{
    WorthQueryGraphIndexInventory, WorthQueryGraphIndexInventoryMatchOutcome,
    WorthQueryGraphIndexSupportRow,
};
use crate::runtime::WorthQueryGraphReadAccessRequirementRow;

pub(crate) fn classify_inventory_match_outcome(
    requirement: &WorthQueryGraphReadAccessRequirementRow,
    row: &WorthQueryGraphIndexSupportRow,
) -> WorthQueryGraphIndexInventoryMatchOutcome {
    if row.supported_relation_direction().is_some()
        && row.supported_relation_direction() != requirement.relation_direction()
    {
        return WorthQueryGraphIndexInventoryMatchOutcome::DirectionMismatch;
    }
    if row.supported_predicate_family().is_some()
        && row.supported_predicate_family() != requirement.predicate_family()
    {
        return WorthQueryGraphIndexInventoryMatchOutcome::PredicateMismatch;
    }
    if row.supported_ordering_posture().is_some()
        && row.supported_ordering_posture() != requirement.ordering_posture()
    {
        return WorthQueryGraphIndexInventoryMatchOutcome::OrderingMismatch;
    }
    if row.supported_requirement_lifecycle().is_some()
        && row.supported_requirement_lifecycle() != requirement.lifecycle_class()
    {
        return WorthQueryGraphIndexInventoryMatchOutcome::LifecycleMismatch;
    }
    if row.rebuild_basis() != requirement.rebuild_basis() {
        return WorthQueryGraphIndexInventoryMatchOutcome::RebuildBasisMismatch;
    }
    if row.invalidation_basis() != requirement.invalidation_basis() {
        return WorthQueryGraphIndexInventoryMatchOutcome::InvalidationBasisMismatch;
    }
    if row.complexity_contract() != requirement.complexity_contract() {
        return WorthQueryGraphIndexInventoryMatchOutcome::ComplexityMismatch;
    }
    WorthQueryGraphIndexInventoryMatchOutcome::ExactMatch
}

pub(crate) fn select_best_support_row_for_requirement<'a>(
    requirement: &WorthQueryGraphReadAccessRequirementRow,
    inventory: &'a WorthQueryGraphIndexInventory,
) -> Option<&'a WorthQueryGraphIndexSupportRow> {
    let rows = inventory.rows_for_requirement_kind(requirement.kind());
    rows.iter()
        .copied()
        .find(|row| {
            classify_inventory_match_outcome(requirement, row)
                == WorthQueryGraphIndexInventoryMatchOutcome::ExactMatch
        })
        .or_else(|| rows.first().copied())
}
