use worth_proof::{
    Artifact, AuthorityMarker, AuthorityProves, AuthorityWitness, PhaseMarker, Proof, ProofMarker,
};

use super::milestone1_readiness::{
    milestone1_migration_readiness_report, Milestone1MigrationReadinessReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone1ProductionTestReady;
impl PhaseMarker for Milestone1ProductionTestReady {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone1ProductionReadinessCertified;
impl ProofMarker for Milestone1ProductionReadinessCertified {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Milestone1ProductionReadinessScope {
    milestone: u8,
}

impl Milestone1ProductionReadinessScope {
    const fn milestone_1() -> Self {
        Self { milestone: 1 }
    }

    pub const fn milestone(&self) -> u8 {
        self.milestone
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Milestone1ProductionReadinessAuthority(());

impl Milestone1ProductionReadinessAuthority {
    const fn certification_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for Milestone1ProductionReadinessAuthority {}
impl AuthorityProves<Milestone1ProductionReadinessCertified>
    for Milestone1ProductionReadinessAuthority
{
}

pub type Milestone1ProductionTestReadyArtifact = Artifact<
    Milestone1ProductionTestReady,
    Milestone1MigrationReadinessReport,
    Proof<Milestone1ProductionReadinessCertified, Milestone1ProductionReadinessAuthority>,
    worth_proof::FreshnessScopedBasis<
        worth_proof::CurrentValidity,
        worth_proof::AssumptionBasis<Milestone1ProductionReadinessScope>,
    >,
>;

pub fn certify_milestone1_production_test_readiness() -> Milestone1ProductionTestReadyArtifact {
    let report = milestone1_migration_readiness_report();
    assert!(milestone1_report_passes_production_test_readiness(&report));

    let authority = AuthorityWitness::from_authority_marker(
        Milestone1ProductionReadinessAuthority::certification_boundary(),
    );
    let proof = Proof::from_authority_witness(&authority);

    Artifact::with_proofs_and_current_basis(
        report,
        proof,
        Milestone1ProductionReadinessScope::milestone_1(),
        authority,
    )
}

pub fn require_milestone1_production_test_readiness(
    readiness: &Milestone1ProductionTestReadyArtifact,
) -> &Milestone1MigrationReadinessReport {
    readiness.payload()
}

fn milestone1_report_passes_production_test_readiness(
    report: &Milestone1MigrationReadinessReport,
) -> bool {
    report.public_api().len() == 10
        && report.proof_seeds().len() == 10
        && report.compatibility_debt().len() == 1
        && report
            .public_api()
            .iter()
            .any(|surface| surface.name() == "compatibility_bridges")
        && report
            .public_api()
            .iter()
            .any(|surface| surface.name() == "aspect_common_path")
        && report
            .public_api()
            .iter()
            .any(|surface| surface.name() == "compatibility_common_path")
        && report
            .proof_seeds()
            .iter()
            .any(|seed| seed.name() == "digest_preparation_readiness")
        && report
            .proof_seeds()
            .iter()
            .any(|seed| seed.name() == "aspect_common_path_front_doors")
        && report
            .proof_seeds()
            .iter()
            .any(|seed| seed.name() == "compatibility_common_path_front_doors")
        && report.proof_seeds().iter().all(|seed| {
            seed.evidence().contains("certification/") || seed.evidence().contains("ui/")
        })
}
