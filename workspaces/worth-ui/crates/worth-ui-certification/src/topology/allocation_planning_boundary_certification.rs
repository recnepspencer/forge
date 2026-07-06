use std::path::Path;

use worth_ui_runtime::facade::{
    certify_activation_boundary_suite as runtime_activation_boundary_suite,
    certify_allocation_inspection_suite as runtime_allocation_inspection_suite,
    certify_allocation_neighborhood_suite as runtime_allocation_neighborhood_suite,
    certify_bounded_reconciliation_suite as runtime_bounded_reconciliation_suite,
    certify_constraint_edge_suite as runtime_constraint_edge_suite,
    certify_durable_resize_input_suite as runtime_durable_resize_input_suite,
    certify_equal_share_suite as runtime_equal_share_suite,
    certify_intrinsic_return_flow_suite as runtime_intrinsic_return_flow_suite,
    certify_parent_child_propagation_suite as runtime_parent_child_propagation_suite,
    certify_plan_handoff_suite as runtime_plan_handoff_suite,
    certify_sibling_negotiation_suite as runtime_sibling_negotiation_suite,
    certify_special_input_suite as runtime_special_input_suite,
    UiAllocationPlanningCertificationReport,
};
#[cfg(test)]
use worth_ui_runtime::facade::UiAllocationPlanningCertificationSuiteKind;

use super::allocation_planning_anti_bypass_audit::audit_allocation_planning_anti_bypass_boundaries;

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

pub fn allocation_neighborhood_suite() -> UiAllocationPlanningCertificationReport {
    runtime_allocation_neighborhood_suite()
}

pub fn constraint_edge_suite() -> UiAllocationPlanningCertificationReport {
    runtime_constraint_edge_suite()
}

pub fn parent_child_propagation_suite() -> UiAllocationPlanningCertificationReport {
    runtime_parent_child_propagation_suite()
}

pub fn intrinsic_return_flow_suite() -> UiAllocationPlanningCertificationReport {
    runtime_intrinsic_return_flow_suite()
}

pub fn sibling_negotiation_suite() -> UiAllocationPlanningCertificationReport {
    runtime_sibling_negotiation_suite()
}

pub fn equal_share_suite() -> UiAllocationPlanningCertificationReport {
    runtime_equal_share_suite()
}

pub fn bounded_reconciliation_suite() -> UiAllocationPlanningCertificationReport {
    runtime_bounded_reconciliation_suite()
}

pub fn special_input_suite() -> UiAllocationPlanningCertificationReport {
    runtime_special_input_suite()
}

pub fn durable_resize_input_suite() -> UiAllocationPlanningCertificationReport {
    runtime_durable_resize_input_suite()
}

pub fn plan_handoff_suite() -> UiAllocationPlanningCertificationReport {
    runtime_plan_handoff_suite()
}

pub fn activation_boundary_suite() -> UiAllocationPlanningCertificationReport {
    runtime_activation_boundary_suite()
}

pub fn allocation_inspection_suite() -> UiAllocationPlanningCertificationReport {
    runtime_allocation_inspection_suite()
}

pub fn certify_allocation_anti_bypass_boundaries(
    workspace_root: &Path,
) -> Result<(), Vec<String>> {
    let violations = audit_allocation_planning_anti_bypass_boundaries(workspace_root);
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        activation_boundary_suite, allocation_inspection_suite, allocation_neighborhood_suite,
        bounded_reconciliation_suite, certify_allocation_anti_bypass_boundaries,
        constraint_edge_suite, durable_resize_input_suite, equal_share_suite,
        intrinsic_return_flow_suite, NAMED_PLANNING_SUITE_KINDS,
        parent_child_propagation_suite, plan_handoff_suite, sibling_negotiation_suite,
        special_input_suite,
    };
    use std::collections::BTreeSet;
    use std::path::Path;
    use worth_ui_runtime::facade::UiAllocationPlanningCertificationSuiteKind;

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
    fn named_planning_certification_suites_run_real_runtime_scenarios() {
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
        for (report, kind) in reports.into_iter().zip(NAMED_PLANNING_SUITE_KINDS) {
            assert_eq!(report.suite_kind(), Some(kind), "wrong suite kind for {kind:?}");
            assert!(report.suite_verified(), "suite contract failed for {kind:?}: {report:?}");
            assert!(report.is_equivalent(), "planning diverged for {kind:?}: {report:?}");
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

    #[test]
    fn workspace_passes_allocation_anti_bypass_boundary_certification() {
        let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        certify_allocation_anti_bypass_boundaries(&workspace_root)
            .expect("workspace should keep allocation planning semantics in the admitted lane");
    }
}
