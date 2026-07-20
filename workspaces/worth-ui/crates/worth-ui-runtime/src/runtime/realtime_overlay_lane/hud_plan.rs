use crate::runtime::{
    WorthUiHudNode, WorthUiRealtimeCertification, WorthUiRealtimeLaneCounters,
    WorthUiRuntimeHandleAllocationReceipt,
};

#[derive(Clone, Debug)]
pub struct WorthUiHudPlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    support_digest: u64,
    hud_plan_digest: u64,
    host_binding: crate::facade::WorthUiHostPlanBinding,
    region_store: crate::runtime::plan_topology::WorthUiPlanRegionStore,
    realtime_slots: crate::runtime::plan_topology::WorthUiPlanRegionSlotSetView<1>,
    counters: WorthUiRealtimeLaneCounters,
}

pub(crate) struct WorthUiHudPlanInput {
    pub handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    pub support_digest: u64,
    pub hud_plan_digest: u64,
    pub host_binding: crate::facade::WorthUiHostPlanBinding,
    pub region_store: crate::runtime::plan_topology::WorthUiPlanRegionStore,
    pub realtime_slots: crate::runtime::plan_topology::WorthUiPlanRegionSlotSetView<1>,
    pub counters: WorthUiRealtimeLaneCounters,
}

impl PartialEq for WorthUiHudPlan {
    fn eq(&self, other: &Self) -> bool {
        self.handle_receipt == other.handle_receipt
            && self.support_digest == other.support_digest
            && self.hud_plan_digest == other.hud_plan_digest
            && self.host_binding == other.host_binding
            && self.realtime_slots == other.realtime_slots
    }
}

impl Eq for WorthUiHudPlan {}

impl WorthUiHudPlan {
    pub(crate) fn new(input: WorthUiHudPlanInput) -> Self {
        Self {
            handle_receipt: input.handle_receipt,
            support_digest: input.support_digest,
            hud_plan_digest: input.hud_plan_digest,
            host_binding: input.host_binding,
            region_store: input.region_store,
            realtime_slots: input.realtime_slots,
            counters: input.counters,
        }
    }

    pub fn handle_receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }
    pub fn support_digest(&self) -> u64 {
        self.support_digest
    }
    pub fn hud_plan_digest(&self) -> u64 {
        self.hud_plan_digest
    }
    pub fn row_count(&self) -> usize {
        self.realtime_slots.len()
    }
    pub fn counters(&self) -> WorthUiRealtimeLaneCounters {
        self.counters
    }

    pub(crate) fn first_row(&self) -> Option<WorthUiHudNode> {
        self.realtime_slots
            .first()
            .and_then(|slot| u32::try_from(slot).ok())
            .and_then(|index| self.row_for_plan_index(index))
    }

    pub(crate) fn row_for_plan_index(&self, plan_index: u32) -> Option<WorthUiHudNode> {
        let slot = u64::from(plan_index);
        if !self.realtime_slots.contains(slot) {
            return None;
        }
        let executable = self.region_store.executable_for_stable_slot(slot)?;
        let meaning = executable.realtime_meaning_reference()?;
        let handle = self
            .region_store
            .runtime_handle_for_stable_slot(slot, self.handle_receipt.arena_identity())?;
        Some(WorthUiHudNode::new(
            handle,
            meaning.contract(),
            self.host_binding,
            self.handle_receipt.basis_digest(),
        ))
    }

    pub(crate) fn certification(&self, row: WorthUiHudNode) -> WorthUiRealtimeCertification {
        WorthUiRealtimeCertification::new(
            self.hud_plan_digest,
            self.support_digest,
            row.renderer_surface_admission().policy_digest(),
            self.handle_receipt,
            self.host_binding,
        )
    }
}
