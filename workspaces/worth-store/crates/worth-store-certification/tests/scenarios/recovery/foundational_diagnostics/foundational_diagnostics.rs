#[path = "../../../support/recovery/foundational_evidence_support/foundational_evidence_support.rs"]
mod evidence_support;

use worth_foundational::{
    FoundationalCertifiedDiagnosticSourceKind, FoundationalDiagnosticAbsenceCause,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticRow, FoundationalDiagnosticRowFamily,
    FoundationalDiagnosticSupportEvidencePosture,
};
use worth_store_recovery_physics::{
    FoundationalRecoveryEvidenceBundle, RecoverySourceDecisionReport, RecoverySourceDiagnosticKind,
};

#[test]
fn recovery_diagnostics_materialize_each_required_typed_row_family() {
    let disagreement_source = evidence_support::verifier_disagreement_source();
    let report = RecoverySourceDecisionReport::from_source(&disagreement_source);

    let expected = [
        (
            RecoverySourceDiagnosticKind::SourceDecision,
            FoundationalDiagnosticRowFamily::Decision,
            FoundationalDiagnosticOutcomeKind::Accepted,
        ),
        (
            RecoverySourceDiagnosticKind::PartialPublicationOutcome,
            FoundationalDiagnosticRowFamily::Decision,
            FoundationalDiagnosticOutcomeKind::Partial,
        ),
        (
            RecoverySourceDiagnosticKind::VerifierDisagreement,
            FoundationalDiagnosticRowFamily::Comparison,
            FoundationalDiagnosticOutcomeKind::Mismatch,
        ),
        (
            RecoverySourceDiagnosticKind::MissingEvidence,
            FoundationalDiagnosticRowFamily::Support,
            FoundationalDiagnosticOutcomeKind::Advisory,
        ),
        (
            RecoverySourceDiagnosticKind::RedactedEvidence,
            FoundationalDiagnosticRowFamily::Support,
            FoundationalDiagnosticOutcomeKind::Advisory,
        ),
        (
            RecoverySourceDiagnosticKind::UnsupportedEvidence,
            FoundationalDiagnosticRowFamily::Support,
            FoundationalDiagnosticOutcomeKind::Unsupported,
        ),
        (
            RecoverySourceDiagnosticKind::NamedGap,
            FoundationalDiagnosticRowFamily::Support,
            FoundationalDiagnosticOutcomeKind::Deferred,
        ),
        (
            RecoverySourceDiagnosticKind::PartialCoverage,
            FoundationalDiagnosticRowFamily::Support,
            FoundationalDiagnosticOutcomeKind::Partial,
        ),
        (
            RecoverySourceDiagnosticKind::BudgetDenial,
            FoundationalDiagnosticRowFamily::Decision,
            FoundationalDiagnosticOutcomeKind::Denied,
        ),
    ];

    for (kind, family, outcome) in expected {
        let row = report
            .row_for(kind)
            .expect("diagnostic row is materialized");
        assert_eq!(row.family(), family);
        assert_eq!(row.outcome_kind(), outcome);
        assert_eq!(
            row.subject(),
            &worth_foundational::FoundationalDiagnosticSubject::BoundaryArtifact {
                artifact_locator: disagreement_source.artifact_locator()
            }
        );
    }

    assert_support_absence(
        report
            .row_for(RecoverySourceDiagnosticKind::MissingEvidence)
            .unwrap(),
        FoundationalDiagnosticAbsenceCause::MissingEvidence,
    );
    assert_support_absence(
        report
            .row_for(RecoverySourceDiagnosticKind::RedactedEvidence)
            .unwrap(),
        FoundationalDiagnosticAbsenceCause::Redacted,
    );
    assert_support_absence(
        report
            .row_for(RecoverySourceDiagnosticKind::UnsupportedEvidence)
            .unwrap(),
        FoundationalDiagnosticAbsenceCause::Unsupported,
    );
    assert_eq!(report.support_report().rows().len(), expected.len());
    assert_eq!(report.explanation_bundle().rows().len(), expected.len());
    assert!(!report.support_report().named_gaps().is_empty());

    let bundle = FoundationalRecoveryEvidenceBundle::from_source(&disagreement_source).unwrap();
    assert_eq!(
        bundle
            .readmitted_diagnostic_support_bundle()
            .unwrap()
            .source_kind(),
        FoundationalCertifiedDiagnosticSourceKind::CurrentBasisBoundaryBundle
    );
}

fn assert_support_absence(
    row: &FoundationalDiagnosticRow,
    cause: FoundationalDiagnosticAbsenceCause,
) {
    let FoundationalDiagnosticRow::Support(row) = row else {
        panic!("expected support row");
    };
    assert_eq!(
        row.evidence_posture(),
        &FoundationalDiagnosticSupportEvidencePosture::Absent(cause)
    );
}
