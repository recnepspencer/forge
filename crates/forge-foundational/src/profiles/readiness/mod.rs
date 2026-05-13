mod authority;
mod certification;
mod inventory;
mod report;
mod vocabulary;

pub use authority::FoundationalProfileProductionReadinessAuthority;
pub use certification::{
    certify_foundational_profile_milestone3_production_test_readiness,
    foundational_profile_milestone3_readiness_report,
    require_foundational_profile_milestone3_production_test_readiness,
    FoundationalProfileProductionTestReadyArtifact,
};
pub use report::FoundationalProfileProductionReadinessReport;
pub use vocabulary::{
    FoundationalProfileCertifiedSurface, FoundationalProfileCertifiedSurfaceEvidence,
    FoundationalProfileCompileFailBoundary, FoundationalProfileForgeProofApi,
    FoundationalProfileForgeProofForbiddenSurface, FoundationalProfileForgeProofSurface,
    FoundationalProfileMilestone3PhaseGate, FoundationalProfilePhaseGateEvidence,
    FoundationalProfileProductionReadinessScope, FoundationalProfileResidualDebt,
    FoundationalProfileRuntimeAssumption, FoundationalProfileRuntimeNonAssumption,
    FoundationalProfileSyntheticRuntimePressure,
};

#[cfg(test)]
mod tests {
    use forge_proof::AuthorityWitness;

    use super::FoundationalProfileProductionReadinessAuthority;

    #[test]
    fn production_readiness_authority_is_crate_controlled() {
        let _authority = AuthorityWitness::from_authority_marker(
            FoundationalProfileProductionReadinessAuthority::certification_boundary(),
        );
    }
}
