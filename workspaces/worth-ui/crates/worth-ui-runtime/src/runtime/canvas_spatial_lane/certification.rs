use crate::runtime::WorthUiRuntimeHandleAllocationReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialCertification {
    canvas_plan_digest: u64,
    support_digest: u64,
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    host_session_identity: u64,
    host_observation_generation: u64,
}

impl WorthUiCanvasSpatialCertification {
    pub(crate) fn new(
        canvas_plan_digest: u64,
        support_digest: u64,
        handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
        host_binding: crate::facade::WorthUiHostPlanBinding,
    ) -> Self {
        Self {
            canvas_plan_digest,
            support_digest,
            handle_receipt,
            host_session_identity: host_binding.session_identity().as_u64(),
            host_observation_generation: host_binding.observation_generation().as_u64(),
        }
    }

    pub fn canvas_plan_digest(self) -> u64 {
        self.canvas_plan_digest
    }

    pub fn support_digest(self) -> u64 {
        self.support_digest
    }

    pub fn handle_receipt(self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }
    pub fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }
    pub fn host_observation_generation(self) -> u64 {
        self.host_observation_generation
    }
}
