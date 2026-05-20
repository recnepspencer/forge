use forge_proof::{Artifact, AuthorityWitness, Proof};

use super::authority::FoundationalBoundaryEvidenceProductionReadinessAuthority;
use super::report::FoundationalBoundaryEvidenceProductionReadinessReport;
use super::vocabulary::FoundationalBoundaryEvidenceProductionReadinessScope;
use crate::boundary_evidence::{
    FoundationalBoundaryEvidenceProductionReadinessCertified,
    FoundationalBoundaryEvidenceProductionTestReady,
};

pub type FoundationalBoundaryEvidenceProductionTestReadyArtifact = Artifact<
    FoundationalBoundaryEvidenceProductionTestReady,
    FoundationalBoundaryEvidenceProductionReadinessReport,
    Proof<
        FoundationalBoundaryEvidenceProductionReadinessCertified,
        FoundationalBoundaryEvidenceProductionReadinessAuthority,
    >,
    forge_proof::FreshnessScopedBasis<
        forge_proof::CurrentValidity,
        forge_proof::AssumptionBasis<FoundationalBoundaryEvidenceProductionReadinessScope>,
    >,
>;

pub fn foundational_boundary_evidence_milestone7_readiness_report(
) -> FoundationalBoundaryEvidenceProductionReadinessReport {
    FoundationalBoundaryEvidenceProductionReadinessReport::new()
}

pub fn certify_foundational_boundary_evidence_milestone7_production_test_readiness(
) -> FoundationalBoundaryEvidenceProductionTestReadyArtifact {
    let report = foundational_boundary_evidence_milestone7_readiness_report();
    assert!(report.passes_readiness_checklist());

    let authority = AuthorityWitness::from_authority_marker(
        FoundationalBoundaryEvidenceProductionReadinessAuthority::certification_boundary(),
    );
    let proof = Proof::from_authority_witness(&authority);

    Artifact::with_proofs_and_current_basis(
        report,
        proof,
        FoundationalBoundaryEvidenceProductionReadinessScope::milestone_7(),
        authority,
    )
}

pub fn require_foundational_boundary_evidence_milestone7_production_test_readiness(
    readiness: &FoundationalBoundaryEvidenceProductionTestReadyArtifact,
) -> &FoundationalBoundaryEvidenceProductionReadinessReport {
    readiness.payload()
}
