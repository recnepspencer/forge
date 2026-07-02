use crate::UiInspectionObligationReasonProjection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionObligationEvidenceReceipt {
    projections: Box<[UiInspectionObligationReasonProjection]>,
}

impl UiInspectionObligationEvidenceReceipt {
    pub fn new(projections: Box<[UiInspectionObligationReasonProjection]>) -> Self {
        Self { projections }
    }

    pub fn projections(&self) -> &[UiInspectionObligationReasonProjection] {
        &self.projections
    }
}
