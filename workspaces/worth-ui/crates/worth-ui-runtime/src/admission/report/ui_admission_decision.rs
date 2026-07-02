use crate::admission::{UiLegalityDecision, UiSupportSnapshot};

use super::UiAdmissionReport;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAdmissionAggregation {
    Denied,
    Unsupported,
    WrongWorld,
    Deferred,
    DiagnosticOnly,
    Admitted,
    AdmittedWithAdvisory,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAdmissionOutcome {
    Unsupported,
    WrongWorld,
    Deferred,
    DiagnosticOnly,
    Denied(UiLegalityDecision),
    Admitted(UiLegalityDecision),
    AdmittedWithAdvisory(UiLegalityDecision),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAdmissionDecision {
    support_snapshot: UiSupportSnapshot,
    outcome: UiAdmissionOutcome,
}

impl UiAdmissionDecision {
    pub(crate) fn new(support_snapshot: UiSupportSnapshot, outcome: UiAdmissionOutcome) -> Self {
        Self {
            support_snapshot,
            outcome,
        }
    }

    pub fn support_snapshot(&self) -> &UiSupportSnapshot {
        &self.support_snapshot
    }

    pub fn outcome(&self) -> &UiAdmissionOutcome {
        &self.outcome
    }

    pub fn aggregation(&self) -> UiAdmissionAggregation {
        match self.outcome() {
            UiAdmissionOutcome::Unsupported => UiAdmissionAggregation::Unsupported,
            UiAdmissionOutcome::WrongWorld => UiAdmissionAggregation::WrongWorld,
            UiAdmissionOutcome::Deferred => UiAdmissionAggregation::Deferred,
            UiAdmissionOutcome::DiagnosticOnly => UiAdmissionAggregation::DiagnosticOnly,
            UiAdmissionOutcome::Denied(_) => UiAdmissionAggregation::Denied,
            UiAdmissionOutcome::Admitted(_) => UiAdmissionAggregation::Admitted,
            UiAdmissionOutcome::AdmittedWithAdvisory(_) => {
                UiAdmissionAggregation::AdmittedWithAdvisory
            }
        }
    }

    pub fn legality_decision(&self) -> Option<&UiLegalityDecision> {
        match self.outcome() {
            UiAdmissionOutcome::Denied(decision)
            | UiAdmissionOutcome::Admitted(decision)
            | UiAdmissionOutcome::AdmittedWithAdvisory(decision) => Some(decision),
            UiAdmissionOutcome::Unsupported
            | UiAdmissionOutcome::WrongWorld
            | UiAdmissionOutcome::DiagnosticOnly
            | UiAdmissionOutcome::Deferred => None,
        }
    }

    pub fn into_report(self) -> UiAdmissionReport {
        UiAdmissionReport::from_decision(self)
    }
}
