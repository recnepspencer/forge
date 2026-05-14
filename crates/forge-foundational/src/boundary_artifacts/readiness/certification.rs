use forge_proof::{Artifact, AuthorityWitness, Proof};

use super::authority::FoundationalBoundaryArtifactProductionReadinessAuthority;
use super::report::FoundationalBoundaryArtifactProductionReadinessReport;
use super::vocabulary::FoundationalBoundaryArtifactProductionReadinessScope;
use crate::boundary_artifacts::{
    FoundationalBoundaryArtifactProductionReadinessCertified,
    FoundationalBoundaryArtifactProductionTestReady,
};

pub type FoundationalBoundaryArtifactProductionTestReadyArtifact = Artifact<
    FoundationalBoundaryArtifactProductionTestReady,
    FoundationalBoundaryArtifactProductionReadinessReport,
    Proof<
        FoundationalBoundaryArtifactProductionReadinessCertified,
        FoundationalBoundaryArtifactProductionReadinessAuthority,
    >,
    forge_proof::FreshnessScopedBasis<
        forge_proof::CurrentValidity,
        forge_proof::AssumptionBasis<FoundationalBoundaryArtifactProductionReadinessScope>,
    >,
>;

pub fn foundational_boundary_artifact_milestone4_readiness_report(
) -> FoundationalBoundaryArtifactProductionReadinessReport {
    FoundationalBoundaryArtifactProductionReadinessReport::new()
}

pub fn certify_foundational_boundary_artifact_milestone4_production_test_readiness(
) -> FoundationalBoundaryArtifactProductionTestReadyArtifact {
    let report = foundational_boundary_artifact_milestone4_readiness_report();
    assert!(report.passes_readiness_checklist());

    let authority = AuthorityWitness::from_authority_marker(
        FoundationalBoundaryArtifactProductionReadinessAuthority::certification_boundary(),
    );
    let proof = Proof::from_authority_witness(&authority);

    Artifact::with_proofs_and_current_basis(
        report,
        proof,
        FoundationalBoundaryArtifactProductionReadinessScope::milestone_4(),
        authority,
    )
}

pub fn require_foundational_boundary_artifact_milestone4_production_test_readiness(
    readiness: &FoundationalBoundaryArtifactProductionTestReadyArtifact,
) -> &FoundationalBoundaryArtifactProductionReadinessReport {
    readiness.payload()
}
