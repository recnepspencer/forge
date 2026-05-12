use forge_proof::{Artifact, AuthorityWitness, Proof};

use super::super::{CanonicalProductionReadinessCertified, CanonicalProductionTestReady};
use super::authority::CanonicalProductionReadinessAuthority;
use super::report::CanonicalProductionReadinessReport;
use super::vocabulary::CanonicalProductionReadinessScope;

pub type CanonicalProductionTestReadyArtifact = Artifact<
    CanonicalProductionTestReady,
    CanonicalProductionReadinessReport,
    Proof<CanonicalProductionReadinessCertified, CanonicalProductionReadinessAuthority>,
    forge_proof::FreshnessScopedBasis<
        forge_proof::CurrentValidity,
        forge_proof::AssumptionBasis<CanonicalProductionReadinessScope>,
    >,
>;

pub fn canonical_milestone2_production_readiness_report() -> CanonicalProductionReadinessReport {
    CanonicalProductionReadinessReport::new()
}

pub fn certify_canonical_milestone2_production_readiness() -> CanonicalProductionTestReadyArtifact {
    let report = canonical_milestone2_production_readiness_report();
    assert!(report.passes_readiness_checklist());

    let authority = AuthorityWitness::from_authority_marker(
        CanonicalProductionReadinessAuthority::certification_boundary(),
    );
    let proof = Proof::from_authority_witness(&authority);

    Artifact::with_proofs_and_current_basis(
        report,
        proof,
        CanonicalProductionReadinessScope::milestone_2(),
        authority,
    )
}

pub fn require_canonical_production_test_readiness(
    readiness: &CanonicalProductionTestReadyArtifact,
) -> &CanonicalProductionReadinessReport {
    readiness.payload()
}
