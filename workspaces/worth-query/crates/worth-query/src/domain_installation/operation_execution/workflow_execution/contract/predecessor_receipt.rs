use super::{
    WorthQueryWorkflowStageReceipt, WorthQueryWorkflowStageWarning, WorthQueryWorkflowValue,
};

pub struct WorthQueryWorkflowPredecessorReceipt<'a> {
    receipt: &'a WorthQueryWorkflowStageReceipt,
}

impl<'a> WorthQueryWorkflowPredecessorReceipt<'a> {
    pub(super) fn new(receipt: &'a WorthQueryWorkflowStageReceipt) -> Self {
        Self { receipt }
    }

    pub fn identity(&self) -> &str {
        self.receipt.identity()
    }

    pub fn stage_identity(&self) -> &str {
        self.receipt.stage_identity()
    }

    pub fn output(&self) -> &WorthQueryWorkflowValue {
        self.receipt.output()
    }

    pub fn result_state(
        &self,
    ) -> Option<crate::domain_installation::WorthQueryOperationResultState> {
        self.receipt.result_state()
    }

    pub fn warnings(&self) -> &[WorthQueryWorkflowStageWarning] {
        self.receipt.warnings()
    }
}
