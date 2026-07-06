use super::claim_validation::reject_invalid_specialized_handle_claims;
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiChildRangeHandle, WorthUiCommandHandle,
    WorthUiComponentHandle, WorthUiHandlePlanGeneration, WorthUiLaneHandle, WorthUiPlanNodeInput,
    WorthUiPlanNodeInputFamily, WorthUiRuntimeHandle, WorthUiRuntimeHandleAllocation,
    WorthUiRuntimeHandleAllocationBasis, WorthUiRuntimeHandleAllocationCounters,
    WorthUiRuntimeHandleAllocationDenial, WorthUiRuntimeHandleAllocationDenialReason,
    WorthUiRuntimeHandleAllocationReceipt, WorthUiRuntimeHandleFamilyWidths,
    WorthUiStateSlotHandle, WorthUiTokenHandle, WorthUiViewBindingHandle,
};

pub(crate) struct WorthUiRuntimeHandleAllocator;

struct HandleAllocationAccumulator {
    plan_generation: WorthUiHandlePlanGeneration,
    counters: WorthUiRuntimeHandleAllocationCounters,
    family_widths: WorthUiRuntimeHandleFamilyWidths,
    runtime_handles: Vec<WorthUiRuntimeHandle>,
    component_handles: Vec<WorthUiComponentHandle>,
    command_handles: Vec<WorthUiCommandHandle>,
    token_handles: Vec<WorthUiTokenHandle>,
    child_range_handles: Vec<WorthUiChildRangeHandle>,
    view_binding_handles: Vec<WorthUiViewBindingHandle>,
    lane_handles: Vec<WorthUiLaneHandle>,
    state_slot_handles: Vec<WorthUiStateSlotHandle>,
}

impl WorthUiRuntimeHandleAllocator {
    pub(crate) fn allocate(
        allocation_planning: &WorthUiAllocationPlanning,
    ) -> Result<WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationDenial> {
        if !allocation_planning.is_admitted() {
            return Err(denial(
                WorthUiRuntimeHandleAllocationDenialReason::StalePlanInputReceipt,
                WorthUiRuntimeHandleAllocationCounters::default(),
            ));
        }
        let node_inputs = allocation_planning
            .node_inputs()
            .expect("admitted allocation planning must expose lowered node inputs");
        let basis =
            WorthUiRuntimeHandleAllocationBasis::from_allocation_planning(allocation_planning);
        let receipt = WorthUiRuntimeHandleAllocationReceipt::from_basis(&basis);
        let counters = reject_invalid_specialized_handle_claims(node_inputs)?;
        let mut allocation = HandleAllocationAccumulator::new(receipt.plan_generation(), counters);

        for (plan_index, node_input) in node_inputs.iter().enumerate() {
            allocation.record_plan_node_input(plan_index, node_input)?;
        }

        Ok(WorthUiRuntimeHandleAllocation::new(
            basis,
            receipt,
            allocation.family_widths,
            allocation.counters,
            allocation.runtime_handles,
            allocation.component_handles,
            allocation.command_handles,
            allocation.token_handles,
            allocation.child_range_handles,
            allocation.view_binding_handles,
            allocation.lane_handles,
            allocation.state_slot_handles,
        ))
    }
}

impl HandleAllocationAccumulator {
    fn new(
        plan_generation: WorthUiHandlePlanGeneration,
        counters: WorthUiRuntimeHandleAllocationCounters,
    ) -> Self {
        Self {
            plan_generation,
            counters,
            family_widths: WorthUiRuntimeHandleFamilyWidths::default(),
            runtime_handles: Vec::new(),
            component_handles: Vec::new(),
            command_handles: Vec::new(),
            token_handles: Vec::new(),
            child_range_handles: Vec::new(),
            view_binding_handles: Vec::new(),
            lane_handles: Vec::new(),
            state_slot_handles: Vec::new(),
        }
    }

    fn record_plan_node_input(
        &mut self,
        plan_index: usize,
        node_input: &WorthUiPlanNodeInput,
    ) -> Result<(), WorthUiRuntimeHandleAllocationDenial> {
        let plan_index = compact_plan_index(plan_index, self.counters)?;
        self.counters.record_plan_node_input();
        self.record_runtime_handle(plan_index, node_input.family());
        self.record_typed_handle(plan_index, node_input);
        Ok(())
    }

    fn record_runtime_handle(&mut self, plan_index: u32, family: WorthUiPlanNodeInputFamily) {
        self.runtime_handles.push(WorthUiRuntimeHandle::new(
            family,
            plan_index,
            self.plan_generation,
        ));
        self.family_widths.record_runtime_handle();
    }

    fn record_typed_handle(&mut self, plan_index: u32, node_input: &WorthUiPlanNodeInput) {
        match node_input.family() {
            WorthUiPlanNodeInputFamily::ComponentInvocation => {
                self.record_component_handle(plan_index)
            }
            WorthUiPlanNodeInputFamily::Command => self.record_command_handle(plan_index),
            WorthUiPlanNodeInputFamily::TokenStyle => self.record_token_handle(plan_index),
            WorthUiPlanNodeInputFamily::ChildRange => self.record_child_range_handle(plan_index),
            WorthUiPlanNodeInputFamily::QueryViewBinding => {
                self.record_view_binding_handle(plan_index, node_input)
            }
            WorthUiPlanNodeInputFamily::LanePartitionRef => self.record_lane_handle(plan_index),
            _ => {
                if node_input.transition().is_some() {
                    self.record_state_slot_handle(plan_index);
                }
            }
        }
    }

    fn record_component_handle(&mut self, plan_index: u32) {
        self.component_handles.push(WorthUiComponentHandle::new(
            plan_index,
            self.plan_generation,
        ));
        self.family_widths.record_component_handle();
        self.counters.record_component_handle();
    }

    fn record_command_handle(&mut self, plan_index: u32) {
        self.command_handles
            .push(WorthUiCommandHandle::new(plan_index, self.plan_generation));
        self.family_widths.record_command_handle();
        self.counters.record_command_handle();
    }

    fn record_token_handle(&mut self, plan_index: u32) {
        self.token_handles
            .push(WorthUiTokenHandle::new(plan_index, self.plan_generation));
        self.family_widths.record_token_handle();
        self.counters.record_token_handle();
    }

    fn record_child_range_handle(&mut self, plan_index: u32) {
        self.child_range_handles.push(WorthUiChildRangeHandle::new(
            plan_index,
            self.plan_generation,
        ));
        self.family_widths.record_child_range_handle();
        self.counters.record_child_range_handle();
    }

    fn record_view_binding_handle(&mut self, plan_index: u32, node_input: &WorthUiPlanNodeInput) {
        if node_input.query_binding_identity().is_none() {
            return;
        }
        debug_assert!(node_input.query_binding_posture().is_some());

        self.view_binding_handles
            .push(WorthUiViewBindingHandle::new(
                plan_index,
                self.plan_generation,
            ));
        self.family_widths.record_view_binding_handle();
        self.counters.record_view_binding_handle();
    }

    fn record_lane_handle(&mut self, plan_index: u32) {
        self.lane_handles
            .push(WorthUiLaneHandle::new(plan_index, self.plan_generation));
        self.family_widths.record_lane_handle();
        self.counters.record_lane_handle();
    }

    fn record_state_slot_handle(&mut self, plan_index: u32) {
        self.state_slot_handles.push(WorthUiStateSlotHandle::new(
            plan_index,
            self.plan_generation,
        ));
        self.family_widths.record_state_slot_handle();
        self.counters.record_state_slot_handle();
    }
}

fn compact_plan_index(
    plan_index: usize,
    counters: WorthUiRuntimeHandleAllocationCounters,
) -> Result<u32, WorthUiRuntimeHandleAllocationDenial> {
    u32::try_from(plan_index).map_err(|_| {
        denial(
            WorthUiRuntimeHandleAllocationDenialReason::UnsupportedHandleFamily,
            counters,
        )
    })
}

fn denial(
    reason: WorthUiRuntimeHandleAllocationDenialReason,
    counters: WorthUiRuntimeHandleAllocationCounters,
) -> WorthUiRuntimeHandleAllocationDenial {
    WorthUiRuntimeHandleAllocationDenial::new(reason, counters)
}
