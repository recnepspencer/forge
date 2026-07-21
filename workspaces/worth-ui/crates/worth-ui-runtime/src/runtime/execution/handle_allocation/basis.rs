use crate::runtime::planning::WorthUiExecutionPlanLoweringFacts;
use crate::runtime::WorthUiRuntimeFrameEpoch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeHandleAllocationBasis {
    prior_artifact_digest: Option<u64>,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    plan_node_input_count: usize,
    query_binding_input_count: usize,
    allocation_planning_identity_digest: u64,
}

impl WorthUiRuntimeHandleAllocationBasis {
    pub(crate) fn from_lowering_authority(authority: &WorthUiExecutionPlanLoweringFacts) -> Self {
        let lowering_basis = authority.plan_input().basis();
        let node_inputs = authority.node_inputs();
        Self::from_lowered_parts(
            lowering_basis,
            node_inputs,
            authority.allocation_identity_digest(),
        )
    }

    fn from_lowered_parts(
        lowering_basis: &crate::runtime::WorthUiPlanLoweringBasis,
        node_inputs: &[crate::runtime::WorthUiPlanNodeInput],
        allocation_planning_identity_digest: u64,
    ) -> Self {
        Self {
            prior_artifact_digest: lowering_basis.prior_artifact_digest(),
            candidate_artifact_digest: lowering_basis.candidate_artifact_digest(),
            frame_epoch: lowering_basis.frame_epoch(),
            plan_node_input_count: node_inputs.len(),
            query_binding_input_count: lowering_basis.query_binding_input_count(),
            allocation_planning_identity_digest,
        }
    }

    pub(crate) fn digest(&self) -> u64 {
        0x8a11_d1e5_0000_000f
            ^ self
                .prior_artifact_digest
                .map_or(0x1a11_0000_0000_0000, |digest| digest.rotate_left(7))
            ^ self.candidate_artifact_digest.rotate_left(19)
            ^ (self.frame_epoch.as_u64()).rotate_left(31)
            ^ (self.plan_node_input_count as u64).rotate_left(43)
            ^ (self.query_binding_input_count as u64).rotate_left(53)
            ^ self.allocation_planning_identity_digest.rotate_left(3)
    }

    pub fn prior_artifact_digest(&self) -> Option<u64> {
        self.prior_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub fn plan_node_input_count(&self) -> usize {
        self.plan_node_input_count
    }

    pub fn query_binding_input_count(&self) -> usize {
        self.query_binding_input_count
    }

    pub fn allocation_planning_identity_digest(&self) -> u64 {
        self.allocation_planning_identity_digest
    }
}
