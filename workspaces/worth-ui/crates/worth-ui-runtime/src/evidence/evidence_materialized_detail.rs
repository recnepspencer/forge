use super::UiInspectionObligationEvidenceReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceMaterializedDetail {
    Obligation(UiInspectionObligationEvidenceReceipt),
}
