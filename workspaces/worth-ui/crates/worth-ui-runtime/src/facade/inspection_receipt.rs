use worth_ui_inspection::{
    UiInspectionObligationEvidenceReceipt, UiInspectionPosture, UiInspectionQuery,
    UiInspectionSupportReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionReceipt {
    query: UiInspectionQuery,
    posture: Option<UiInspectionPosture>,
    support_report: Option<UiInspectionSupportReport>,
    obligation_evidence: Option<UiInspectionObligationEvidenceReceipt>,
}

impl UiInspectionReceipt {
    pub(crate) fn from_support(
        query: UiInspectionQuery,
        support_report: UiInspectionSupportReport,
    ) -> Self {
        let posture = UiInspectionPosture::from_support_report(support_report);
        Self {
            query,
            posture: Some(posture),
            support_report: Some(support_report),
            obligation_evidence: None,
        }
    }

    pub(crate) fn from_obligation(
        query: UiInspectionQuery,
        obligation_evidence: UiInspectionObligationEvidenceReceipt,
    ) -> Self {
        Self {
            query,
            posture: None,
            support_report: None,
            obligation_evidence: Some(obligation_evidence),
        }
    }

    pub fn query(&self) -> &UiInspectionQuery {
        &self.query
    }

    pub fn posture(&self) -> Option<UiInspectionPosture> {
        self.posture
    }

    pub fn support_report(&self) -> Option<UiInspectionSupportReport> {
        self.support_report
    }

    pub fn obligation_evidence(&self) -> Option<&UiInspectionObligationEvidenceReceipt> {
        self.obligation_evidence.as_ref()
    }
}
