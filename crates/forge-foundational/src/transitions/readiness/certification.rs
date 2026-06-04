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
    FoundationalTransitionProductionReadinessReport::milestone_5()
}

pub fn foundational_transition_milestone9_scoped_merge_readiness_report(
) -> FoundationalTransitionProductionReadinessReport {
    FoundationalTransitionProductionReadinessReport::milestone_9_scoped_merge()
}

pub fn certify_foundational_transition_milestone5_production_test_readiness(
) -> FoundationalTransitionProductionTestReadyArtifact {
    let report = foundational_transition_milestone5_readiness_report();
    certify_transition_production_test_readiness(
        report,
        FoundationalTransitionProductionReadinessScope::milestone_5(),
    )
}

pub fn certify_foundational_transition_milestone9_scoped_merge_production_test_readiness(
) -> FoundationalTransitionProductionTestReadyArtifact {
    let report = foundational_transition_milestone9_scoped_merge_readiness_report();
    certify_transition_production_test_readiness(
        report,
        FoundationalTransitionProductionReadinessScope::milestone_9_scoped_merge(),
    )
}

fn certify_transition_production_test_readiness(
    report: FoundationalTransitionProductionReadinessReport,
    scope: FoundationalTransitionProductionReadinessScope,
) -> FoundationalTransitionProductionTestReadyArtifact {
    assert!(report.passes_readiness_checklist());

    let authority = AuthorityWitness::from_authority_marker(
        FoundationalTransitionProductionReadinessAuthority::certification_boundary(),
    );
    let proof = Proof::from_authority_witness(&authority);

    Artifact::with_proofs_and_current_basis(report, proof, scope, authority)
}

pub fn require_foundational_transition_milestone5_production_test_readiness(
    readiness: &FoundationalTransitionProductionTestReadyArtifact,
) -> &FoundationalTransitionProductionReadinessReport {
    readiness.payload()
}

pub fn require_foundational_transition_milestone9_scoped_merge_production_test_readiness(
    readiness: &FoundationalTransitionProductionTestReadyArtifact,
) -> &FoundationalTransitionProductionReadinessReport {
    readiness.payload()
}
