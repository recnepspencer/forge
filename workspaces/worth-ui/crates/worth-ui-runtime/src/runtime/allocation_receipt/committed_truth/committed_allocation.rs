use crate::runtime::planning::allocation_planning::WorthUiAllocationPlanningProjection;
use crate::runtime::WorthUiAllocationPlanningBasis;

/// Lowered allocation payload owned by a committed receipt.
///
/// Planning may be discarded after commit. Execution consumers receive this
/// immutable payload, never the candidate planning object.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiCommittedAllocation {
    basis: WorthUiAllocationPlanningBasis,
    planning_projection: WorthUiAllocationPlanningProjection,
    allocation_identity_digest: u64,
    resize_basis: Option<crate::runtime::UiResizeAllocationPlanningBasis>,
}

impl UiCommittedAllocation {
    pub(super) fn from_candidate(candidate: &super::UiAllocationCandidate) -> Self {
        let planning = candidate.planning();
        let planning_projection = planning
            .projection()
            .expect("only admitted planning can become committed allocation")
            .clone();
        Self {
            basis: planning.basis().clone(),
            planning_projection,
            allocation_identity_digest: candidate.planning_identity_digest(),
            resize_basis: candidate.resize_basis().cloned(),
        }
    }

    pub(crate) fn allocation_identity_digest(&self) -> u64 {
        self.allocation_identity_digest
    }
    pub(crate) fn resize_basis(&self) -> Option<&crate::runtime::UiResizeAllocationPlanningBasis> {
        self.resize_basis.as_ref()
    }
    pub(crate) fn measurement_basis(&self) -> &crate::evidence::UiMeasurementBasis {
        self.basis.measurement_basis()
    }
    pub(crate) fn allocation_neighborhood(&self) -> &crate::evidence::UiAllocationNeighborhood {
        self.basis.allocation_neighborhood()
    }
    pub(crate) fn planning_basis(&self) -> &WorthUiAllocationPlanningBasis {
        &self.basis
    }

    pub(crate) fn planning_projection(&self) -> &WorthUiAllocationPlanningProjection {
        &self.planning_projection
    }
}
