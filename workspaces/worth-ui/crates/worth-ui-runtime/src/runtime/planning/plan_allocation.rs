use std::borrow::Borrow;

use super::construct_verified_planning_input_handoff;
use super::WorthUiAllocationReplanDenial;
use crate::evidence::{UiConstraintPropagationDenial, UiMeasurementBasis};
use crate::runtime::allocation_planning::WorthUiAllocationPlanner;
use crate::runtime::planning::UiAllocationCandidateMintAuthority;
use crate::runtime::{
    UiAllocationCandidate, WorthUiAllocationPlanning, WorthUiAllocationPlanningBasis,
    WorthUiAllocationPlanningCounters, WorthUiAllocationPlanningDenial,
    WorthUiAllocationPlanningDenialReason, WorthUiPendingActivation,
};

pub(crate) enum ConstraintSetAdmissionDecision {
    Admitted(crate::graph::UiAdmittedAllocationConstraintBasis),
    Denied(UiConstraintPropagationDenial),
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
        counters,
    ));
    UiAllocationCandidate::from_planning(
        WorthUiAllocationPlanning::new(basis, None, denial_posture, counters),
        UiAllocationCandidateMintAuthority::new(),
    )
}

pub(crate) fn plan_allocation_for_pending_activation<P: Borrow<WorthUiPendingActivation>>(
    pending_activation: P,
    measurement_basis: &UiMeasurementBasis,
    allocation_neighborhood: &crate::evidence::UiAllocationNeighborhood,
) -> UiAllocationCandidate {
    let pending_activation = pending_activation.borrow();
    match classify_constraint_set_admission(measurement_basis, allocation_neighborhood) {
        ConstraintSetAdmissionDecision::Denied(constraint_set_denial) => {
            build_constraint_set_denial_planning(
                measurement_basis,
                allocation_neighborhood,
                constraint_set_denial,
            )
        }
        ConstraintSetAdmissionDecision::Admitted(constraint_basis) => {
            let handoff =
                construct_verified_planning_input_handoff(pending_activation, constraint_basis)
                    .expect("constraint admission must preserve graph-planning alignment");
            UiAllocationCandidate::from_planning(
                WorthUiAllocationPlanner::plan(handoff.into_admission()),
                UiAllocationCandidateMintAuthority::new(),
            )
        }
    }
}

pub(crate) fn replan_admitted_candidate(
    previous: &UiAllocationCandidate,
) -> Result<UiAllocationCandidate, WorthUiAllocationReplanDenial> {
    replan_admitted_candidate_with_portal(previous, None)
}

fn replan_admitted_candidate_with_portal(
    previous: &UiAllocationCandidate,
    portal: Option<crate::runtime::UiPortalAllocationPlanningBasis>,
) -> Result<UiAllocationCandidate, WorthUiAllocationReplanDenial> {
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
        .map_err(|_| WorthUiAllocationReplanDenial::ConstraintSetDenied)?;
    let projection = previous
        .planning()
        .projection()
        .cloned()
        .ok_or(WorthUiAllocationReplanDenial::CandidateProjectionUnavailable)?;
    let admission =
        crate::runtime::allocation_planning::WorthUiAllocationPlanningAdmission::from_projection(
            projection,
            constraint_basis,
            portal,
        );
    Ok(UiAllocationCandidate::from_planning(
        WorthUiAllocationPlanner::plan(admission),
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
