use super::super::slot_set;
use super::{WorthUiPlanRegionStorageCounters, WorthUiPlanRegionStore};
use crate::runtime::plan_topology::WorthUiPlanRegionExecutable;

impl WorthUiPlanRegionStore {
    pub(crate) fn first_realtime_budget_exhaustion(&self) -> Option<(u16, u16)> {
        let slot = slot_set::WorthUiPlanRegionSlotSetView::<1>::new(
            [self.realtime_budget_exhaustion_root.clone()],
            usize::from(self.realtime_budget_exhaustion_root.is_some()),
        )
        .first()?;
        let meaning = self
            .executable_for_stable_slot(slot)?
            .realtime_meaning_reference()?;
        let contract = meaning.contract();
        Some((
            contract.frame_budget_millis(),
            contract.declared_frame_cost_millis(),
        ))
    }

    pub(super) fn replace_realtime_budget_index(
        &mut self,
        stable_slot: u64,
        predecessor_exhausted: bool,
        successor_exhausted: bool,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) {
        if predecessor_exhausted == successor_exhausted {
            return;
        }
        self.realtime_budget_exhaustion_root = if successor_exhausted {
            slot_set::insert(&self.realtime_budget_exhaustion_root, stable_slot, counters)
        } else {
            slot_set::remove(&self.realtime_budget_exhaustion_root, stable_slot, counters)
        };
    }
}

pub(super) fn realtime_budget_exhausted(executable: &WorthUiPlanRegionExecutable) -> bool {
    executable
        .realtime_meaning_reference()
        .is_some_and(|meaning| {
            meaning.contract().declared_frame_cost_millis()
                > meaning.contract().frame_budget_millis()
        })
}
