use crate::runtime::{
    WorthUiPrimitiveContentAnatomyReceipt, WorthUiPrimitiveContentReceipt,
    WorthUiQueryGraphExecutionReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveProvedContentAnatomy {
    anatomy: WorthUiPrimitiveContentAnatomyReceipt,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
}

impl WorthUiPrimitiveProvedContentAnatomy {
    pub(crate) fn from_content_receipt(receipt: &WorthUiPrimitiveContentReceipt) -> Self {
        let query_graph_execution = receipt.query_graph_execution_receipt();
        let anatomy = receipt.anatomy_receipt();
        Self {
            anatomy,
            query_graph_execution,
        }
    }

    pub fn anatomy(&self) -> &WorthUiPrimitiveContentAnatomyReceipt {
        &self.anatomy
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.query_graph_execution
    }
}
