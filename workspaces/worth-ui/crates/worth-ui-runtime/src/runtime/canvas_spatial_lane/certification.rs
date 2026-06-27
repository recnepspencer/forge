use crate::runtime::WorthUiRuntimeHandleAllocationReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialCertification {
    canvas_plan_digest: u64,
    support_digest: u64,
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
}

impl WorthUiCanvasSpatialCertification {
    pub(crate) fn new(
        canvas_plan_digest: u64,
        support_digest: u64,
        handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    ) -> Self {
        Self {
            canvas_plan_digest,
            support_digest,
            handle_receipt,
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
}
