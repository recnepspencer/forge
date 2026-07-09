use crate::runtime::WorthUiAllocationPlanning;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationPlanningDeterminismPosture {
    Equivalent,
    Divergent,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAllocationPlanningCertificationSuiteKind {
    AllocationNeighborhood,
    ConstraintEdge,
    ParentChildPropagation,
    IntrinsicReturnFlow,
    SiblingNegotiation,
    EqualShare,
    BoundedReconciliation,
    SpecialInput,
    DurableResizeInput,
    PlanHandoff,
    ActivationBoundary,
    AllocationInspection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationPlanningCertificationReport {
    determinism_posture: UiAllocationPlanningDeterminismPosture,
    suite_kind: Option<UiAllocationPlanningCertificationSuiteKind>,
    suite_verified: bool,
    neighborhood_identity_matches: bool,
    constraint_set_identity_matches: bool,
    neighborhood_width_bounded: bool,
    emitted_edges_match: bool,
    solve_trace_converges: bool,
    remainder_policy_matches: bool,
    resize_posture_matches: bool,
    special_inputs_match: bool,
    cost_class_matches: bool,
    handoff_identity_matches: bool,
    inspection_receipts_match: bool,
    denied_broadening_matches: bool,
}

pub fn certify_allocation_planning_determinism(
    first: &WorthUiAllocationPlanning,
    second: &WorthUiAllocationPlanning,
) -> UiAllocationPlanningCertificationReport {
    certification_report(first, second, None)
}

pub fn certify_allocation_planning_suite(
    suite_kind: UiAllocationPlanningCertificationSuiteKind,
    first: &WorthUiAllocationPlanning,
    second: &WorthUiAllocationPlanning,
) -> UiAllocationPlanningCertificationReport {
    let report = certification_report(first, second, Some(suite_kind));
    UiAllocationPlanningCertificationReport {
        suite_verified: suite_contract_satisfied(suite_kind, &report),
        ..report
    }
}

impl UiAllocationPlanningCertificationReport {
    pub fn determinism_posture(&self) -> UiAllocationPlanningDeterminismPosture {
        self.determinism_posture
    }

    pub fn is_equivalent(&self) -> bool {
        self.determinism_posture == UiAllocationPlanningDeterminismPosture::Equivalent
    }

    pub fn suite_kind(&self) -> Option<UiAllocationPlanningCertificationSuiteKind> {
        self.suite_kind
    }

    pub fn suite_verified(&self) -> bool {
        self.suite_verified
    }

    pub fn neighborhood_identity_matches(&self) -> bool {
        self.neighborhood_identity_matches
    }

    pub fn constraint_set_identity_matches(&self) -> bool {
        self.constraint_set_identity_matches
    }

    pub fn neighborhood_width_is_bounded(&self) -> bool {
        self.neighborhood_width_bounded
    }

    pub fn emitted_edges_match(&self) -> bool {
        self.emitted_edges_match
    }

    pub fn solve_trace_converges(&self) -> bool {
        self.solve_trace_converges
    }

    pub fn remainder_policy_matches(&self) -> bool {
        self.remainder_policy_matches
    }

    pub fn resize_posture_matches(&self) -> bool {
        self.resize_posture_matches
    }

    pub fn special_inputs_match(&self) -> bool {
        self.special_inputs_match
    }

    pub fn cost_class_matches(&self) -> bool {
        self.cost_class_matches
    }

    pub fn handoff_identity_matches(&self) -> bool {
        self.handoff_identity_matches
    }

    pub fn inspection_receipts_match(&self) -> bool {
        self.inspection_receipts_match
    }

    pub fn denied_broadening_matches(&self) -> bool {
        self.denied_broadening_matches
    }
}

fn certification_report(
    first: &WorthUiAllocationPlanning,
    second: &WorthUiAllocationPlanning,
    suite_kind: Option<UiAllocationPlanningCertificationSuiteKind>,
) -> UiAllocationPlanningCertificationReport {
    let first_receipt = crate::evidence::project_allocation_planning_inspection_receipt(first);
    let second_receipt = crate::evidence::project_allocation_planning_inspection_receipt(second);
    let first_cost = first_receipt.cost();
    let second_cost = second_receipt.cost();
    let first_trace = first_receipt.solve_trace();
    let second_trace = second_receipt.solve_trace();

    UiAllocationPlanningCertificationReport {
        determinism_posture: if equivalent_planning(
            first,
            second,
            &first_cost,
            &second_cost,
            first_trace,
            second_trace,
        ) {
            UiAllocationPlanningDeterminismPosture::Equivalent
        } else {
            UiAllocationPlanningDeterminismPosture::Divergent
        },
        suite_kind,
        suite_verified: false,
        neighborhood_identity_matches: first.allocation_neighborhood().identity().identity_digest()
            == second
                .allocation_neighborhood()
                .identity()
                .identity_digest(),
        constraint_set_identity_matches: first
            .allocation_constraint_set()
            .map(|set| set.identity().identity_digest())
            == second
                .allocation_constraint_set()
                .map(|set| set.identity().identity_digest()),
        neighborhood_width_bounded: first_cost.nodes_admitted() <= first_cost.nodes_considered()
            && second_cost.nodes_admitted() <= second_cost.nodes_considered(),
        emitted_edges_match: first_cost.edges_emitted() == second_cost.edges_emitted(),
        solve_trace_converges: first_trace.is_deterministic() && second_trace.is_deterministic(),
        remainder_policy_matches: first_trace.remainder_policy() == second_trace.remainder_policy(),
        resize_posture_matches: first_trace.resize_permission_posture()
            == second_trace.resize_permission_posture(),
        special_inputs_match: first_cost.special_inputs_loaded()
            == second_cost.special_inputs_loaded(),
        cost_class_matches: first_cost.cost_class() == second_cost.cost_class(),
        handoff_identity_matches: first.is_admitted() == second.is_admitted()
            && first
                .lowering_basis()
                .map(|basis| basis.active_artifact_digest())
                == second
                    .lowering_basis()
                    .map(|basis| basis.active_artifact_digest()),
        inspection_receipts_match: first_receipt == second_receipt,
        denied_broadening_matches: first_cost.denied_broadening_reason()
            == second_cost.denied_broadening_reason(),
    }
}

pub(crate) fn suite_contract_satisfied(
    suite_kind: UiAllocationPlanningCertificationSuiteKind,
    report: &UiAllocationPlanningCertificationReport,
) -> bool {
    match suite_kind {
        UiAllocationPlanningCertificationSuiteKind::AllocationNeighborhood => {
            report.is_equivalent()
                && report.neighborhood_identity_matches()
                && report.neighborhood_width_is_bounded()
        }
        UiAllocationPlanningCertificationSuiteKind::ConstraintEdge => {
            report.is_equivalent()
                && report.constraint_set_identity_matches()
                && report.emitted_edges_match()
        }
        UiAllocationPlanningCertificationSuiteKind::ParentChildPropagation => {
            report.is_equivalent()
                && report.constraint_set_identity_matches()
                && report.emitted_edges_match()
                && report.neighborhood_width_is_bounded()
        }
        UiAllocationPlanningCertificationSuiteKind::IntrinsicReturnFlow => {
            report.is_equivalent()
                && report.emitted_edges_match()
                && report.neighborhood_width_is_bounded()
        }
        UiAllocationPlanningCertificationSuiteKind::SiblingNegotiation => {
            report.is_equivalent()
                && report.constraint_set_identity_matches()
                && report.neighborhood_width_is_bounded()
        }
        UiAllocationPlanningCertificationSuiteKind::EqualShare => {
            report.is_equivalent()
                && report.remainder_policy_matches()
                && report.constraint_set_identity_matches()
        }
        UiAllocationPlanningCertificationSuiteKind::BoundedReconciliation => {
            report.is_equivalent()
                && report.neighborhood_width_is_bounded()
                && report.denied_broadening_matches()
        }
        UiAllocationPlanningCertificationSuiteKind::SpecialInput => {
            report.is_equivalent() && report.special_inputs_match() && report.cost_class_matches()
        }
        UiAllocationPlanningCertificationSuiteKind::DurableResizeInput => {
            report.is_equivalent() && report.resize_posture_matches() && report.cost_class_matches()
        }
        UiAllocationPlanningCertificationSuiteKind::PlanHandoff => {
            report.is_equivalent() && report.handoff_identity_matches()
        }
        UiAllocationPlanningCertificationSuiteKind::ActivationBoundary => {
            report.is_equivalent()
                && report.handoff_identity_matches()
                && report.cost_class_matches()
        }
        UiAllocationPlanningCertificationSuiteKind::AllocationInspection => {
            report.is_equivalent() && report.inspection_receipts_match()
        }
    }
}

fn equivalent_planning(
    first: &WorthUiAllocationPlanning,
    second: &WorthUiAllocationPlanning,
    first_cost: &crate::evidence::UiAllocationPlanningCostReceipt,
    second_cost: &crate::evidence::UiAllocationPlanningCostReceipt,
    first_trace: &crate::evidence::UiAllocationSolveTrace,
    second_trace: &crate::evidence::UiAllocationSolveTrace,
) -> bool {
    first.planning_identity_digest() == second.planning_identity_digest()
        && first.measurement_basis().identity_digest()
            == second.measurement_basis().identity_digest()
        && first.allocation_neighborhood().identity().identity_digest()
            == second
                .allocation_neighborhood()
                .identity()
                .identity_digest()
        && first
            .allocation_constraint_set()
            .map(|constraint_set| constraint_set.identity().identity_digest())
            == second
                .allocation_constraint_set()
                .map(|constraint_set| constraint_set.identity().identity_digest())
        && first.denial_posture() == second.denial_posture()
        && first_cost == second_cost
        && first_trace == second_trace
}
