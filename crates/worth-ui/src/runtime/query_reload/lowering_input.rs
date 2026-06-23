use crate::runtime::{
    WorthUiQueryBindingComparison, WorthUiQueryEffectPostureReceipt, WorthUiQueryLiveRebindPlan,
    WorthUiQueryProjectionFactReceipt, WorthUiQueryStateSnapshotReceipt,
    WorthUiQuerySupportReceipt, WorthUiVirtualizedDataFrameTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryRuntimeFactLoweringInput {
    support_receipt: WorthUiQuerySupportReceipt,
    binding_comparison: WorthUiQueryBindingComparison,
    live_rebind_plan: WorthUiQueryLiveRebindPlan,
    projection_fact_receipts: Vec<WorthUiQueryProjectionFactReceipt>,
    state_snapshot_receipts: Vec<WorthUiQueryStateSnapshotReceipt>,
    effect_posture_receipts: Vec<WorthUiQueryEffectPostureReceipt>,
    virtualized_frame_targets: Vec<WorthUiVirtualizedDataFrameTarget>,
}

impl WorthUiQueryRuntimeFactLoweringInput {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn from_runtime_evidence(
        support_receipt: WorthUiQuerySupportReceipt,
        binding_comparison: WorthUiQueryBindingComparison,
        live_rebind_plan: WorthUiQueryLiveRebindPlan,
    ) -> Self {
        Self {
            support_receipt,
            binding_comparison,
            live_rebind_plan,
            projection_fact_receipts: Vec::new(),
            state_snapshot_receipts: Vec::new(),
            effect_posture_receipts: Vec::new(),
            virtualized_frame_targets: Vec::new(),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn with_projection_fact_receipts(
        mut self,
        receipts: impl IntoIterator<Item = WorthUiQueryProjectionFactReceipt>,
    ) -> Self {
        self.projection_fact_receipts.extend(receipts);
        self
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn with_state_snapshot_receipts(
        mut self,
        receipts: impl IntoIterator<Item = WorthUiQueryStateSnapshotReceipt>,
    ) -> Self {
        self.state_snapshot_receipts.extend(receipts);
        self
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn with_effect_posture_receipts(
        mut self,
        receipts: impl IntoIterator<Item = WorthUiQueryEffectPostureReceipt>,
    ) -> Self {
        self.effect_posture_receipts.extend(receipts);
        self
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn with_virtualized_frame_targets(
        mut self,
        targets: impl IntoIterator<Item = WorthUiVirtualizedDataFrameTarget>,
    ) -> Self {
        self.virtualized_frame_targets.extend(targets);
        self
    }

    pub(crate) fn support_receipt(&self) -> WorthUiQuerySupportReceipt {
        self.support_receipt
    }

    pub(crate) fn binding_comparison(&self) -> &WorthUiQueryBindingComparison {
        &self.binding_comparison
    }

    pub(crate) fn live_rebind_plan(&self) -> &WorthUiQueryLiveRebindPlan {
        &self.live_rebind_plan
    }

    pub(crate) fn projection_fact_receipts(&self) -> &[WorthUiQueryProjectionFactReceipt] {
        &self.projection_fact_receipts
    }

    pub(crate) fn state_snapshot_receipts(&self) -> &[WorthUiQueryStateSnapshotReceipt] {
        &self.state_snapshot_receipts
    }

    pub(crate) fn effect_posture_receipts(&self) -> &[WorthUiQueryEffectPostureReceipt] {
        &self.effect_posture_receipts
    }

    pub(crate) fn virtualized_frame_targets(&self) -> &[WorthUiVirtualizedDataFrameTarget] {
        &self.virtualized_frame_targets
    }
}
