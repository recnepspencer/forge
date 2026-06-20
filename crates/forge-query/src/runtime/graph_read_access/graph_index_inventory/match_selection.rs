use super::{
    ForgeQueryGraphIndexInventory, ForgeQueryGraphIndexInventoryMatchOutcome,
    ForgeQueryGraphIndexSupportRow,
};
use crate::runtime::ForgeQueryGraphReadAccessRequirementRow;

pub(crate) fn classify_inventory_match_outcome(
    requirement: &ForgeQueryGraphReadAccessRequirementRow,
    row: &ForgeQueryGraphIndexSupportRow,
) -> ForgeQueryGraphIndexInventoryMatchOutcome {
    if row.supported_relation_direction().is_some()
        && row.supported_relation_direction() != requirement.relation_direction()
    {
        return ForgeQueryGraphIndexInventoryMatchOutcome::DirectionMismatch;
    }
    if row.supported_predicate_family().is_some()
        && row.supported_predicate_family() != requirement.predicate_family()
    {
        return ForgeQueryGraphIndexInventoryMatchOutcome::PredicateMismatch;
    }
    if row.supported_ordering_posture().is_some()
        && row.supported_ordering_posture() != requirement.ordering_posture()
    {
        return ForgeQueryGraphIndexInventoryMatchOutcome::OrderingMismatch;
    }
    if row.supported_requirement_lifecycle().is_some()
        && row.supported_requirement_lifecycle() != requirement.lifecycle_class()
    {
        return ForgeQueryGraphIndexInventoryMatchOutcome::LifecycleMismatch;
    }
    if row.rebuild_basis() != requirement.rebuild_basis() {
        return ForgeQueryGraphIndexInventoryMatchOutcome::RebuildBasisMismatch;
    }
    if row.invalidation_basis() != requirement.invalidation_basis() {
        return ForgeQueryGraphIndexInventoryMatchOutcome::InvalidationBasisMismatch;
    }
    if row.complexity_contract() != requirement.complexity_contract() {
        return ForgeQueryGraphIndexInventoryMatchOutcome::ComplexityMismatch;
    }
    ForgeQueryGraphIndexInventoryMatchOutcome::ExactMatch
}

pub(crate) fn select_best_support_row_for_requirement<'a>(
    requirement: &ForgeQueryGraphReadAccessRequirementRow,
    inventory: &'a ForgeQueryGraphIndexInventory,
) -> Option<&'a ForgeQueryGraphIndexSupportRow> {
    let rows = inventory.rows_for_requirement_kind(requirement.kind());
    rows.iter()
        .copied()
        .find(|row| {
            classify_inventory_match_outcome(requirement, row)
                == ForgeQueryGraphIndexInventoryMatchOutcome::ExactMatch
        })
        .or_else(|| rows.first().copied())
}
