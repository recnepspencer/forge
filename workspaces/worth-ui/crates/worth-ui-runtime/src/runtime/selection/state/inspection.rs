impl super::UiSelectionRuntimeState {
    pub(super) fn record_drop(
        &mut self,
        owner: super::super::UiSelectionOwnerIdentity,
        reason: super::super::UiSelectionDropInspectionReason,
        delta: &super::super::UiSelectionDelta,
    ) {
        if let Some(drop) =
            super::super::UiSelectionDropInspectionRecord::from_delta(owner, reason, delta)
        {
            self.last_drop = Some(drop);
        }
    }

    pub(crate) const fn last_drop(&self) -> Option<super::super::UiSelectionDropInspectionRecord> {
        self.last_drop
    }

    pub(crate) fn owner_count(&self) -> usize {
        self.owners.len()
    }
}
