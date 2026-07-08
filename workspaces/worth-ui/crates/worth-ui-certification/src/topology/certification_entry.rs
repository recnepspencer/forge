use worth_ui_runtime::facade::evidence::{
    certify_allocation_planning_suite, UiAllocationPlanningCertificationReport,
    UiAllocationPlanningCertificationSuiteKind,
};
use worth_ui_test_support::planning_pair_for_certification_suite;

fn run_named_suite(
    kind: UiAllocationPlanningCertificationSuiteKind,
) -> UiAllocationPlanningCertificationReport {
    let (first, second) = planning_pair_for_certification_suite(kind);
    certify_allocation_planning_suite(kind, &first, &second)
}

pub fn allocation_neighborhood_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::AllocationNeighborhood)
}

pub fn constraint_edge_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::ConstraintEdge)
}

pub fn parent_child_propagation_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::ParentChildPropagation)
}

pub fn intrinsic_return_flow_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::IntrinsicReturnFlow)
}

pub fn sibling_negotiation_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::SiblingNegotiation)
}

pub fn equal_share_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::EqualShare)
}

pub fn bounded_reconciliation_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::BoundedReconciliation)
}

pub fn special_input_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::SpecialInput)
}

pub fn durable_resize_input_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::DurableResizeInput)
}

pub fn plan_handoff_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::PlanHandoff)
}

pub fn activation_boundary_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::ActivationBoundary)
}

pub fn allocation_inspection_suite() -> UiAllocationPlanningCertificationReport {
    run_named_suite(UiAllocationPlanningCertificationSuiteKind::AllocationInspection)
}
