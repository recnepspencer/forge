use crate::runtime::{
    WorthUiChildRangeHandle, WorthUiCommandHandle, WorthUiComponentHandle, WorthUiLaneHandle,
    WorthUiRuntimeHandle, WorthUiRuntimeHandleAllocationBasis,
    WorthUiRuntimeHandleAllocationCounters, WorthUiRuntimeHandleAllocationReceipt,
    WorthUiRuntimeHandleFamilyWidths, WorthUiStateSlotHandle, WorthUiTokenHandle,
    WorthUiViewBindingHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeHandleAllocation {
    basis: WorthUiRuntimeHandleAllocationBasis,
    receipt: WorthUiRuntimeHandleAllocationReceipt,
    family_widths: WorthUiRuntimeHandleFamilyWidths,
    counters: WorthUiRuntimeHandleAllocationCounters,
    runtime_handles: Vec<WorthUiRuntimeHandle>,
    component_handles: Vec<WorthUiComponentHandle>,
    command_handles: Vec<WorthUiCommandHandle>,
    token_handles: Vec<WorthUiTokenHandle>,
    child_range_handles: Vec<WorthUiChildRangeHandle>,
    view_binding_handles: Vec<WorthUiViewBindingHandle>,
    lane_handles: Vec<WorthUiLaneHandle>,
    state_slot_handles: Vec<WorthUiStateSlotHandle>,
}

impl WorthUiRuntimeHandleAllocation {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        basis: WorthUiRuntimeHandleAllocationBasis,
        receipt: WorthUiRuntimeHandleAllocationReceipt,
        family_widths: WorthUiRuntimeHandleFamilyWidths,
        counters: WorthUiRuntimeHandleAllocationCounters,
        runtime_handles: Vec<WorthUiRuntimeHandle>,
        component_handles: Vec<WorthUiComponentHandle>,
        command_handles: Vec<WorthUiCommandHandle>,
        token_handles: Vec<WorthUiTokenHandle>,
        child_range_handles: Vec<WorthUiChildRangeHandle>,
        view_binding_handles: Vec<WorthUiViewBindingHandle>,
        lane_handles: Vec<WorthUiLaneHandle>,
        state_slot_handles: Vec<WorthUiStateSlotHandle>,
    ) -> Self {
        Self {
            basis,
            receipt,
            family_widths,
            counters,
            runtime_handles,
            component_handles,
            command_handles,
            token_handles,
            child_range_handles,
            view_binding_handles,
            lane_handles,
            state_slot_handles,
        }
    }

    pub fn basis(&self) -> &WorthUiRuntimeHandleAllocationBasis {
        &self.basis
    }

    pub fn receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.receipt
    }

    pub fn family_widths(&self) -> WorthUiRuntimeHandleFamilyWidths {
        self.family_widths
    }

    pub fn counters(&self) -> WorthUiRuntimeHandleAllocationCounters {
        self.counters
    }

    pub fn runtime_handles(&self) -> &[WorthUiRuntimeHandle] {
        &self.runtime_handles
    }

    pub fn component_handles(&self) -> &[WorthUiComponentHandle] {
        &self.component_handles
    }

    pub fn command_handles(&self) -> &[WorthUiCommandHandle] {
        &self.command_handles
    }

    pub fn token_handles(&self) -> &[WorthUiTokenHandle] {
        &self.token_handles
    }

    pub fn child_range_handles(&self) -> &[WorthUiChildRangeHandle] {
        &self.child_range_handles
    }

    pub fn view_binding_handles(&self) -> &[WorthUiViewBindingHandle] {
        &self.view_binding_handles
    }

    pub fn lane_handles(&self) -> &[WorthUiLaneHandle] {
        &self.lane_handles
    }

    pub fn state_slot_handles(&self) -> &[WorthUiStateSlotHandle] {
        &self.state_slot_handles
    }
}
