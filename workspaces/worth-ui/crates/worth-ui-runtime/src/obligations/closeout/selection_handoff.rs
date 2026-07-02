use crate::admission::UiSupportSnapshot;
use crate::obligations::inspection::{UiObligationEvidenceHandle, UiObligationEvidenceIndex};
use crate::obligations::selection::{UiSelectedObligation, UiSelectedObligationSet};
use crate::obligations::touch::UiGraphTouchDescriptor;

#[derive(Clone, Copy)]
pub struct UiObligationSelectionHandoff<'a> {
    selected: &'a UiSelectedObligationSet,
}

impl<'a> UiObligationSelectionHandoff<'a> {
    pub(crate) const fn new(selected: &'a UiSelectedObligationSet) -> Self {
        Self { selected }
    }

    pub fn touch(self) -> &'a UiGraphTouchDescriptor {
        self.selected.touch()
    }

    pub fn support_snapshot(self) -> &'a UiSupportSnapshot {
        self.selected.support_snapshot()
    }

    pub fn obligations(self) -> &'a [UiSelectedObligation] {
        self.selected.obligations()
    }

    pub fn selected_obligation_handles(self) -> Box<[UiObligationEvidenceHandle]> {
        self.selected.selected_obligation_handles()
    }

    pub fn evidence_index(self) -> &'a UiObligationEvidenceIndex {
        self.selected.evidence_index()
    }
}
