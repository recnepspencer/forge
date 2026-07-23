#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHandleCapacityExhaustion {
    PlanIndex,
    StableSlot,
    SlotGeneration,
    ChildRange,
}

pub(crate) struct WorthUiHandleCapacity;

impl WorthUiHandleCapacity {
    pub(crate) fn plan_index(index: usize) -> Result<u32, WorthUiHandleCapacityExhaustion> {
        u32::try_from(index).map_err(|_| WorthUiHandleCapacityExhaustion::PlanIndex)
    }

    pub(crate) fn stable_slot(slot: u64) -> Result<u32, WorthUiHandleCapacityExhaustion> {
        u32::try_from(slot).map_err(|_| WorthUiHandleCapacityExhaustion::StableSlot)
    }

    pub(crate) fn next_stable_slot(slot: u64) -> Result<u64, WorthUiHandleCapacityExhaustion> {
        Self::stable_slot(slot)?;
        slot.checked_add(1)
            .ok_or(WorthUiHandleCapacityExhaustion::StableSlot)
    }

    pub(crate) fn next_slot_generation(
        generation: u64,
    ) -> Result<u64, WorthUiHandleCapacityExhaustion> {
        generation
            .checked_add(1)
            .ok_or(WorthUiHandleCapacityExhaustion::SlotGeneration)
    }

    pub(crate) fn child_range(count: usize) -> Result<u32, WorthUiHandleCapacityExhaustion> {
        u32::try_from(count).map_err(|_| WorthUiHandleCapacityExhaustion::ChildRange)
    }
}
