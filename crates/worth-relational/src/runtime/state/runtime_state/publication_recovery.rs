use crate::history::data::CommitId;
use crate::publication::data::DeferredPublicationSettlement;

use super::RelationalRuntimePublicationBinding;

impl RelationalRuntimePublicationBinding {
    pub(crate) fn register_deferred_settlement(
        &self,
        settlement: DeferredPublicationSettlement,
        maximum_settlements: usize,
    ) -> Result<(), ()> {
        let commit_id = settlement.commit().commit_id;
        let mut settlements = self
            .lifecycle
            .deferred_settlements
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if !settlements.contains_key(&commit_id) && settlements.len() >= maximum_settlements {
            return Err(());
        }
        settlements.insert(commit_id, settlement);
        Ok(())
    }

    pub(crate) fn deferred_settlement(
        &self,
        commit_id: CommitId,
    ) -> Option<DeferredPublicationSettlement> {
        self.lifecycle
            .deferred_settlements
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(&commit_id)
            .cloned()
    }

    pub(crate) fn release_deferred_settlement(&self, commit_id: CommitId) {
        self.lifecycle
            .deferred_settlements
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(&commit_id);
    }

    pub(super) fn clear_deferred_settlements(&self) {
        self.lifecycle
            .deferred_settlements
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clear();
    }

    #[cfg(test)]
    pub(crate) fn deferred_settlement_count(&self) -> usize {
        self.lifecycle
            .deferred_settlements
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .len()
    }
}
