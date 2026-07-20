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
}

pub(crate) struct WorthUiRuntimeHandleAllocationInput {
    pub basis: WorthUiRuntimeHandleAllocationBasis,
    pub receipt: WorthUiRuntimeHandleAllocationReceipt,
    pub family_widths: WorthUiRuntimeHandleFamilyWidths,
    pub counters: WorthUiRuntimeHandleAllocationCounters,
    pub runtime_handles: Vec<WorthUiRuntimeHandle>,
}

impl WorthUiRuntimeHandleAllocation {
    pub(crate) fn new(input: WorthUiRuntimeHandleAllocationInput) -> Self {
        let WorthUiRuntimeHandleAllocationInput {
            basis,
            receipt,
            family_widths,
            counters,
            runtime_handles,
        } = input;
        Self {
            basis,
            receipt,
            family_widths,
            counters,
            runtime_handles,
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

    pub fn component_handles(&self) -> impl Iterator<Item = WorthUiComponentHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::ComponentInvocation,
            WorthUiComponentHandle::from_runtime_handle,
        )
    }

    pub fn command_handles(&self) -> impl Iterator<Item = WorthUiCommandHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::Command,
            WorthUiCommandHandle::from_runtime_handle,
        )
    }

    pub fn token_handles(&self) -> impl Iterator<Item = WorthUiTokenHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::TokenStyle,
            WorthUiTokenHandle::from_runtime_handle,
        )
    }

    pub fn child_range_handles(&self) -> impl Iterator<Item = WorthUiChildRangeHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::ChildRange,
            WorthUiChildRangeHandle::from_runtime_handle,
        )
    }

    pub fn view_binding_handles(&self) -> impl Iterator<Item = WorthUiViewBindingHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::QueryViewBinding,
            WorthUiViewBindingHandle::from_runtime_handle,
        )
    }

    pub fn lane_handles(&self) -> impl Iterator<Item = WorthUiLaneHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::LanePartitionRef,
            WorthUiLaneHandle::from_runtime_handle,
        )
    }

    pub fn state_slot_handles(&self) -> impl Iterator<Item = WorthUiStateSlotHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::StateSlot,
            WorthUiStateSlotHandle::from_runtime_handle,
        )
    }

    fn project_family<'a, T: 'a>(
        &'a self,
        family: crate::runtime::WorthUiPlanNodeInputFamily,
        project: fn(WorthUiRuntimeHandle) -> T,
    ) -> impl Iterator<Item = T> + 'a {
        self.runtime_handles
            .iter()
            .copied()
            .filter(move |handle| handle.family() == family)
            .map(project)
    }
}
