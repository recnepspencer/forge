use worth_ui_certification::topology::{
    activation_boundary_suite, allocation_inspection_suite, allocation_neighborhood_suite,
    bounded_reconciliation_suite, constraint_edge_suite, durable_resize_input_suite,
    equal_share_suite, intrinsic_return_flow_suite, parent_child_propagation_suite,
    plan_handoff_suite, sibling_negotiation_suite, special_input_suite,
};
use worth_ui_runtime::facade::evidence::UiAllocationPlanningCertificationSuiteKind;

#[test]
fn named_planning_certification_suites_run_real_runtime_scenarios() {
    let kinds = [
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
    let reports = [
        allocation_neighborhood_suite(),
        constraint_edge_suite(),
        parent_child_propagation_suite(),
        intrinsic_return_flow_suite(),
        sibling_negotiation_suite(),
        equal_share_suite(),
        bounded_reconciliation_suite(),
        special_input_suite(),
        durable_resize_input_suite(),
        plan_handoff_suite(),
        activation_boundary_suite(),
        allocation_inspection_suite(),
    ];
    for (report, kind) in reports.into_iter().zip(kinds) {
        assert_eq!(report.suite_kind(), Some(kind));
        assert!(report.suite_verified(), "suite failed: {report:?}");
        assert!(report.is_equivalent(), "planning diverged: {report:?}");
        match kind {
            UiAllocationPlanningCertificationSuiteKind::AllocationNeighborhood => {
                assert!(report.neighborhood_identity_matches());
                assert!(report.neighborhood_width_is_bounded());
            }
            UiAllocationPlanningCertificationSuiteKind::ConstraintEdge => {
                assert!(report.constraint_set_identity_matches());
                assert!(report.emitted_edges_match());
            }
            UiAllocationPlanningCertificationSuiteKind::EqualShare => {
                assert!(report.remainder_policy_matches());
            }
            UiAllocationPlanningCertificationSuiteKind::SpecialInput => {
                assert!(report.special_inputs_match());
                assert!(report.cost_class_matches());
            }
            UiAllocationPlanningCertificationSuiteKind::PlanHandoff => {
                assert!(report.handoff_identity_matches());
            }
            UiAllocationPlanningCertificationSuiteKind::AllocationInspection => {
                assert!(report.inspection_receipts_match());
            }
            _ => assert!(report.solve_trace_converges() || report.is_equivalent()),
        }
    }
}
