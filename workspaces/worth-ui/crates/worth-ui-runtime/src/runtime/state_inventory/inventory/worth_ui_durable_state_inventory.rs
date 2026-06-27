use crate::runtime::{
    WorthUiDurableStateFamily, WorthUiDurableStateFamilyId, WorthUiDurableStateInventoryCounters,
    WorthUiTransientInteractionPolicy, WorthUiTransientInteractionState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateInventory {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    families: Vec<WorthUiDurableStateFamily>,
    transient_policies: Vec<(
        WorthUiTransientInteractionState,
        WorthUiTransientInteractionPolicy,
    )>,
    counters: WorthUiDurableStateInventoryCounters,
}

impl WorthUiDurableStateInventory {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        mut families: Vec<WorthUiDurableStateFamily>,
        transient_policies: Vec<(
            WorthUiTransientInteractionState,
            WorthUiTransientInteractionPolicy,
        )>,
        counters: WorthUiDurableStateInventoryCounters,
    ) -> Self {
        families.sort_by(|left, right| left.id().cmp(right.id()));
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            families,
            transient_policies,
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

    pub fn family(
        &self,
        family_id: &WorthUiDurableStateFamilyId,
    ) -> Option<&WorthUiDurableStateFamily> {
        self.families.iter().find(|family| family.id() == family_id)
    }

    pub fn transient(
        &self,
        state: WorthUiTransientInteractionState,
    ) -> WorthUiTransientInteractionPolicy {
        self.transient_policies
            .iter()
            .find(|(stored_state, _)| *stored_state == state)
            .map(|(_, policy)| *policy)
            .unwrap_or_else(|| state.default_policy())
    }

    pub fn counters(&self) -> WorthUiDurableStateInventoryCounters {
        self.counters
    }
}
