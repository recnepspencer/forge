use crate::evidence::{UiAllocationConstraintSet, UiAllocationNeighborhood, UiMeasurementBasis};
use crate::graph::UiGraphGeneration;
use crate::runtime::allocation_planning::WorthUiAllocationPlanningAdmission;
use crate::runtime::WorthUiPendingActivation;

/// Witness that graph-admitted inputs align before planning admission is consumed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPlanningInputHandoffWitness {
    graph_generation: UiGraphGeneration,
    neighborhood_identity_digest: u64,
    constraint_set_identity_digest: u64,
    measurement_basis_identity_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiPlanningInputHandoffDenial {
    BasisNeighborhoodMismatch,
    ConstraintNeighborhoodMismatch,
}

/// Verified graph→planning handoff: witness must be present before admission is consumed.
#[derive(Debug, Clone)]
pub(crate) struct WorthUiVerifiedPlanningInputHandoff {
    _witness: WorthUiPlanningInputHandoffWitness,
    admission: WorthUiAllocationPlanningAdmission,
}

impl WorthUiVerifiedPlanningInputHandoff {
    pub(crate) fn into_admission(self) -> WorthUiAllocationPlanningAdmission {
        self.admission
    }
}

pub(crate) fn verify_planning_input_alignment(
    measurement_basis: &UiMeasurementBasis,
    allocation_neighborhood: &UiAllocationNeighborhood,
    constraint_set: &UiAllocationConstraintSet,
) -> Result<WorthUiPlanningInputHandoffWitness, WorthUiPlanningInputHandoffDenial> {
    if measurement_basis.identity_digest()
        != allocation_neighborhood.measurement_basis_identity_digest()
    {
        return Err(WorthUiPlanningInputHandoffDenial::BasisNeighborhoodMismatch);
    }
    if constraint_set.neighborhood_identity_digest()
        != allocation_neighborhood.identity().identity_digest()
    {
        return Err(WorthUiPlanningInputHandoffDenial::ConstraintNeighborhoodMismatch);
    }
    Ok(WorthUiPlanningInputHandoffWitness {
        graph_generation: allocation_neighborhood.graph_generation(),
        neighborhood_identity_digest: allocation_neighborhood.identity().identity_digest(),
        constraint_set_identity_digest: constraint_set.identity().identity_digest(),
        measurement_basis_identity_digest: measurement_basis.identity_digest(),
    })
}

pub(crate) fn construct_verified_planning_input_handoff(
    pending_activation: &WorthUiPendingActivation,
    measurement_basis: &UiMeasurementBasis,
    allocation_neighborhood: &UiAllocationNeighborhood,
    constraint_set: &UiAllocationConstraintSet,
) -> Result<WorthUiVerifiedPlanningInputHandoff, WorthUiPlanningInputHandoffDenial> {
    let witness = verify_planning_input_alignment(
        measurement_basis,
        allocation_neighborhood,
        constraint_set,
    )?;
    Ok(WorthUiVerifiedPlanningInputHandoff {
        _witness: witness,
        admission: WorthUiAllocationPlanningAdmission::from_pending_activation(
            pending_activation,
            measurement_basis,
            allocation_neighborhood,
            constraint_set,
        ),
    })
}
