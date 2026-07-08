use std::borrow::Borrow;

use crate::evidence::{
    UiAllocationConstraintSet, UiConstraintPropagationDenial, UiMeasurementBasis,
};
use crate::runtime::allocation_planning::WorthUiAllocationPlanner;
use super::construct_verified_planning_input_handoff;
use crate::runtime::launch::host::WorthUiRuntimeHost;
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiAllocationPlanningBasis, WorthUiAllocationPlanningCounters,
    WorthUiAllocationPlanningDenial, WorthUiAllocationPlanningDenialReason, WorthUiExecutionPlanInput,
    WorthUiPendingActivation, WorthUiPlanLoweringDenial,
};

use super::measurement_basis::collect_planning_measurement_basis;

pub(crate) enum ConstraintSetAdmissionDecision {
    Admitted(UiAllocationConstraintSet),
    Denied(UiConstraintPropagationDenial),
}

pub(crate) enum PlanLoweringDecision {
    Lowered(WorthUiExecutionPlanInput),
    Denied(WorthUiPlanLoweringDenial),
}

pub(crate) fn classify_constraint_set_admission(
    measurement_basis: &UiMeasurementBasis,
    allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
) -> ConstraintSetAdmissionDecision {
    match measurement_basis.admit_allocation_constraint_set(allocation_neighborhood) {
        Ok(constraint_set) => ConstraintSetAdmissionDecision::Admitted(constraint_set),
        Err(denial) => ConstraintSetAdmissionDecision::Denied(denial),
    }
}

pub(crate) fn lower_execution_plan_for_planning<P: Borrow<WorthUiPendingActivation>>(
    host: &WorthUiRuntimeHost,
    pending_activation: P,
) -> PlanLoweringDecision {
    match host.prepare_execution_plan_input(pending_activation) {
        Ok(lowered_input) => PlanLoweringDecision::Lowered(lowered_input),
        Err(plan_lowering_denial) => PlanLoweringDecision::Denied(plan_lowering_denial),
    }
}

pub(crate) fn build_constraint_set_denial_planning(
    measurement_basis: &UiMeasurementBasis,
    allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
    constraint_set_denial: UiConstraintPropagationDenial,
) -> WorthUiAllocationPlanning {
    let mut counters = WorthUiAllocationPlanningCounters::default();
    counters.record_planning_attempt();
    counters.record_measurement_basis_read();
    let basis = WorthUiAllocationPlanningBasis::new(
        measurement_basis.clone(),
        allocation_neighborhood.clone(),
        None,
    );
    let denial_posture = Some(WorthUiAllocationPlanningDenial::new(
        WorthUiAllocationPlanningDenialReason::ConstraintSetDenied,
        None,
        Some(constraint_set_denial),
        None,
        None,
        counters,
    ));
    WorthUiAllocationPlanning::new(basis, None, denial_posture, counters)
}

pub(crate) fn plan_allocation_for_pending_activation<P: Borrow<WorthUiPendingActivation>>(
    host: &WorthUiRuntimeHost,
    pending_activation: P,
    measurement_basis: &UiMeasurementBasis,
    allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
) -> WorthUiAllocationPlanning {
    let pending_activation = pending_activation.borrow();
    let measurement_basis = collect_planning_measurement_basis(
        measurement_basis,
        allocation_neighborhood,
        pending_activation
            .staged_replacement()
            .reconciliation_plan()
            .durable_resize_inputs(),
    );
    match classify_constraint_set_admission(&measurement_basis, allocation_neighborhood) {
        ConstraintSetAdmissionDecision::Denied(constraint_set_denial) => {
            return build_constraint_set_denial_planning(
                &measurement_basis,
                allocation_neighborhood,
                constraint_set_denial,
            );
        }
        ConstraintSetAdmissionDecision::Admitted(constraint_set) => {
            let handoff = construct_verified_planning_input_handoff(
                pending_activation,
                &measurement_basis,
                allocation_neighborhood,
                &constraint_set,
            )
            .expect("constraint admission must preserve graph-planning alignment");
            match lower_execution_plan_for_planning(host, pending_activation) {
                PlanLoweringDecision::Lowered(lowered_input) => {
                    WorthUiAllocationPlanner::plan_from_lowered_input(
                        handoff.into_admission(),
                        lowered_input,
                    )
                }
                PlanLoweringDecision::Denied(plan_lowering_denial) => {
                    WorthUiAllocationPlanner::deny_from_plan_lowering(
                        &measurement_basis,
                        allocation_neighborhood,
                        plan_lowering_denial,
                    )
                }
            }
        }
    }
}