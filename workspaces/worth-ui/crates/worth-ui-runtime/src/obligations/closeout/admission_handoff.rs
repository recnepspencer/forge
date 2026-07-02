use crate::admission::{
    UiAdmissionAggregation, UiAdmissionTarget, UiLegalityDecision, UiSupportSnapshot,
};
use crate::obligations::dispatch::UiObligationDispatchPlan;
use crate::obligations::inspection::{UiObligationEvidenceHandle, UiObligationEvidenceIndex};
use crate::obligations::verdict::UiObligationVerdict;

use super::UiObligationCloseoutReport;

#[derive(Clone, Copy)]
pub struct UiAdmissionAuthorityHandoff<'a> {
    report: &'a crate::admission::UiAdmissionReport,
}

impl<'a> UiAdmissionAuthorityHandoff<'a> {
    pub(crate) const fn new(report: &'a crate::admission::UiAdmissionReport) -> Self {
        Self { report }
    }

    pub fn target(self) -> &'a UiAdmissionTarget {
        self.report.target()
    }

    pub fn support_snapshot(self) -> &'a UiSupportSnapshot {
        self.report.support_snapshot()
    }

    pub fn aggregation(self) -> UiAdmissionAggregation {
        self.report.aggregation()
    }

    pub fn legality_decision(self) -> Option<&'a UiLegalityDecision> {
        self.report.legality_decision()
    }

    pub fn dispatch_plan(self) -> Option<&'a UiObligationDispatchPlan> {
        self.report.dispatch_plan()
    }

    pub fn verdicts(self) -> &'a [UiObligationVerdict] {
        self.report.verdicts()
    }

    pub fn verdict_evidence_handles(self) -> Box<[UiObligationEvidenceHandle]> {
        self.report.verdict_evidence_handles()
    }

    pub fn evidence_index(self) -> &'a UiObligationEvidenceIndex {
        self.report.evidence_index()
    }

    pub fn closeout_report(self) -> UiObligationCloseoutReport {
        self.report.closeout_report()
    }
}
