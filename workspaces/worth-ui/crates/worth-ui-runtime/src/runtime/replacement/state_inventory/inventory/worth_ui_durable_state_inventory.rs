use crate::runtime::{WorthUiDurableStateFamily, WorthUiDurableStateInventoryCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateInventory {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    families: Vec<WorthUiDurableStateFamily>,
    counters: WorthUiDurableStateInventoryCounters,
}

impl WorthUiDurableStateInventory {
    pub(super) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        mut families: Vec<WorthUiDurableStateFamily>,
        counters: WorthUiDurableStateInventoryCounters,
    ) -> Self {
        families.sort_by(|left, right| left.id().cmp(right.id()));
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            families,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn families(&self) -> &[WorthUiDurableStateFamily] {
        &self.families
    }

    pub(crate) fn counters(&self) -> WorthUiDurableStateInventoryCounters {
        self.counters
    }
}
