use crate::declaration::UiDeclaredMeasurementBasisSource;
use crate::evidence::UiAllocationPlanningCertificationSuiteKind;
use crate::runtime::WorthUiAllocationPlanning;

use super::certification_fixture::{
    planning_pair_from_runtime_fixture, CertificationScenarioShape,
};

pub fn planning_pair_for_certification_suite(
    kind: UiAllocationPlanningCertificationSuiteKind,
) -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    match kind {
        UiAllocationPlanningCertificationSuiteKind::AllocationNeighborhood => {
            allocation_neighborhood_pair()
        }
        UiAllocationPlanningCertificationSuiteKind::ConstraintEdge => constraint_edge_pair(),
        UiAllocationPlanningCertificationSuiteKind::ParentChildPropagation => {
            parent_child_propagation_pair()
        }
        UiAllocationPlanningCertificationSuiteKind::IntrinsicReturnFlow => {
            intrinsic_return_flow_pair()
        }
        UiAllocationPlanningCertificationSuiteKind::SiblingNegotiation => sibling_negotiation_pair(),
        UiAllocationPlanningCertificationSuiteKind::EqualShare => equal_share_pair(),
        UiAllocationPlanningCertificationSuiteKind::BoundedReconciliation => {
            bounded_reconciliation_pair()
        }
        UiAllocationPlanningCertificationSuiteKind::SpecialInput => special_input_pair(),
        UiAllocationPlanningCertificationSuiteKind::DurableResizeInput => {
            durable_resize_input_pair()
        }
        UiAllocationPlanningCertificationSuiteKind::PlanHandoff => plan_handoff_pair(),
        UiAllocationPlanningCertificationSuiteKind::ActivationBoundary => {
            activation_boundary_pair()
        }
        UiAllocationPlanningCertificationSuiteKind::AllocationInspection => {
            allocation_inspection_pair()
        }
    }
}

fn allocation_neighborhood_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:stack",
        CertificationScenarioShape::Control {
            nodes: 1,
            bounded: true,
        },
        None,
    )
}

fn constraint_edge_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:overlay",
        CertificationScenarioShape::Control {
            nodes: 2,
            bounded: true,
        },
        None,
    )
}

fn parent_child_propagation_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:row",
        CertificationScenarioShape::Control {
            nodes: 2,
            bounded: true,
        },
        None,
    )
}

fn intrinsic_return_flow_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:row",
        CertificationScenarioShape::Intrinsic {
            nodes: 2,
            bounded: true,
        },
        None,
    )
}

fn sibling_negotiation_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:row",
        CertificationScenarioShape::Peer {
            nodes: 3,
            bounded: true,
        },
        None,
    )
}

fn equal_share_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:grid",
        CertificationScenarioShape::Peer {
            nodes: 3,
            bounded: false,
        },
        None,
    )
}

fn bounded_reconciliation_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:split",
        CertificationScenarioShape::Intrinsic {
            nodes: 4,
            bounded: true,
        },
        None,
    )
}

fn special_input_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:scroll",
        CertificationScenarioShape::Control {
            nodes: 1,
            bounded: true,
        },
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport),
    )
}

fn durable_resize_input_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:split",
        CertificationScenarioShape::Control {
            nodes: 1,
            bounded: true,
        },
        None,
    )
}

fn plan_handoff_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:split",
        CertificationScenarioShape::Peer {
            nodes: 3,
            bounded: true,
        },
        None,
    )
}

fn activation_boundary_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:overlay",
        CertificationScenarioShape::Peer {
            nodes: 2,
            bounded: true,
        },
        None,
    )
}

fn allocation_inspection_pair() -> (WorthUiAllocationPlanning, WorthUiAllocationPlanning) {
    planning_pair_from_runtime_fixture(
        "operator:portal-anchor",
        CertificationScenarioShape::Control {
            nodes: 1,
            bounded: true,
        },
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor),
    )
}
