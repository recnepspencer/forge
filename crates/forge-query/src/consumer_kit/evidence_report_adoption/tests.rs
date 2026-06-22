use super::{
    evidence_report_adoption_audit, ForgeQueryEvidenceReportAdoptionFindingKind,
    ForgeQueryEvidenceReportAdoptionResidueClassification,
    ForgeQueryEvidenceReportAdoptionSourceSet,
};
use crate::ForgeQueryEvidenceScope;

#[test]
fn adoption_audit_rejects_digest_call_in_covered_source() {
    let report = evidence_report_adoption_audit()
        .covering_sources(
            ForgeQueryEvidenceReportAdoptionSourceSet::new("worth-kernel").source_file(
                "covered",
                "covered.rs",
                "fn f(parts: &[String]) { let _ = digest_owned_parts(parts); }",
                ForgeQueryEvidenceReportAdoptionResidueClassification::CoveredQueryEvidenceAdoption,
            ),
        )
        .evaluate()
        .expect("seeded covered source parses");

    assert_eq!(report.findings().len(), 1);
    assert_eq!(
        report.findings()[0].kind(),
        ForgeQueryEvidenceReportAdoptionFindingKind::CoveredSurfaceUsesWorthDigest
    );
    assert_eq!(
        report.report_identity().scope(),
        ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionReport
    );
}

#[test]
fn adoption_audit_records_defended_residue_without_finding() {
    let report = evidence_report_adoption_audit()
        .covering_sources(
            ForgeQueryEvidenceReportAdoptionSourceSet::new("worth-kernel").source_file(
                "defended",
                "defended.rs",
                "use crate::construction::digest::digest_owned_parts; fn f(parts: &[String]) { let _ = digest_owned_parts(parts); }",
                ForgeQueryEvidenceReportAdoptionResidueClassification::DefendedDomainArtifactIdentity,
            ),
        )
        .evaluate()
        .expect("defended source parses");

    assert!(report.findings().is_empty());
    assert_eq!(report.residue_rows().len(), 1);
    assert_eq!(report.residue_rows()[0].symbol(), "digest_owned_parts");
    assert_eq!(
        report.residue_rows()[0].row_identity().scope(),
        ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionResidue
    );
}

#[test]
fn adoption_audit_rejects_unclassified_residue() {
    let report = evidence_report_adoption_audit()
        .covering_sources(
            ForgeQueryEvidenceReportAdoptionSourceSet::new("worth-kernel").source_file(
                "unclassified",
                "unclassified.rs",
                "fn f() { let _ = ConstructionDigestScope::ArtifactIdentity; }",
                ForgeQueryEvidenceReportAdoptionResidueClassification::Unclassified,
            ),
        )
        .evaluate()
        .expect("seeded unclassified source parses");

    assert_eq!(report.findings().len(), 1);
    assert_eq!(
        report.findings()[0].kind(),
        ForgeQueryEvidenceReportAdoptionFindingKind::UnclassifiedWorthDigestResidue
    );
}
