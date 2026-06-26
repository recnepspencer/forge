use super::fingerprint::plan_input_fingerprint;
use crate::runtime::{WorthUiExecutionPlanInput, WorthUiRuntimeFrameEpoch};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeHandleAllocationBasis {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    plan_node_input_count: usize,
    query_binding_input_count: usize,
    plan_input_fingerprint: u64,
}

impl WorthUiRuntimeHandleAllocationBasis {
    pub(crate) fn from_plan_input(plan_input: &WorthUiExecutionPlanInput) -> Self {
        Self {
            active_artifact_digest: plan_input.basis().active_artifact_digest(),
            candidate_artifact_digest: plan_input.basis().candidate_artifact_digest(),
            frame_epoch: plan_input.basis().frame_epoch(),
            plan_node_input_count: plan_input.node_inputs().len(),
            query_binding_input_count: plan_input.basis().staged_query_rebind_entry_count(),
            plan_input_fingerprint: plan_input_fingerprint(plan_input.node_inputs()),
        }
    }

    pub(crate) fn digest(&self) -> u64 {
        0x8a11_d1e5_0000_000f
            ^ self.active_artifact_digest.rotate_left(7)
            ^ self.candidate_artifact_digest.rotate_left(19)
            ^ (self.frame_epoch.as_u64()).rotate_left(31)
            ^ (self.plan_node_input_count as u64).rotate_left(43)
            ^ (self.query_binding_input_count as u64).rotate_left(53)
            ^ self.plan_input_fingerprint.rotate_left(3)
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
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

    pub fn plan_input_fingerprint(&self) -> u64 {
        self.plan_input_fingerprint
    }
}
