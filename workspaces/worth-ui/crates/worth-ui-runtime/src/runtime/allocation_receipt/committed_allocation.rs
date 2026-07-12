use crate::runtime::{
    WorthUiAllocationPlanningBasis, WorthUiPlanLoweringBasis, WorthUiPlanNodeInput,
};

/// Lowered allocation payload owned by a committed receipt.
///
/// Planning may be discarded after commit. Execution consumers receive this
/// immutable payload, never the candidate planning object.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiCommittedAllocation {
    basis: WorthUiAllocationPlanningBasis,
    lowering_basis: WorthUiPlanLoweringBasis,
    node_inputs: Box<[WorthUiPlanNodeInput]>,
    allocation_identity_digest: u64,
    resize_basis: Option<crate::runtime::UiResizeAllocationPlanningBasis>,
}

impl UiCommittedAllocation {
    pub(super) fn from_candidate(candidate: &super::UiAllocationCandidate) -> Self {
        let planning = candidate.planning();
        let lowering_basis = planning
            .lowering_basis()
            .expect("only admitted planning can become committed allocation")
            .clone();
        let node_inputs = planning
            .node_inputs()
            .expect("only admitted planning can become committed allocation")
            .to_vec()
            .into_boxed_slice();
        Self {
            basis: planning.basis().clone(),
            lowering_basis,
            node_inputs,
            allocation_identity_digest: candidate.planning_identity_digest(),
            resize_basis: candidate.resize_basis().cloned(),
        }
    }

    pub(crate) fn lowering_basis(&self) -> &WorthUiPlanLoweringBasis {
        &self.lowering_basis
    }
    pub(crate) fn node_inputs(&self) -> &[WorthUiPlanNodeInput] {
        &self.node_inputs
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
}
