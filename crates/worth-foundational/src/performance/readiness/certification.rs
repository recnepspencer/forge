use worth_proof::{Artifact, AuthorityWitness, Proof};

use super::authority::FoundationalPerformanceProductionReadinessAuthority;
use super::report::FoundationalPerformanceProductionReadinessReport;
use super::vocabulary::FoundationalPerformanceProductionReadinessScope;
use crate::performance::{
    FoundationalPerformanceProductionReadinessCertified, FoundationalPerformanceProductionTestReady,
};

pub type FoundationalPerformanceProductionTestReadyArtifact = Artifact<
    FoundationalPerformanceProductionTestReady,
    FoundationalPerformanceProductionReadinessReport,
    Proof<
        FoundationalPerformanceProductionReadinessCertified,
        FoundationalPerformanceProductionReadinessAuthority,
    >,
    worth_proof::FreshnessScopedBasis<
        worth_proof::CurrentValidity,
        worth_proof::AssumptionBasis<FoundationalPerformanceProductionReadinessScope>,
    >,
>;

pub fn foundational_performance_milestone8_readiness_report(
) -> FoundationalPerformanceProductionReadinessReport {
    FoundationalPerformanceProductionReadinessReport::new()
}

pub fn certify_foundational_performance_milestone8_production_test_readiness(
) -> FoundationalPerformanceProductionTestReadyArtifact {
    let report = foundational_performance_milestone8_readiness_report();
    assert!(report.passes_readiness_checklist());

    let authority = AuthorityWitness::from_authority_marker(
        FoundationalPerformanceProductionReadinessAuthority::certification_boundary(),
    );
    let proof = Proof::from_authority_witness(&authority);

    Artifact::with_proofs_and_current_basis(
        report,
        proof,
        FoundationalPerformanceProductionReadinessScope::milestone_8(),
        authority,
    )
}

pub fn require_foundational_performance_milestone8_production_test_readiness(
    readiness: &FoundationalPerformanceProductionTestReadyArtifact,
) -> &FoundationalPerformanceProductionReadinessReport {
    readiness.payload()
}
