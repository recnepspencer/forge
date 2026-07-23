use super::{
    WorthUiPlanRegionStorageCounters, WorthUiPlanRegionStore, WorthUiPlanRegionTransitionEvidence,
};

#[derive(Clone, Debug)]
pub(crate) struct WorthUiPlanRegionSuccessor {
    pub(super) store: WorthUiPlanRegionStore,
    pub(super) evidence: Vec<WorthUiPlanRegionTransitionEvidence>,
    pub(super) counters: WorthUiPlanRegionStorageCounters,
}

impl WorthUiPlanRegionSuccessor {
    pub(crate) fn store(&self) -> &WorthUiPlanRegionStore {
        &self.store
    }

    pub(crate) fn into_store(self) -> WorthUiPlanRegionStore {
        self.store
    }

    pub fn evidence(&self) -> &[WorthUiPlanRegionTransitionEvidence] {
        &self.evidence
    }

    pub fn counters(&self) -> WorthUiPlanRegionStorageCounters {
        self.counters
    }
}
