use crate::foundational_evidence_support as evidence_support;

use worth_foundational::{
    FoundationalCertifiedDiagnosticSourceKind, FoundationalCertifiedPerformanceClass,
    FoundationalCertifiedPerformanceSourceKind, FoundationalDiagnosticCertifiedCoverageClass,
};
use worth_store_recovery_physics::RecoveryEvidenceCanonicalBasis;

#[test]
fn executed_recovery_findings_materialize_equivalent_independent_surfaces() {
    let first = evidence_support::verified_source();
    let second = evidence_support::verified_source();

    let first_bundle = evidence_support::bundle_from_source(&first);
    let second_bundle = evidence_support::bundle_from_source(&second);
    let first_independent = RecoveryEvidenceCanonicalBasis::full(&first).unwrap();
    let second_independent = RecoveryEvidenceCanonicalBasis::full(&second).unwrap();

    assert_eq!(first_bundle.receipt(), second_bundle.receipt());
    assert_eq!(first_bundle.report(), second_bundle.report());
    assert_eq!(first_bundle.performance(), second_bundle.performance());
    assert_eq!(first_bundle.canonical_basis(), &first_independent);
    assert_eq!(second_bundle.canonical_basis(), &second_independent);
    assert_eq!(
        first_independent.digest().value().bytes(),
        second_independent.digest().value().bytes()
    );
    assert_eq!(
        first_bundle
            .certified_diagnostic_support_bundle()
            .unwrap()
            .source_kind(),
        FoundationalCertifiedDiagnosticSourceKind::CurrentBasisBoundaryBundle
    );
    assert_eq!(
        first_bundle
            .certified_diagnostic_support_bundle()
            .unwrap()
            .coverage_class(),
        FoundationalDiagnosticCertifiedCoverageClass::HostileCoveragePresent
    );
    assert_eq!(
        first_bundle
            .performance()
            .certified_support_expansion()
            .unwrap()
            .source_kind(),
        FoundationalCertifiedPerformanceSourceKind::MaterializedSupportExpansionReport
    );
    assert_eq!(
        first_bundle
            .performance()
            .certified_support_expansion()
            .unwrap()
            .certified_class(),
        FoundationalCertifiedPerformanceClass::SupportExpansionCompatibility
    );
}
