use crate::declaration::stable_text_digest;
use worth_ui_inspection::{
    UiInspectionAdmissionPosture, UiInspectionQuery, UiInspectionRelevanceOutcome,
};

use crate::admission::inspection::UiAdmissionEvidenceRecord;
use crate::admission::{
    UiAdmissionAggregation, UiAdmissionDecision, UiAdmissionTarget, UiLegalityDecision,
    UiMeasurementAdmission, UiSupportSnapshot,
};
use crate::evidence::UiEvidenceAuthorityGeneration;
use crate::evidence::UiEvidenceRef;
use crate::facade::UiInspectionReceipt;
use crate::obligations::closeout::{UiAdmissionAuthorityHandoff, UiObligationCloseoutReport};
use crate::obligations::diagnostics::UiObligationDiagnosticProjection;
use crate::obligations::dispatch::UiObligationDispatchPlan;
use crate::obligations::inspection::{
    admitted_report_evidence_records, dispatch_evidence_records, verdict_evidence_records,
    UiObligationEvidenceHandle, UiObligationEvidenceIndex,
};
use crate::obligations::selection::UiSelectedObligationSet;
use crate::obligations::verdict::UiObligationVerdict;

use super::evidence_index_builders::aggregation_from_selected;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAdmissionReport {
    identity_digest: u64,
    authority_generation: UiEvidenceAuthorityGeneration,
    target: UiAdmissionTarget,
    support_snapshot: UiSupportSnapshot,
    measurement_admission: Option<UiMeasurementAdmission>,
    aggregation: UiAdmissionAggregation,
    legality_decision: Option<UiLegalityDecision>,
    dispatch_plan: Option<UiObligationDispatchPlan>,
    verdicts: Box<[UiObligationVerdict]>,
    evidence_index: UiObligationEvidenceIndex,
}

impl UiAdmissionReport {
    pub(crate) fn identity_digest_for_decision(decision: &UiAdmissionDecision) -> u64 {
        admission_report_identity_digest(
            decision.support_snapshot().target(),
            decision.aggregation(),
            decision.legality_decision(),
            None,
        )
    }

    pub(crate) fn from_decision(
        decision: UiAdmissionDecision,
        authority_generation: UiEvidenceAuthorityGeneration,
    ) -> Self {
        let aggregation = decision.aggregation();
        let legality_decision = decision.legality_decision().cloned();
        let target = decision.support_snapshot().target().clone();
        let support_snapshot = decision.support_snapshot().clone();
        let identity_digest = Self::identity_digest_for_decision(&decision);

        let report = Self {
            identity_digest,
            authority_generation,
            target,
            support_snapshot,
            measurement_admission: None,
            aggregation,
            legality_decision,
            dispatch_plan: None,
            verdicts: Box::new([]),
            evidence_index: UiObligationEvidenceIndex::empty(),
        };
        let evidence_index = UiObligationEvidenceIndex::new(
            admitted_report_evidence_records(&report).into_boxed_slice(),
        );

        report.with_evidence_index(evidence_index)
    }

    pub(crate) fn from_selected_execution(
        selected: &UiSelectedObligationSet,
        support_snapshot: UiSupportSnapshot,
        measurement_admission: Option<UiMeasurementAdmission>,
        dispatch_plan: UiObligationDispatchPlan,
        verdicts: Box<[UiObligationVerdict]>,
    ) -> Self {
        let aggregation = aggregation_from_selected(&support_snapshot, &verdicts);
        let identity_digest = admission_report_identity_digest(
            support_snapshot.target(),
            aggregation,
            None,
            Some(&verdicts),
        );
        let report = Self {
            identity_digest,
            authority_generation: selected.authority_generation(),
            target: support_snapshot.target().clone(),
            support_snapshot,
            measurement_admission,
            aggregation,
            legality_decision: None,
            dispatch_plan: Some(dispatch_plan),
            verdicts,
            evidence_index: UiObligationEvidenceIndex::empty(),
        };
        let evidence_index = selected
            .evidence_index()
            .with_appended(dispatch_evidence_records(
                report
                    .dispatch_plan()
                    .expect("selected execution report should retain dispatch plan"),
            ))
            .with_appended(verdict_evidence_records(selected, report.verdicts()))
            .with_appended(admitted_report_evidence_records(&report));

        report.with_evidence_index(evidence_index)
    }

    pub fn target(&self) -> &UiAdmissionTarget {
        &self.target
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    fn with_evidence_index(mut self, evidence_index: UiObligationEvidenceIndex) -> Self {
        self.evidence_index = evidence_index;
        self
    }

    pub fn authority_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.authority_generation
    }

    pub fn support_snapshot(&self) -> &UiSupportSnapshot {
        &self.support_snapshot
    }

    pub fn aggregation(&self) -> UiAdmissionAggregation {
        self.aggregation
    }

    pub fn measurement_admission(&self) -> Option<&UiMeasurementAdmission> {
        self.measurement_admission.as_ref()
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

    pub fn evidence_ref(&self) -> UiEvidenceRef {
        UiAdmissionEvidenceRecord::for_report(self).reference()
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
        let relevance_admission = query.admit_relevance();
        if !matches!(
            relevance_admission.outcome(),
            UiInspectionRelevanceOutcome::Matched
        ) {
            return UiInspectionReceipt::from_relevance_admission(
                query,
                relevance_admission,
                Some(self.authority_generation),
            );
        }
        UiInspectionReceipt::from_obligation(
            query.clone(),
            relevance_admission,
            self.authority_generation,
            self.evidence_index.inspect(
                &crate::obligations::inspection::UiObligationEvidenceQuery::from_inspection_query(
                    &query,
                ),
                self.authority_generation,
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

fn admission_report_identity_digest(
    target: &UiAdmissionTarget,
    aggregation: UiAdmissionAggregation,
    legality_decision: Option<&UiLegalityDecision>,
    verdicts: Option<&[UiObligationVerdict]>,
) -> u64 {
    let legality_digest = legality_decision
        .map(|decision| stable_text_digest(&format!("{decision:?}")))
        .unwrap_or(0);
    let verdict_digest = verdicts
        .unwrap_or(&[])
        .iter()
        .fold(0_u64, |digest, verdict| {
            digest ^ verdict.identity_digest().rotate_left(11)
        });

    stable_text_digest("admission-report")
        ^ target.graph_node_identity().digest().rotate_left(7)
        ^ stable_text_digest(&format!("{aggregation:?}")).rotate_left(17)
        ^ legality_digest.rotate_left(29)
        ^ verdict_digest.rotate_left(37)
}
