use std::rc::Rc;

#[cfg(test)]
use std::rc::Weak;

use crate::runtime::WorthUiPlanNodeInputFamily;

use super::slot_set::{self, WorthUiPlanRegionSlotSetNode, WorthUiPlanRegionSlotSetView};
use super::WorthUiPlanRegionStorageCounters;

const FAMILY_COUNT: usize = 13;

#[derive(Clone, Debug)]
pub(super) struct WorthUiPlanRegionFamilyIndex {
    roots: [Option<Rc<WorthUiPlanRegionSlotSetNode>>; FAMILY_COUNT],
    counts: [usize; FAMILY_COUNT],
    semantic_digests: [u64; FAMILY_COUNT],
}

impl Default for WorthUiPlanRegionFamilyIndex {
    fn default() -> Self {
        Self {
            roots: std::array::from_fn(|_| None),
            counts: [0; FAMILY_COUNT],
            semantic_digests: [0; FAMILY_COUNT],
        }
    }
}

impl WorthUiPlanRegionFamilyIndex {
    pub(super) fn insert(
        &mut self,
        family: WorthUiPlanNodeInputFamily,
        stable_slot: u64,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) {
        let index = family_index(family);
        if slot_set::contains(&self.roots[index], stable_slot) {
            return;
        }
        self.roots[index] = slot_set::insert(&self.roots[index], stable_slot, counters);
        self.counts[index] += 1;
    }

    pub(super) fn remove(
        &mut self,
        family: WorthUiPlanNodeInputFamily,
        stable_slot: u64,
        counters: &mut WorthUiPlanRegionStorageCounters,
    ) {
        let index = family_index(family);
        if !slot_set::contains(&self.roots[index], stable_slot) {
            return;
        }
        self.roots[index] = slot_set::remove(&self.roots[index], stable_slot, counters);
        self.counts[index] -= 1;
    }

    pub(super) fn count(&self, family: WorthUiPlanNodeInputFamily) -> usize {
        self.counts[family_index(family)]
    }

    pub(super) fn toggle_semantic_digest(
        &mut self,
        family: WorthUiPlanNodeInputFamily,
        digest: u64,
    ) {
        self.semantic_digests[family_index(family)] ^= digest;
    }

    pub(super) fn semantic_digest(&self, family: WorthUiPlanNodeInputFamily) -> u64 {
        self.semantic_digests[family_index(family)]
    }

    pub(super) fn view<const N: usize>(
        &self,
        families: [WorthUiPlanNodeInputFamily; N],
    ) -> WorthUiPlanRegionSlotSetView<N> {
        let roots = families.map(|family| self.roots[family_index(family)].clone());
        let len = families.into_iter().map(|family| self.count(family)).sum();
        WorthUiPlanRegionSlotSetView::new(roots, len)
    }

    #[cfg(test)]
    pub(super) fn reachable_node_count(&self) -> usize {
        self.roots.iter().map(slot_set::reachable_node_count).sum()
    }

    #[cfg(test)]
    pub(super) fn exclusive_root_probes(&self) -> Vec<Weak<WorthUiPlanRegionSlotSetNode>> {
        self.roots
            .iter()
            .filter_map(|root| root.as_ref())
            .filter(|root| Rc::strong_count(root) == 1)
            .map(Rc::downgrade)
            .collect()
    }
}

fn family_index(family: WorthUiPlanNodeInputFamily) -> usize {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation => 0,
        WorthUiPlanNodeInputFamily::LayoutRegion => 1,
        WorthUiPlanNodeInputFamily::Command => 2,
        WorthUiPlanNodeInputFamily::TokenStyle => 3,
        WorthUiPlanNodeInputFamily::ChildRange => 4,
        WorthUiPlanNodeInputFamily::QueryViewBinding => 5,
        WorthUiPlanNodeInputFamily::Accessibility => 6,
        WorthUiPlanNodeInputFamily::DiagnosticsRef => 7,
        WorthUiPlanNodeInputFamily::LanePartitionRef => 8,
        WorthUiPlanNodeInputFamily::RenderResourceRef => 9,
        WorthUiPlanNodeInputFamily::StateSlot => 10,
        WorthUiPlanNodeInputFamily::CanvasSpatial => 11,
        WorthUiPlanNodeInputFamily::RealtimeOverlay => 12,
    }
}
