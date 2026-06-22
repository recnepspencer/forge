use crate::runtime::{
    WorthUiOrdinaryLaneCertification, WorthUiOrdinaryLaneCounters, WorthUiOrdinaryLaneNode,
    WorthUiRuntimeHandleAllocationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLanePlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    support_digest: u64,
    ordinary_plan_digest: u64,
    rows: Vec<WorthUiOrdinaryLaneNode>,
    component_plan_indexes: Vec<u32>,
    command_plan_indexes: Vec<u32>,
    token_plan_indexes: Vec<u32>,
    counters: WorthUiOrdinaryLaneCounters,
}

impl WorthUiOrdinaryLanePlan {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
        support_digest: u64,
        ordinary_plan_digest: u64,
        rows: Vec<WorthUiOrdinaryLaneNode>,
        component_plan_indexes: Vec<u32>,
        command_plan_indexes: Vec<u32>,
        token_plan_indexes: Vec<u32>,
        counters: WorthUiOrdinaryLaneCounters,
    ) -> Self {
        Self {
            handle_receipt,
            support_digest,
            ordinary_plan_digest,
            rows,
            component_plan_indexes,
            command_plan_indexes,
            token_plan_indexes,
            counters,
        }
    }

    pub fn handle_receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }

    pub fn support_digest(&self) -> u64 {
        self.support_digest
    }

    pub fn ordinary_plan_digest(&self) -> u64 {
        self.ordinary_plan_digest
    }

    pub fn rows(&self) -> &[WorthUiOrdinaryLaneNode] {
        &self.rows
    }

    pub fn component_plan_indexes(&self) -> &[u32] {
        &self.component_plan_indexes
    }

    pub fn command_plan_indexes(&self) -> &[u32] {
        &self.command_plan_indexes
    }

    pub fn token_plan_indexes(&self) -> &[u32] {
        &self.token_plan_indexes
    }

    pub fn counters(&self) -> WorthUiOrdinaryLaneCounters {
        self.counters
    }

    pub(crate) fn row_for_plan_index(&self, plan_index: u32) -> Option<&WorthUiOrdinaryLaneNode> {
        self.rows
            .binary_search_by_key(&plan_index, WorthUiOrdinaryLaneNode::plan_index)
            .ok()
            .map(|index| &self.rows[index])
    }

    pub(crate) fn certification(
        &self,
        lane: crate::runtime::WorthUiOrdinaryExecutionLane,
    ) -> WorthUiOrdinaryLaneCertification {
        WorthUiOrdinaryLaneCertification::new(
            lane,
            self.ordinary_plan_digest,
            self.support_digest,
            self.handle_receipt,
        )
    }
}
