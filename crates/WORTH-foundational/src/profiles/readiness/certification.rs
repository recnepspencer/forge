use worth_proof::{Artifact, AuthorityWitness, Proof};

use super::authority::FoundationalProfileProductionReadinessAuthority;
use super::report::FoundationalProfileProductionReadinessReport;
use super::vocabulary::FoundationalProfileProductionReadinessScope;
use crate::profiles::{
    FoundationalProfileProductionReadinessCertified, FoundationalProfileProductionTestReady,
};

pub type FoundationalProfileProductionTestReadyArtifact = Artifact<
    FoundationalProfileProductionTestReady,
    FoundationalProfileProductionReadinessReport,
    Proof<
        FoundationalProfileProductionReadinessCertified,
        FoundationalProfileProductionReadinessAuthority,
    >,
    worth_proof::FreshnessScopedBasis<
        worth_proof::CurrentValidity,
        worth_proof::AssumptionBasis<FoundationalProfileProductionReadinessScope>,
    >,
>;

pub fn foundational_profile_milestone3_readiness_report(
) -> FoundationalProfileProductionReadinessReport {
    FoundationalProfileProductionReadinessReport::new()
}

pub fn certify_foundational_profile_milestone3_production_test_readiness(
) -> FoundationalProfileProductionTestReadyArtifact {
    let report = foundational_profile_milestone3_readiness_report();
    assert!(report.passes_readiness_checklist());

    let authority = AuthorityWitness::from_authority_marker(
        FoundationalProfileProductionReadinessAuthority::certification_boundary(),
    );
    let proof = Proof::from_authority_witness(&authority);

    Artifact::with_proofs_and_current_basis(
        report,
        proof,
        FoundationalProfileProductionReadinessScope::milestone_3(),
        authority,
    )
}

pub fn require_foundational_profile_milestone3_production_test_readiness(
    readiness: &FoundationalProfileProductionTestReadyArtifact,
) -> &FoundationalProfileProductionReadinessReport {
    readiness.payload()
}
