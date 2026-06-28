use forge_foundational::{
    BoundaryArtifactLocator, BoundarySourceLocator, FoundationalDiagnosticExplanationBundle,
    FoundationalDiagnosticRow, FoundationalDiagnosticSubject, FoundationalDiagnosticSupportReport,
};

use crate::OfflineRecoveryVerifierConclusion;

use super::super::executed_evidence_source::RecoveryPhysicsEvidenceSource;
use super::row_lowering::{diagnostic_code, foundational_row};
use super::support_materialization::{explanation_bundle, support_report};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoverySourceDiagnosticKind {
    SourceDecision,
    PartialPublicationOutcome,
    VerifierDisagreement,
    MissingEvidence,
    RedactedEvidence,
    UnsupportedEvidence,
    NamedGap,
    PartialCoverage,
    BudgetDenial,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverySourceDecisionReport {
    locator: BoundarySourceLocator,
    diagnostics: Vec<RecoverySourceDiagnosticKind>,
    foundational_rows: Vec<FoundationalDiagnosticRow>,
    support_report: FoundationalDiagnosticSupportReport,
    explanation_bundle: FoundationalDiagnosticExplanationBundle,
    verifier_conclusion: OfflineRecoveryVerifierConclusion,
}

impl RecoverySourceDecisionReport {
    pub fn from_source(source: &RecoveryPhysicsEvidenceSource) -> Self {
        Self::from_parts(
            source.source_locator().clone(),
            source.artifact_locator(),
            source.verifier_conclusion(),
            !source.verifier_state_agrees() || !source.verifier_counters_agree(),
        )
    }

    fn from_parts(
        locator: BoundarySourceLocator,
        artifact: BoundaryArtifactLocator,
        verifier_conclusion: OfflineRecoveryVerifierConclusion,
        verifier_disagrees: bool,
    ) -> Self {
        let mut diagnostics = vec![
            RecoverySourceDiagnosticKind::SourceDecision,
            RecoverySourceDiagnosticKind::PartialPublicationOutcome,
            RecoverySourceDiagnosticKind::MissingEvidence,
            RecoverySourceDiagnosticKind::RedactedEvidence,
            RecoverySourceDiagnosticKind::UnsupportedEvidence,
            RecoverySourceDiagnosticKind::NamedGap,
            RecoverySourceDiagnosticKind::PartialCoverage,
            RecoverySourceDiagnosticKind::BudgetDenial,
        ];
        if verifier_disagrees {
            diagnostics.push(RecoverySourceDiagnosticKind::VerifierDisagreement);
        }
        let foundational_rows = diagnostics
            .iter()
            .copied()
            .map(|kind| foundational_row(kind, artifact))
            .collect::<Vec<_>>();
        let subject = FoundationalDiagnosticSubject::BoundaryArtifact {
            artifact_locator: artifact,
        };
        Self {
            locator,
            verifier_conclusion,
            support_report: support_report(subject.clone(), &foundational_rows),
            explanation_bundle: explanation_bundle(subject, &foundational_rows),
            foundational_rows,
            diagnostics,
        }
    }

    pub const fn locator(&self) -> &BoundarySourceLocator {
        &self.locator
    }

    pub fn diagnostics(&self) -> &[RecoverySourceDiagnosticKind] {
        &self.diagnostics
    }

    pub const fn verifier_conclusion(&self) -> OfflineRecoveryVerifierConclusion {
        self.verifier_conclusion
    }

    pub fn foundational_rows(&self) -> &[FoundationalDiagnosticRow] {
        &self.foundational_rows
    }

    pub const fn support_report(&self) -> &FoundationalDiagnosticSupportReport {
        &self.support_report
    }

    pub const fn explanation_bundle(&self) -> &FoundationalDiagnosticExplanationBundle {
        &self.explanation_bundle
    }

    pub fn row_for(
        &self,
        kind: RecoverySourceDiagnosticKind,
    ) -> Option<&FoundationalDiagnosticRow> {
        let code = diagnostic_code(kind);
        self.foundational_rows
            .iter()
            .find(|row| row.code() == &code)
    }
}
