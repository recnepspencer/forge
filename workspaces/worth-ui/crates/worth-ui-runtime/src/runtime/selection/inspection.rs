#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiSelectionDropInspectionReason {
    Interaction,
    CatalogReconciliation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiSelectionDropInspectionRecord {
    owner: super::UiSelectionOwnerIdentity,
    reason: UiSelectionDropInspectionReason,
    removed_count: u32,
    selected_count: usize,
    revision: u64,
}

impl UiSelectionDropInspectionRecord {
    pub(super) fn from_delta(
        owner: super::UiSelectionOwnerIdentity,
        reason: UiSelectionDropInspectionReason,
        delta: &super::UiSelectionDelta,
    ) -> Option<Self> {
        (!delta.removed().is_empty()).then(|| Self {
            owner,
            reason,
            removed_count: u32::try_from(delta.removed().len()).unwrap_or(u32::MAX),
            selected_count: delta.selected_count(),
            revision: delta.revision(),
        })
    }

    pub(crate) const fn owner(self) -> super::UiSelectionOwnerIdentity {
        self.owner
    }
    pub(crate) const fn reason(self) -> UiSelectionDropInspectionReason {
        self.reason
    }
    pub(crate) const fn removed_count(self) -> u32 {
        self.removed_count
    }
    pub(crate) const fn selected_count(self) -> usize {
        self.selected_count
    }
    pub(crate) const fn revision(self) -> u64 {
        self.revision
    }
}
