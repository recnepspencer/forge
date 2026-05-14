use forge_proof::{Artifact, AuthorityWitness, Proof};

use super::authority::FoundationalTransitionProductionReadinessAuthority;
use super::report::FoundationalTransitionProductionReadinessReport;
use super::vocabulary::FoundationalTransitionProductionReadinessScope;
use crate::transitions::{
    FoundationalTransitionProductionReadinessCertified, FoundationalTransitionProductionTestReady,
};

pub type FoundationalTransitionProductionTestReadyArtifact = Artifact<
    FoundationalTransitionProductionTestReady,
    FoundationalTransitionProductionReadinessReport,
    Proof<
        FoundationalTransitionProductionReadinessCertified,
        FoundationalTransitionProductionReadinessAuthority,
    >,
    forge_proof::FreshnessScopedBasis<
        forge_proof::CurrentValidity,
        forge_proof::AssumptionBasis<FoundationalTransitionProductionReadinessScope>,
    >,
>;

pub fn foundational_transition_milestone5_readiness_report(
) -> FoundationalTransitionProductionReadinessReport {
    FoundationalTransitionProductionReadinessReport::new()
}

pub fn certify_foundational_transition_milestone5_production_test_readiness(
) -> FoundationalTransitionProductionTestReadyArtifact {
    let report = foundational_transition_milestone5_readiness_report();
    assert!(report.passes_readiness_checklist());

    let authority = AuthorityWitness::from_authority_marker(
        FoundationalTransitionProductionReadinessAuthority::certification_boundary(),
    );
    let proof = Proof::from_authority_witness(&authority);

    Artifact::with_proofs_and_current_basis(
        report,
        proof,
        FoundationalTransitionProductionReadinessScope::milestone_5(),
        authority,
    )
}

pub fn require_foundational_transition_milestone5_production_test_readiness(
    readiness: &FoundationalTransitionProductionTestReadyArtifact,
) -> &FoundationalTransitionProductionReadinessReport {
    readiness.payload()
}
