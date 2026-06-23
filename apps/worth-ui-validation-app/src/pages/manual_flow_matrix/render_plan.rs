use crate::{
    ValidationHeaderAppliedStyleReceipt, ValidationManualFlowMatrixSnapshot,
    ValidationManualFlowVisibleRow,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationManualFlowMatrixRenderPlan {
    style: ValidationHeaderAppliedStyleReceipt,
    snapshot: ValidationManualFlowMatrixSnapshot,
}

impl ValidationManualFlowMatrixRenderPlan {
    pub fn new(
        style: ValidationHeaderAppliedStyleReceipt,
        snapshot: ValidationManualFlowMatrixSnapshot,
    ) -> Self {
        Self { style, snapshot }
    }

    pub fn style(&self) -> &ValidationHeaderAppliedStyleReceipt {
        &self.style
    }

    pub fn rows(&self) -> &[ValidationManualFlowVisibleRow] {
        self.snapshot.rows()
    }
}
