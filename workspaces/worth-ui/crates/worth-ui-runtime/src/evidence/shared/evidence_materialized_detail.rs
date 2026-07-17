use crate::evidence::{UiAllocationPlanningEvidenceDetail, UiInspectionObligationEvidenceReceipt};
use worth_ui_inspection::UiInspectionMeasurementEvidenceView;

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceMaterializedDetail {
    AllocationPlanning(Box<UiAllocationPlanningEvidenceDetail>),
    Obligation(UiInspectionObligationEvidenceReceipt),
    Measurement(UiInspectionMeasurementEvidenceView),
}
