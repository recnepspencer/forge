use crate::declaration::stable_text_digest;
use crate::runtime::allocation_planning::WorthUiAllocationPlanningLowering;
use crate::runtime::{
    WorthUiAllocationPlanningBasis, WorthUiAllocationPlanningCounters,
    WorthUiAllocationPlanningDenial, WorthUiPlanLoweringBasis, WorthUiPlanLoweringContext,
    WorthUiPlanNodeInput,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiAllocationPlanning {
    basis: WorthUiAllocationPlanningBasis,
    lowering: Option<WorthUiAllocationPlanningLowering>,
    planning_identity_digest: u64,
    denial_posture: Option<WorthUiAllocationPlanningDenial>,
    counters: WorthUiAllocationPlanningCounters,
}

impl WorthUiAllocationPlanning {
    pub(crate) fn new(
        basis: WorthUiAllocationPlanningBasis,
        lowering: Option<WorthUiAllocationPlanningLowering>,
        denial_posture: Option<WorthUiAllocationPlanningDenial>,
        counters: WorthUiAllocationPlanningCounters,
    ) -> Self {
        let planning_identity_digest =
            allocation_planning_identity_digest(&basis, lowering.as_ref(), denial_posture.as_ref());
        Self {
            basis,
            lowering,
            planning_identity_digest,
            denial_posture,
            counters,
        }
    }

    pub fn basis(&self) -> &WorthUiAllocationPlanningBasis {
        &self.basis
    }

    pub fn measurement_basis(&self) -> &crate::evidence::UiMeasurementBasis {
        self.basis.measurement_basis()
    }

    pub fn allocation_neighborhood(&self) -> &crate::evidence::UiAllocationNeighborhood {
        self.basis.allocation_neighborhood()
    }

    pub fn allocation_constraint_set(&self) -> Option<&crate::evidence::UiAllocationConstraintSet> {
        self.basis.allocation_constraint_set()
    }

    pub fn planning_identity_digest(&self) -> u64 {
        self.planning_identity_digest
    }

    pub fn denial_posture(&self) -> Option<&WorthUiAllocationPlanningDenial> {
        self.denial_posture.as_ref()
    }

    pub fn is_admitted(&self) -> bool {
        self.denial_posture.is_none() && self.lowering.is_some()
    }

    pub fn counters(&self) -> WorthUiAllocationPlanningCounters {
        self.counters
    }

    pub(crate) fn lowering_basis(&self) -> Option<&WorthUiPlanLoweringBasis> {
        self.lowering
            .as_ref()
            .map(WorthUiAllocationPlanningLowering::basis)
    }

    pub(crate) fn lowering_context(&self) -> Option<&WorthUiPlanLoweringContext> {
        self.lowering
            .as_ref()
            .map(WorthUiAllocationPlanningLowering::context)
    }

    pub(crate) fn node_inputs(&self) -> Option<&[WorthUiPlanNodeInput]> {
        self.lowering
            .as_ref()
            .map(WorthUiAllocationPlanningLowering::node_inputs)
    }

    pub(crate) fn lowered_input(&self) -> Option<crate::runtime::WorthUiExecutionPlanInput> {
        self.lowering
            .as_ref()
            .map(WorthUiAllocationPlanningLowering::execution_plan_input)
    }
}

fn allocation_planning_identity_digest(
    basis: &WorthUiAllocationPlanningBasis,
    lowering: Option<&WorthUiAllocationPlanningLowering>,
    denial_posture: Option<&WorthUiAllocationPlanningDenial>,
) -> u64 {
    let mut digest = stable_text_digest("worth-ui.runtime.allocation-planning")
        ^ basis.measurement_basis().identity_digest().rotate_left(7)
        ^ basis
            .allocation_neighborhood()
            .identity()
            .identity_digest()
            .rotate_left(13);

    if let Some(constraint_set) = basis.allocation_constraint_set() {
        digest ^= constraint_set.identity().identity_digest().rotate_left(17);
    }

    if let Some(lowering) = lowering {
        digest ^= lowering.basis().active_artifact_digest().rotate_left(19);
        digest ^= lowering.basis().candidate_artifact_digest().rotate_left(23);
        digest ^= lowering.basis().frame_epoch().as_u64().rotate_left(29);
        for (index, node_input) in lowering.node_inputs().iter().enumerate() {
            digest ^= (index as u64).rotate_left(3);
            digest ^= (node_input.family() as u64).rotate_left(11);
            digest ^= stable_text_digest(node_input.identity_basis()).rotate_left(17);
        }
    }

    if let Some(denial) = denial_posture {
        digest ^= (denial.reason() as u64).rotate_left(31);
    }

    digest
}
