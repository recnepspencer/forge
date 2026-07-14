use worth_foundational::{
    BoundaryArtifactLocator, FoundationalDiagnosticAbsenceCause, FoundationalDiagnosticCodeId,
    FoundationalDiagnosticComparisonRow, FoundationalDiagnosticDecisionRow,
    FoundationalDiagnosticDenialClass, FoundationalDiagnosticEvidencePosture,
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticLocator,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticRow, FoundationalDiagnosticScopeId,
    FoundationalDiagnosticSemanticLabelSet, FoundationalDiagnosticSeverity,
    FoundationalDiagnosticSubject, FoundationalDiagnosticSupportEvidencePosture,
    FoundationalDiagnosticSupportRow, FoundationalDiagnosticWidenedFalloutPosture,
};

use super::source_decision_report::RecoverySourceDiagnosticKind;

pub(crate) fn foundational_row(
    kind: RecoverySourceDiagnosticKind,
    artifact: BoundaryArtifactLocator,
) -> FoundationalDiagnosticRow {
    let code = diagnostic_code(kind);
    let scope = FoundationalDiagnosticScopeId::new("store.recovery.s4").expect("static scope");
    let subject = FoundationalDiagnosticSubject::BoundaryArtifact {
        artifact_locator: artifact,
    };
    let locator = FoundationalDiagnosticLocator::BoundaryArtifact(artifact);
    let labels = FoundationalDiagnosticSemanticLabelSet::new([code.clone()]);
    match kind {
        RecoverySourceDiagnosticKind::VerifierDisagreement => {
            FoundationalDiagnosticRow::Comparison(FoundationalDiagnosticComparisonRow::new(
                code,
                scope,
                FoundationalDiagnosticSeverity::Warning,
                subject,
                locator.clone(),
                FoundationalDiagnosticOutcomeKind::Mismatch,
                labels,
                Some(locator),
                FoundationalDiagnosticEvidencePosture::RetainedDirect,
            ))
        }
        RecoverySourceDiagnosticKind::MissingEvidence
        | RecoverySourceDiagnosticKind::RedactedEvidence
        | RecoverySourceDiagnosticKind::UnsupportedEvidence
        | RecoverySourceDiagnosticKind::NamedGap
        | RecoverySourceDiagnosticKind::PartialCoverage => {
            FoundationalDiagnosticRow::Support(FoundationalDiagnosticSupportRow::new(
                code,
                scope,
                FoundationalDiagnosticSeverity::Advisory,
                subject,
                locator,
                support_outcome(kind),
                labels,
                support_posture(kind),
                FoundationalDiagnosticLocalityClaim::ExactSubject,
                FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
            ))
        }
        RecoverySourceDiagnosticKind::SourceDecision
        | RecoverySourceDiagnosticKind::PartialPublicationOutcome
        | RecoverySourceDiagnosticKind::BudgetDenial => {
            FoundationalDiagnosticRow::Decision(FoundationalDiagnosticDecisionRow::new(
                code,
                scope,
                decision_severity(kind),
                subject,
                locator,
                decision_outcome(kind),
                labels,
                decision_denial(kind),
                FoundationalDiagnosticLocalityClaim::ExactSubject,
                FoundationalDiagnosticWidenedFalloutPosture::NotWidened,
            ))
        }
    }
}

pub(crate) fn diagnostic_code(kind: RecoverySourceDiagnosticKind) -> FoundationalDiagnosticCodeId {
    let code = match kind {
        RecoverySourceDiagnosticKind::SourceDecision => "source-decision",
        RecoverySourceDiagnosticKind::PartialPublicationOutcome => "partial-publication",
        RecoverySourceDiagnosticKind::VerifierDisagreement => "verifier-disagreement",
        RecoverySourceDiagnosticKind::MissingEvidence => "missing-evidence",
        RecoverySourceDiagnosticKind::RedactedEvidence => "redacted-evidence",
        RecoverySourceDiagnosticKind::UnsupportedEvidence => "unsupported-evidence",
        RecoverySourceDiagnosticKind::NamedGap => "named-gap",
        RecoverySourceDiagnosticKind::PartialCoverage => "partial-coverage",
        RecoverySourceDiagnosticKind::BudgetDenial => "budget-denial",
    };
    FoundationalDiagnosticCodeId::new(code).expect("static diagnostic code")
}

fn support_outcome(kind: RecoverySourceDiagnosticKind) -> FoundationalDiagnosticOutcomeKind {
    match kind {
        RecoverySourceDiagnosticKind::UnsupportedEvidence => {
            FoundationalDiagnosticOutcomeKind::Unsupported
        }
        RecoverySourceDiagnosticKind::PartialCoverage => FoundationalDiagnosticOutcomeKind::Partial,
        RecoverySourceDiagnosticKind::NamedGap => FoundationalDiagnosticOutcomeKind::Deferred,
        _ => FoundationalDiagnosticOutcomeKind::Advisory,
    }
}

fn support_posture(
    kind: RecoverySourceDiagnosticKind,
) -> FoundationalDiagnosticSupportEvidencePosture {
    match kind {
        RecoverySourceDiagnosticKind::MissingEvidence => {
            FoundationalDiagnosticSupportEvidencePosture::Absent(
                FoundationalDiagnosticAbsenceCause::MissingEvidence,
            )
        }
        RecoverySourceDiagnosticKind::RedactedEvidence => {
            FoundationalDiagnosticSupportEvidencePosture::Absent(
                FoundationalDiagnosticAbsenceCause::Redacted,
            )
        }
        RecoverySourceDiagnosticKind::UnsupportedEvidence => {
            FoundationalDiagnosticSupportEvidencePosture::Absent(
                FoundationalDiagnosticAbsenceCause::Unsupported,
            )
        }
        _ => FoundationalDiagnosticSupportEvidencePosture::Present(
            FoundationalDiagnosticEvidencePosture::Summarized,
        ),
    }
}

fn decision_severity(kind: RecoverySourceDiagnosticKind) -> FoundationalDiagnosticSeverity {
    match kind {
        RecoverySourceDiagnosticKind::BudgetDenial => FoundationalDiagnosticSeverity::Denial,
        _ => FoundationalDiagnosticSeverity::Info,
    }
}

fn decision_outcome(kind: RecoverySourceDiagnosticKind) -> FoundationalDiagnosticOutcomeKind {
    match kind {
        RecoverySourceDiagnosticKind::BudgetDenial => FoundationalDiagnosticOutcomeKind::Denied,
        RecoverySourceDiagnosticKind::PartialPublicationOutcome => {
            FoundationalDiagnosticOutcomeKind::Partial
        }
        _ => FoundationalDiagnosticOutcomeKind::Accepted,
    }
}

fn decision_denial(
    kind: RecoverySourceDiagnosticKind,
) -> Option<FoundationalDiagnosticDenialClass> {
    match kind {
        RecoverySourceDiagnosticKind::BudgetDenial => {
            Some(FoundationalDiagnosticDenialClass::PolicyDenied)
        }
        _ => None,
    }
}
