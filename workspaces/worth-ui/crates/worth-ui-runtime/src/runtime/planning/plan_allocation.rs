use std::borrow::Borrow;

use super::construct_verified_planning_input_handoff;
use crate::evidence::{UiConstraintPropagationDenial, UiMeasurementBasis};
use crate::runtime::allocation_planning::WorthUiAllocationPlanner;
use crate::runtime::launch::runtime_instance::WorthUiRuntime;
use crate::runtime::planning::UiAllocationCandidateMintAuthority;
use crate::runtime::{
    UiAllocationCandidate, WorthUiAllocationPlanning, WorthUiAllocationPlanningBasis,
    WorthUiAllocationPlanningCounters, WorthUiAllocationPlanningDenial,
    WorthUiAllocationPlanningDenialReason, WorthUiExecutionPlanInput, WorthUiPendingActivation,
    WorthUiPlanLoweringDenial,
};

pub(crate) enum ConstraintSetAdmissionDecision {
    Admitted(crate::graph::UiAdmittedAllocationConstraintBasis),
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
    match measurement_basis.admit_allocation_constraint_basis(allocation_neighborhood) {
        Ok(constraint_set) => ConstraintSetAdmissionDecision::Admitted(constraint_set),
        Err(denial) => ConstraintSetAdmissionDecision::Denied(denial),
    }
}

pub(crate) fn lower_execution_plan_for_planning<P: Borrow<WorthUiPendingActivation>>(
    host: &WorthUiRuntime,
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
) -> UiAllocationCandidate {
    let mut counters = WorthUiAllocationPlanningCounters::default();
    counters.record_planning_attempt();
    counters.record_measurement_basis_read();
    let basis = WorthUiAllocationPlanningBasis::denied(
        measurement_basis.clone(),
        allocation_neighborhood.clone(),
    );
    let denial_posture = Some(WorthUiAllocationPlanningDenial::new(
        WorthUiAllocationPlanningDenialReason::ConstraintSetDenied,
        None,
        Some(constraint_set_denial),
        None,
        None,
        counters,
    ));
    UiAllocationCandidate::from_planning(
        WorthUiAllocationPlanning::new(basis, None, denial_posture, counters),
        UiAllocationCandidateMintAuthority::new(),
    )
}

pub(crate) fn plan_allocation_for_pending_activation<P: Borrow<WorthUiPendingActivation>>(
    host: &WorthUiRuntime,
    pending_activation: P,
    measurement_basis: &UiMeasurementBasis,
    allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
) -> UiAllocationCandidate {
    let pending_activation = pending_activation.borrow();
    match classify_constraint_set_admission(measurement_basis, allocation_neighborhood) {
        ConstraintSetAdmissionDecision::Denied(constraint_set_denial) => {
            return build_constraint_set_denial_planning(
                measurement_basis,
                allocation_neighborhood,
                constraint_set_denial,
            );
        }
        ConstraintSetAdmissionDecision::Admitted(constraint_basis) => {
            let handoff =
                construct_verified_planning_input_handoff(pending_activation, constraint_basis)
                    .expect("constraint admission must preserve graph-planning alignment");
            match lower_execution_plan_for_planning(host, pending_activation) {
                PlanLoweringDecision::Lowered(lowered_input) => {
                    UiAllocationCandidate::from_planning(
                        WorthUiAllocationPlanner::plan_from_lowered_input(
                            handoff.into_admission(),
                            lowered_input,
                        ),
                        UiAllocationCandidateMintAuthority::new(),
                    )
                }
                PlanLoweringDecision::Denied(plan_lowering_denial) => {
                    UiAllocationCandidate::from_planning(
                        WorthUiAllocationPlanner::deny_from_plan_lowering(
                            measurement_basis,
                            allocation_neighborhood,
                            plan_lowering_denial,
                        ),
                        UiAllocationCandidateMintAuthority::new(),
                    )
                }
            }
        }
    }
}

pub(crate) fn replan_admitted_candidate(
    previous: &UiAllocationCandidate,
) -> Result<UiAllocationCandidate, WorthUiAllocationPlanningDenialReason> {
    replan_admitted_candidate_with_portal(previous, None)
}

fn replan_admitted_candidate_with_portal(
    previous: &UiAllocationCandidate,
    portal: Option<crate::runtime::UiPortalAllocationPlanningBasis>,
) -> Result<UiAllocationCandidate, WorthUiAllocationPlanningDenialReason> {
    let measurement_basis = previous.measurement_basis();
    let neighborhood = previous.allocation_neighborhood();
    let constraint_basis = portal
        .as_ref()
        .map_or_else(
            || measurement_basis.admit_allocation_constraint_basis(neighborhood),
            |portal| {
                measurement_basis
                    .admit_allocation_constraint_basis_with_portal(neighborhood, portal)
            },
        )
        .map_err(|_| WorthUiAllocationPlanningDenialReason::ConstraintSetDenied)?;
    let lowered_input = previous
        .planning()
        .lowered_input()
        .ok_or(WorthUiAllocationPlanningDenialReason::PlanLoweringDenied)?;
    let admission = crate::runtime::allocation_planning::WorthUiAllocationPlanningAdmission::from_execution_plan_input(
        &lowered_input,
        constraint_basis,
        portal,
    );
    Ok(UiAllocationCandidate::from_planning(
        WorthUiAllocationPlanner::plan_from_lowered_input(admission, lowered_input),
        UiAllocationCandidateMintAuthority::new(),
    ))
}

pub(crate) fn replan_selected_candidates(
    selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
) -> Result<Vec<UiAllocationCandidate>, u16> {
    selection
        .ordered_neighborhoods()
        .iter()
        .enumerate()
        .map(|(ordinal, selected)| {
            replan_admitted_candidate(selected.allocation_candidate()).map_err(|_| ordinal as u16)
        })
        .collect()
}

pub(crate) fn replan_selected_candidates_with_portal(
    selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
) -> Result<Vec<UiAllocationCandidate>, u16> {
    let consequences = selection.transaction_basis().consequences();
    selection
        .ordered_neighborhoods()
        .iter()
        .enumerate()
        .map(|(ordinal, selected)| {
            let mut matching = consequences.portal_anchors().iter().filter(|consequence| {
                consequence
                    .movement()
                    .target()
                    .primary()
                    .neighborhood_identity()
                    == selected.identity()
            });
            let first = matching.next();
            if matching.next().is_some() {
                return Err(ordinal as u16);
            }
            let portal = first
                .map(|consequence| {
                    crate::runtime::UiPortalAllocationPlanningBasis::seal(
                        consequence.movement(),
                        selected.identity(),
                    )
                    .ok_or(ordinal as u16)
                })
                .transpose()?;
            replan_admitted_candidate_with_portal(selected.allocation_candidate(), portal)
                .map_err(|_| ordinal as u16)
        })
        .collect()
}

pub(crate) fn replan_selected_candidates_with_resize(
    selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    basis: &crate::runtime::UiResizeAllocationPlanningBasis,
) -> Result<Vec<UiAllocationCandidate>, u16> {
    let mut candidates = replan_selected_candidates(selection)?;
    for candidate in &mut candidates {
        candidate.seal_resize_basis(basis.clone());
    }
    Ok(candidates)
}

#[cfg(test)]
pub(crate) fn candidate_from_test_planning(
    planning: WorthUiAllocationPlanning,
) -> UiAllocationCandidate {
    UiAllocationCandidate::from_planning(planning, UiAllocationCandidateMintAuthority::new())
}
