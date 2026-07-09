use worth_proof::{Artifact, AuthorityWitness, Proof};

use super::authority::FoundationalDiagnosticProductionReadinessAuthority;
use super::report::FoundationalDiagnosticProductionReadinessReport;
use super::vocabulary::FoundationalDiagnosticProductionReadinessScope;
use crate::diagnostics::{
    FoundationalDiagnosticProductionReadinessCertified, FoundationalDiagnosticProductionTestReady,
};

pub type FoundationalDiagnosticProductionTestReadyArtifact = Artifact<
    FoundationalDiagnosticProductionTestReady,
    FoundationalDiagnosticProductionReadinessReport,
    Proof<
        FoundationalDiagnosticProductionReadinessCertified,
        FoundationalDiagnosticProductionReadinessAuthority,
    >,
    worth_proof::FreshnessScopedBasis<
        worth_proof::CurrentValidity,
        worth_proof::AssumptionBasis<FoundationalDiagnosticProductionReadinessScope>,
    >,
>;

pub fn foundational_diagnostic_milestone6_readiness_report(
) -> FoundationalDiagnosticProductionReadinessReport {
    FoundationalDiagnosticProductionReadinessReport::new()
}

pub fn certify_foundational_diagnostic_milestone6_production_test_readiness(
) -> FoundationalDiagnosticProductionTestReadyArtifact {
    let report = foundational_diagnostic_milestone6_readiness_report();
    assert!(report.passes_readiness_checklist());

    let authority = AuthorityWitness::from_authority_marker(
        FoundationalDiagnosticProductionReadinessAuthority::certification_boundary(),
    );
    let proof = Proof::from_authority_witness(&authority);

    Artifact::with_proofs_and_current_basis(
        report,
        proof,
        FoundationalDiagnosticProductionReadinessScope::milestone_6(),
        authority,
    )
}

pub fn require_foundational_diagnostic_milestone6_production_test_readiness(
    readiness: &FoundationalDiagnosticProductionTestReadyArtifact,
) -> &FoundationalDiagnosticProductionReadinessReport {
    readiness.payload()
}
