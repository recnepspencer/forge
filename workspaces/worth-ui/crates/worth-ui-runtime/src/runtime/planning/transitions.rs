use std::borrow::Borrow;

use crate::evidence::{UiAllocationNeighborhood, UiMeasurementBasis};
use crate::graph::UiAllocationNeighborhoodDenial;
use crate::runtime::WorthUiPendingActivation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPlanningLaneReadiness {
    Ready,
    BasisNeighborhoodMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPlanningLaneInputDenial {
    BasisNeighborhoodMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiPlanningLaneAdmissionDenial {
    AllocationNeighborhood(UiAllocationNeighborhoodDenial),
    BasisNeighborhoodMismatch,
    StalePendingActivation {
        active_frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
        pending_frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
    },
}

impl From<UiAllocationNeighborhoodDenial> for WorthUiPlanningLaneAdmissionDenial {
    fn from(denial: UiAllocationNeighborhoodDenial) -> Self {
        Self::AllocationNeighborhood(denial)
    }
}

impl From<WorthUiPlanningLaneInputDenial> for WorthUiPlanningLaneAdmissionDenial {
    fn from(denial: WorthUiPlanningLaneInputDenial) -> Self {
        match denial {
            WorthUiPlanningLaneInputDenial::BasisNeighborhoodMismatch => {
                Self::BasisNeighborhoodMismatch
            }
        }
    }
}

/// Planning lane entry proof: allocation planning requires staged pending activation.
#[derive(Debug, Clone)]
pub struct WorthUiPlanningLaneInput<P> {
    pub(crate) pending_activation: P,
    pub(crate) measurement_basis: UiMeasurementBasis,
    pub(crate) allocation_neighborhood: UiAllocationNeighborhood,
}

pub fn classify_planning_lane_readiness(
    measurement_basis: &UiMeasurementBasis,
    allocation_neighborhood: &UiAllocationNeighborhood,
) -> WorthUiPlanningLaneReadiness {
    if measurement_basis.identity_digest()
        != allocation_neighborhood.measurement_basis_identity_digest()
    {
        return WorthUiPlanningLaneReadiness::BasisNeighborhoodMismatch;
    }
    WorthUiPlanningLaneReadiness::Ready
}

pub fn construct_planning_lane_input<P>(
    pending_activation: P,
    measurement_basis: UiMeasurementBasis,
    allocation_neighborhood: UiAllocationNeighborhood,
) -> Result<WorthUiPlanningLaneInput<P>, WorthUiPlanningLaneInputDenial>
where
    P: Borrow<WorthUiPendingActivation>,
{
    match classify_planning_lane_readiness(&measurement_basis, &allocation_neighborhood) {
        WorthUiPlanningLaneReadiness::Ready => Ok(WorthUiPlanningLaneInput {
            pending_activation,
            measurement_basis,
            allocation_neighborhood,
        }),
        WorthUiPlanningLaneReadiness::BasisNeighborhoodMismatch => {
            Err(WorthUiPlanningLaneInputDenial::BasisNeighborhoodMismatch)
        }
    }
}

impl<P> WorthUiPlanningLaneInput<P>
where
    P: Borrow<WorthUiPendingActivation>,
{
    pub fn pending_activation(&self) -> &WorthUiPendingActivation {
        self.pending_activation.borrow()
    }

    pub fn measurement_basis(&self) -> &UiMeasurementBasis {
        &self.measurement_basis
    }

    pub fn allocation_neighborhood(&self) -> &UiAllocationNeighborhood {
        &self.allocation_neighborhood
    }
}
