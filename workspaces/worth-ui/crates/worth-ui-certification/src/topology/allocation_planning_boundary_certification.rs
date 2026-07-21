use super::allocation_planning_anti_bypass_audit::audit_allocation_planning_anti_bypass_boundaries;
use super::workspace_source_inventory::WorkspaceSourceInventory;

#[cfg(test)]
use worth_ui_runtime::facade::evidence::UiAllocationPlanningCertificationSuiteKind;

#[cfg(test)]
const NAMED_PLANNING_SUITE_KINDS: [UiAllocationPlanningCertificationSuiteKind; 12] = [
    UiAllocationPlanningCertificationSuiteKind::AllocationNeighborhood,
    UiAllocationPlanningCertificationSuiteKind::ConstraintEdge,
    UiAllocationPlanningCertificationSuiteKind::ParentChildPropagation,
    UiAllocationPlanningCertificationSuiteKind::IntrinsicReturnFlow,
    UiAllocationPlanningCertificationSuiteKind::SiblingNegotiation,
    UiAllocationPlanningCertificationSuiteKind::EqualShare,
    UiAllocationPlanningCertificationSuiteKind::BoundedReconciliation,
    UiAllocationPlanningCertificationSuiteKind::SpecialInput,
    UiAllocationPlanningCertificationSuiteKind::DurableResizeInput,
    UiAllocationPlanningCertificationSuiteKind::PlanHandoff,
    UiAllocationPlanningCertificationSuiteKind::ActivationBoundary,
    UiAllocationPlanningCertificationSuiteKind::AllocationInspection,
];

pub use super::certification_entry::{
    activation_boundary_suite, allocation_inspection_suite, allocation_neighborhood_suite,
    bounded_reconciliation_suite, constraint_edge_suite, durable_resize_input_suite,
    equal_share_suite, intrinsic_return_flow_suite, parent_child_propagation_suite,
    plan_handoff_suite, sibling_negotiation_suite, special_input_suite,
};

pub fn certify_allocation_anti_bypass_boundaries(
    inventory: &WorkspaceSourceInventory,
) -> Result<(), Vec<String>> {
    let violations = audit_allocation_planning_anti_bypass_boundaries(inventory);
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        certify_allocation_anti_bypass_boundaries, WorkspaceSourceInventory,
        NAMED_PLANNING_SUITE_KINDS,
    };
    use std::collections::BTreeSet;
    use std::path::Path;
    use worth_ui_runtime::facade::evidence::UiAllocationPlanningCertificationSuiteKind;

    #[test]
    fn named_planning_certification_suites_bind_distinct_suite_kinds() {
        let unique: BTreeSet<UiAllocationPlanningCertificationSuiteKind> =
            NAMED_PLANNING_SUITE_KINDS.into_iter().collect();
        assert_eq!(NAMED_PLANNING_SUITE_KINDS.len(), 12);
        assert_eq!(unique.len(), 12);
        assert_eq!(
            NAMED_PLANNING_SUITE_KINDS[0],
            UiAllocationPlanningCertificationSuiteKind::AllocationNeighborhood
        );
        assert_eq!(
            NAMED_PLANNING_SUITE_KINDS[11],
            UiAllocationPlanningCertificationSuiteKind::AllocationInspection
        );
    }

    #[test]
    fn workspace_passes_allocation_anti_bypass_boundary_certification() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = WorkspaceSourceInventory::capture(workspace_root);
        certify_allocation_anti_bypass_boundaries(&inventory)
            .expect("workspace should keep allocation planning semantics in the admitted lane");
    }
}
