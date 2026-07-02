use worth_ui_inspection::{UiInspectionAdmissionPosture, UiInspectionQuery};

use crate::admission::{
    UiAdmissionAggregation, UiAdmissionDecision, UiAdmissionTarget, UiLegalityDecision,
    UiSupportSnapshot,
};
use crate::facade::UiInspectionReceipt;
use crate::obligations::closeout::{UiAdmissionAuthorityHandoff, UiObligationCloseoutReport};
use crate::obligations::diagnostics::UiObligationDiagnosticProjection;
use crate::obligations::dispatch::UiObligationDispatchPlan;
use crate::obligations::inspection::{UiObligationEvidenceHandle, UiObligationEvidenceIndex};
use crate::obligations::selection::UiSelectedObligationSet;
use crate::obligations::verdict::UiObligationVerdict;

use super::evidence_index_builders::{
    aggregation_from_selected, denial_evidence_index, verdict_evidence_records,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAdmissionReport {
    target: UiAdmissionTarget,
    support_snapshot: UiSupportSnapshot,
    aggregation: UiAdmissionAggregation,
    legality_decision: Option<UiLegalityDecision>,
    dispatch_plan: Option<UiObligationDispatchPlan>,
    verdicts: Box<[UiObligationVerdict]>,
    evidence_index: UiObligationEvidenceIndex,
}

impl UiAdmissionReport {
    pub(crate) fn from_decision(decision: UiAdmissionDecision) -> Self {
        let aggregation = decision.aggregation();
        let legality_decision = decision.legality_decision().cloned();
        let target = decision.support_snapshot().target().clone();
        let support_snapshot = decision.support_snapshot().clone();

        Self {
            target,
            support_snapshot,
            aggregation,
            legality_decision,
            dispatch_plan: None,
            verdicts: Box::new([]),
            evidence_index: denial_evidence_index(&decision),
        }
    }

    pub(crate) fn from_selected_execution(
        selected: &UiSelectedObligationSet,
        dispatch_plan: UiObligationDispatchPlan,
        verdicts: Box<[UiObligationVerdict]>,
    ) -> Self {
        let aggregation = aggregation_from_selected(selected, &verdicts);
        let evidence_index = selected
            .evidence_index()
            .with_appended(verdict_evidence_records(selected, &verdicts));

        Self {
            target: selected.support_snapshot().target().clone(),
            support_snapshot: selected.support_snapshot().clone(),
            aggregation,
            legality_decision: None,
            dispatch_plan: Some(dispatch_plan),
            verdicts,
            evidence_index,
        }
    }

    pub fn target(&self) -> &UiAdmissionTarget {
        &self.target
    }

    pub fn support_snapshot(&self) -> &UiSupportSnapshot {
        &self.support_snapshot
    }

    pub fn aggregation(&self) -> UiAdmissionAggregation {
        self.aggregation
    }

    pub fn legality_decision(&self) -> Option<&UiLegalityDecision> {
        self.legality_decision.as_ref()
    }

    pub fn dispatch_plan(&self) -> Option<&UiObligationDispatchPlan> {
        self.dispatch_plan.as_ref()
    }

    pub fn verdicts(&self) -> &[UiObligationVerdict] {
        &self.verdicts
    }

    pub fn evidence_index(&self) -> &UiObligationEvidenceIndex {
        &self.evidence_index
    }

    pub fn verdict_evidence_handles(&self) -> Box<[UiObligationEvidenceHandle]> {
        self.verdicts
            .iter()
            .map(UiObligationVerdict::evidence_handle)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn handoff(&self) -> UiAdmissionAuthorityHandoff<'_> {
        UiAdmissionAuthorityHandoff::new(self)
    }

    pub fn closeout_report(&self) -> UiObligationCloseoutReport {
        UiObligationCloseoutReport::milestone34()
    }

    pub fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        UiInspectionReceipt::from_obligation(
            query.clone(),
            self.evidence_index.inspect(
                &crate::obligations::inspection::UiObligationEvidenceQuery::from_inspection_query(
                    &query,
                ),
            ),
        )
    }

    pub fn diagnostic_projection(&self) -> UiObligationDiagnosticProjection {
        UiObligationDiagnosticProjection::from_evidence_index(&self.evidence_index)
    }

    pub fn inspection_posture(&self) -> UiInspectionAdmissionPosture {
        match self.legality_decision() {
            Some(decision) => match decision.posture() {
                crate::admission::UiLegalityPosture::Denied(
                    crate::admission::UiLegalityReason::MissingQueryPrerequisiteEvidence,
                )
                | crate::admission::UiLegalityPosture::Denied(
                    crate::admission::UiLegalityReason::MissingHostCapabilityReport,
                ) => UiInspectionAdmissionPosture::Unsupported,
                crate::admission::UiLegalityPosture::Denied(
                    crate::admission::UiLegalityReason::WrongQueryBasis { .. },
                ) => UiInspectionAdmissionPosture::WrongQueryBasis,
                crate::admission::UiLegalityPosture::Denied(
                    crate::admission::UiLegalityReason::WrongHostCapability { .. },
                ) => UiInspectionAdmissionPosture::WrongHostCapability,
                crate::admission::UiLegalityPosture::Denied(
                    crate::admission::UiLegalityReason::Stale { .. },
                ) => UiInspectionAdmissionPosture::Stale,
                crate::admission::UiLegalityPosture::Denied(
                    crate::admission::UiLegalityReason::Ambiguous { .. },
                ) => UiInspectionAdmissionPosture::Ambiguous,
                crate::admission::UiLegalityPosture::Denied(
                    crate::admission::UiLegalityReason::RebindRequired { .. },
                ) => UiInspectionAdmissionPosture::RebindRequired,
                crate::admission::UiLegalityPosture::Denied(
                    crate::admission::UiLegalityReason::BudgetExceeded { .. },
                ) => UiInspectionAdmissionPosture::BudgetExceeded,
                crate::admission::UiLegalityPosture::Denied(_) => {
                    UiInspectionAdmissionPosture::Denied
                }
                crate::admission::UiLegalityPosture::Admitted => {
                    UiInspectionAdmissionPosture::Admitted
                }
                crate::admission::UiLegalityPosture::AdmittedWithAdvisory(_) => {
                    UiInspectionAdmissionPosture::AdmittedWithAdvisory
                }
            },
            None => match self.aggregation() {
                UiAdmissionAggregation::Denied => UiInspectionAdmissionPosture::Denied,
                UiAdmissionAggregation::Unsupported => UiInspectionAdmissionPosture::Unsupported,
                UiAdmissionAggregation::WrongWorld => UiInspectionAdmissionPosture::WrongWorld,
                UiAdmissionAggregation::Deferred => UiInspectionAdmissionPosture::Deferred,
                UiAdmissionAggregation::DiagnosticOnly => {
                    UiInspectionAdmissionPosture::DiagnosticOnly
                }
                UiAdmissionAggregation::Admitted => UiInspectionAdmissionPosture::Admitted,
                UiAdmissionAggregation::AdmittedWithAdvisory => {
                    UiInspectionAdmissionPosture::AdmittedWithAdvisory
                }
            },
        }
    }
}
