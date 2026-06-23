use crate::{
    ValidationHeaderAppliedStyleReceipt, ValidationManualFlowMatrixRenderPlan,
    ValidationManualFlowMatrixSnapshot,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationManualFlowMatrixProjection {
    style: ValidationHeaderAppliedStyleReceipt,
    snapshot: ValidationManualFlowMatrixSnapshot,
}

impl ValidationManualFlowMatrixProjection {
    pub fn new(
        style: ValidationHeaderAppliedStyleReceipt,
        snapshot: ValidationManualFlowMatrixSnapshot,
    ) -> Self {
        Self { style, snapshot }
    }

    pub fn into_render_plan(self) -> ValidationManualFlowMatrixRenderPlan {
        ValidationManualFlowMatrixRenderPlan::new(self.style, self.snapshot)
    }
}
