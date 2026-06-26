use crate::runtime::{
    WorthUiRuntimeHandleAllocationReceipt, WorthUiVirtualizedDataCertification,
    WorthUiVirtualizedDataCounters, WorthUiVirtualizedDataNode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataPlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    support_digest: u64,
    data_plan_digest: u64,
    rows: Vec<WorthUiVirtualizedDataNode>,
    view_binding_plan_indexes: Vec<u32>,
    counters: WorthUiVirtualizedDataCounters,
}

impl WorthUiVirtualizedDataPlan {
    pub(crate) fn new(
        handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
        support_digest: u64,
        data_plan_digest: u64,
        rows: Vec<WorthUiVirtualizedDataNode>,
        view_binding_plan_indexes: Vec<u32>,
        counters: WorthUiVirtualizedDataCounters,
    ) -> Self {
        Self {
            handle_receipt,
            support_digest,
            data_plan_digest,
            rows,
            view_binding_plan_indexes,
            counters,
        }
    }

    pub fn handle_receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }

    pub fn support_digest(&self) -> u64 {
        self.support_digest
    }

    pub fn data_plan_digest(&self) -> u64 {
        self.data_plan_digest
    }

    pub fn rows(&self) -> &[WorthUiVirtualizedDataNode] {
        &self.rows
    }

    pub fn view_binding_plan_indexes(&self) -> &[u32] {
        &self.view_binding_plan_indexes
    }

    pub fn counters(&self) -> WorthUiVirtualizedDataCounters {
        self.counters
    }

    pub(crate) fn row_for_plan_index(
        &self,
        plan_index: u32,
    ) -> Option<&WorthUiVirtualizedDataNode> {
        self.rows
            .binary_search_by_key(&plan_index, WorthUiVirtualizedDataNode::plan_index)
            .ok()
            .map(|index| &self.rows[index])
    }

    pub(crate) fn certification(&self) -> WorthUiVirtualizedDataCertification {
        WorthUiVirtualizedDataCertification::new(
            self.data_plan_digest,
            self.support_digest,
            self.handle_receipt,
        )
    }
}
