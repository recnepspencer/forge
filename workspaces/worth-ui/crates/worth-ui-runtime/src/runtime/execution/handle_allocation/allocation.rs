use crate::runtime::{
    WorthUiChildRangeHandle, WorthUiRuntimeHandle, WorthUiRuntimeHandleAllocationBasis,
    WorthUiRuntimeHandleAllocationCounters, WorthUiRuntimeHandleAllocationReceipt,
    WorthUiRuntimeHandleFamilyWidths,
};
#[cfg(test)]
use crate::runtime::{WorthUiCommandHandle, WorthUiComponentHandle, WorthUiViewBindingHandle};

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

    #[cfg(test)]
    pub fn family_widths(&self) -> WorthUiRuntimeHandleFamilyWidths {
        self.family_widths
    }

    pub fn counters(&self) -> WorthUiRuntimeHandleAllocationCounters {
        self.counters
    }

    pub fn runtime_handles(&self) -> &[WorthUiRuntimeHandle] {
        &self.runtime_handles
    }

    #[cfg(test)]
    pub fn component_handles(&self) -> impl Iterator<Item = WorthUiComponentHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::ComponentInvocation,
            WorthUiComponentHandle::from_runtime_handle,
        )
    }

    #[cfg(test)]
    pub fn command_handles(&self) -> impl Iterator<Item = WorthUiCommandHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::Command,
            WorthUiCommandHandle::from_runtime_handle,
        )
    }

    pub fn child_range_handles(&self) -> impl Iterator<Item = WorthUiChildRangeHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::ChildRange,
            WorthUiChildRangeHandle::from_runtime_handle,
        )
    }

    #[cfg(test)]
    pub fn view_binding_handles(&self) -> impl Iterator<Item = WorthUiViewBindingHandle> + '_ {
        self.project_family(
            crate::runtime::WorthUiPlanNodeInputFamily::QueryViewBinding,
            WorthUiViewBindingHandle::from_runtime_handle,
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
