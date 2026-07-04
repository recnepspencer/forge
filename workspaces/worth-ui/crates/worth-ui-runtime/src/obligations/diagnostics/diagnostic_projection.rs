use crate::obligations::inspection::UiObligationEvidenceIndex;
use worth_ui_inspection::{
    UiInspectionEvidenceSource, UiInspectionObligationDecision,
    UiInspectionObligationDenialPosture, UiInspectionObligationDispatchPosture,
    UiInspectionObligationFamily, UiInspectionObligationLegalityReason,
    UiInspectionObligationNonSelectionReason, UiInspectionObligationSelectionReason,
    UiInspectionObligationVerdictClass, UiInspectionObligationVerdictPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiObligationDiagnosticRow {
    handle_digest: u64,
    family: Option<UiInspectionObligationFamily>,
    decision: UiInspectionObligationDecision,
    dispatch_posture: Option<UiInspectionObligationDispatchPosture>,
    verdict_class: Option<UiInspectionObligationVerdictClass>,
    verdict_posture: Option<UiInspectionObligationVerdictPosture>,
    denial_posture: Option<UiInspectionObligationDenialPosture>,
    selection_reasons: Box<[UiInspectionObligationSelectionReason]>,
    non_selection_reason: Option<UiInspectionObligationNonSelectionReason>,
    legality_reason: Option<UiInspectionObligationLegalityReason>,
    prerequisite_sources: Box<[UiInspectionEvidenceSource]>,
}

impl UiObligationDiagnosticRow {
    pub(crate) fn new(
        handle_digest: u64,
        family: Option<UiInspectionObligationFamily>,
        decision: UiInspectionObligationDecision,
        dispatch_posture: Option<UiInspectionObligationDispatchPosture>,
        verdict_class: Option<UiInspectionObligationVerdictClass>,
        verdict_posture: Option<UiInspectionObligationVerdictPosture>,
        denial_posture: Option<UiInspectionObligationDenialPosture>,
        selection_reasons: Box<[UiInspectionObligationSelectionReason]>,
        non_selection_reason: Option<UiInspectionObligationNonSelectionReason>,
        legality_reason: Option<UiInspectionObligationLegalityReason>,
        prerequisite_sources: Box<[UiInspectionEvidenceSource]>,
    ) -> Self {
        Self {
            handle_digest,
            family,
            decision,
            dispatch_posture,
            verdict_class,
            verdict_posture,
            denial_posture,
            selection_reasons,
            non_selection_reason,
            legality_reason,
            prerequisite_sources,
        }
    }

    pub fn handle_digest(&self) -> u64 {
        self.handle_digest
    }

    pub fn family(&self) -> Option<UiInspectionObligationFamily> {
        self.family
    }

    pub fn decision(&self) -> UiInspectionObligationDecision {
        self.decision
    }

    pub fn dispatch_posture(&self) -> Option<UiInspectionObligationDispatchPosture> {
        self.dispatch_posture
    }

    pub fn verdict_class(&self) -> Option<UiInspectionObligationVerdictClass> {
        self.verdict_class
    }

    pub fn verdict_posture(&self) -> Option<UiInspectionObligationVerdictPosture> {
        self.verdict_posture
    }

    pub fn denial_posture(&self) -> Option<UiInspectionObligationDenialPosture> {
        self.denial_posture
    }

    pub fn selection_reasons(&self) -> &[UiInspectionObligationSelectionReason] {
        &self.selection_reasons
    }

    pub fn non_selection_reason(&self) -> Option<UiInspectionObligationNonSelectionReason> {
        self.non_selection_reason
    }

    pub fn legality_reason(&self) -> Option<UiInspectionObligationLegalityReason> {
        self.legality_reason
    }

    pub fn prerequisite_sources(&self) -> &[UiInspectionEvidenceSource] {
        &self.prerequisite_sources
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiObligationDiagnosticProjection {
    rows: Box<[UiObligationDiagnosticRow]>,
}

impl UiObligationDiagnosticProjection {
    pub(crate) fn from_evidence_index(index: &UiObligationEvidenceIndex) -> Self {
        let rows = index
            .records()
            .iter()
            .map(|record| {
                let projection = record.to_projection();
                UiObligationDiagnosticRow::new(
                    projection.handle_digest(),
                    projection.family(),
                    projection.decision(),
                    projection.dispatch_posture(),
                    projection.verdict_class(),
                    projection.verdict_posture(),
                    projection.denial_posture(),
                    projection.selection_reasons().to_vec().into_boxed_slice(),
                    projection.non_selection_reason(),
                    projection.legality_reason(),
                    projection
                        .prerequisite_sources()
                        .to_vec()
                        .into_boxed_slice(),
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self { rows }
    }

    pub fn rows(&self) -> &[UiObligationDiagnosticRow] {
        &self.rows
    }
}
