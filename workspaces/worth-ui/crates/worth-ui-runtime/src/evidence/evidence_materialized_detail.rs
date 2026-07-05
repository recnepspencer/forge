use super::UiInspectionObligationEvidenceReceipt;
use worth_ui_inspection::UiInspectionMeasurementEvidenceView;

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceMaterializedDetail {
    Obligation(UiInspectionObligationEvidenceReceipt),
    Measurement(UiInspectionMeasurementEvidenceView),
}
