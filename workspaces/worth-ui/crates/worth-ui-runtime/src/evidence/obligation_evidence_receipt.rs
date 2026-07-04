use super::{UiEvidenceRef, UiInspectionObligationReasonProjection};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionObligationEvidenceReceipt {
    refs: Box<[UiEvidenceRef]>,
    projections: Box<[UiInspectionObligationReasonProjection]>,
}

impl UiInspectionObligationEvidenceReceipt {
    pub(crate) fn new(
        refs: Box<[UiEvidenceRef]>,
        projections: Box<[UiInspectionObligationReasonProjection]>,
    ) -> Self {
        Self { refs, projections }
    }

    pub fn refs(&self) -> &[UiEvidenceRef] {
        &self.refs
    }

    pub fn projections(&self) -> &[UiInspectionObligationReasonProjection] {
        &self.projections
    }
}
