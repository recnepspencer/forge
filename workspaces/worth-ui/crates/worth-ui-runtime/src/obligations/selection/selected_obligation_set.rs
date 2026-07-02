use crate::admission::UiSupportSnapshot;
use crate::obligations::closeout::UiObligationSelectionHandoff;
use crate::obligations::inspection::{UiObligationEvidenceIndex, UiObligationEvidenceQuery};
use crate::obligations::touch::UiGraphTouchDescriptor;
use worth_ui_inspection::UiInspectionQuery;

use crate::facade::UiInspectionReceipt;

use super::UiSelectedObligation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSelectedObligationSet {
    touch: UiGraphTouchDescriptor,
    support_snapshot: UiSupportSnapshot,
    obligations: Box<[UiSelectedObligation]>,
    evidence_index: UiObligationEvidenceIndex,
}

impl UiSelectedObligationSet {
    pub(crate) fn new(
        touch: UiGraphTouchDescriptor,
        support_snapshot: UiSupportSnapshot,
        obligations: Box<[UiSelectedObligation]>,
        evidence_index: UiObligationEvidenceIndex,
    ) -> Self {
        Self {
            touch,
            support_snapshot,
            obligations,
            evidence_index,
        }
    }

    pub fn touch(&self) -> &UiGraphTouchDescriptor {
        &self.touch
    }

    pub fn support_snapshot(&self) -> &UiSupportSnapshot {
        &self.support_snapshot
    }

    pub fn obligations(&self) -> &[UiSelectedObligation] {
        &self.obligations
    }

    pub fn evidence_index(&self) -> &UiObligationEvidenceIndex {
        &self.evidence_index
    }

    pub fn selected_obligation_handles(
        &self,
    ) -> Box<[crate::obligations::inspection::UiObligationEvidenceHandle]> {
        self.obligations
            .iter()
            .map(UiSelectedObligation::evidence_handle)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn handoff(&self) -> UiObligationSelectionHandoff<'_> {
        UiObligationSelectionHandoff::new(self)
    }

    pub fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        UiInspectionReceipt::from_obligation(
            query.clone(),
            self.evidence_index
                .inspect(&UiObligationEvidenceQuery::from_inspection_query(&query)),
        )
    }
}
