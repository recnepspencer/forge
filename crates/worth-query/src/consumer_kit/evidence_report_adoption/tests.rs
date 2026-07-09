use super::{
    evidence_report_adoption_audit, WorthQueryEvidenceReportAdoptionFindingKind,
    WorthQueryEvidenceReportAdoptionResidueClassification,
    WorthQueryEvidenceReportAdoptionSourceSet,
};
use crate::WorthQueryEvidenceScope;

#[test]
fn adoption_audit_rejects_digest_call_in_covered_source() {
    let report = evidence_report_adoption_audit()
        .covering_sources(
            WorthQueryEvidenceReportAdoptionSourceSet::new("worth-kernel").source_file(
                "covered",
                "covered.rs",
                "fn f(parts: &[String]) { let _ = digest_owned_parts(parts); }",
                WorthQueryEvidenceReportAdoptionResidueClassification::CoveredQueryEvidenceAdoption,
            ),
        )
        .evaluate()
        .expect("seeded covered source parses");

    assert_eq!(report.findings().len(), 1);
    assert_eq!(
        report.findings()[0].kind(),
        WorthQueryEvidenceReportAdoptionFindingKind::CoveredSurfaceUsesWorthDigest
    );
    assert_eq!(
        report.report_identity().scope(),
        WorthQueryEvidenceScope::ConsumerEvidenceReportAdoptionReport
    );
}

#[test]
fn adoption_audit_records_defended_residue_without_finding() {
    let report = evidence_report_adoption_audit()
        .covering_sources(
            WorthQueryEvidenceReportAdoptionSourceSet::new("worth-kernel").source_file(
                "defended",
                "defended.rs",
                "use crate::construction::digest::digest_owned_parts; fn f(parts: &[String]) { let _ = digest_owned_parts(parts); }",
                WorthQueryEvidenceReportAdoptionResidueClassification::DefendedDomainArtifactIdentity,
            ),
        )
        .evaluate()
        .expect("defended source parses");

    assert!(report.findings().is_empty());
    assert_eq!(report.residue_rows().len(), 1);
    assert_eq!(report.residue_rows()[0].symbol(), "digest_owned_parts");
    assert_eq!(
        report.residue_rows()[0].row_identity().scope(),
        WorthQueryEvidenceScope::ConsumerEvidenceReportAdoptionResidue
    );
}

#[test]
fn adoption_audit_rejects_unclassified_residue() {
    let report = evidence_report_adoption_audit()
        .covering_sources(
            WorthQueryEvidenceReportAdoptionSourceSet::new("worth-kernel").source_file(
                "unclassified",
                "unclassified.rs",
                "fn f() { let _ = ConstructionDigestScope::ArtifactIdentity; }",
                WorthQueryEvidenceReportAdoptionResidueClassification::Unclassified,
            ),
        )
        .evaluate()
        .expect("seeded unclassified source parses");

    assert_eq!(report.findings().len(), 1);
    assert_eq!(
        report.findings()[0].kind(),
        WorthQueryEvidenceReportAdoptionFindingKind::UnclassifiedWorthDigestResidue
    );
}
