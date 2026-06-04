mod authority;
mod certification;
mod inventory;
mod report;
mod scoped_inventory;
mod vocabulary;

pub use authority::FoundationalTransitionProductionReadinessAuthority;
pub use certification::{
    certify_foundational_transition_milestone5_production_test_readiness,
    certify_foundational_transition_milestone9_scoped_merge_production_test_readiness,
    foundational_transition_milestone5_readiness_report,
    foundational_transition_milestone9_scoped_merge_readiness_report,
    require_foundational_transition_milestone5_production_test_readiness,
    require_foundational_transition_milestone9_scoped_merge_production_test_readiness,
    FoundationalTransitionProductionTestReadyArtifact,
};
pub use report::FoundationalTransitionProductionReadinessReport;
pub use vocabulary::{
    FoundationalTransitionCertifiedSurface, FoundationalTransitionCertifiedSurfaceEvidence,
    FoundationalTransitionCompileFailBoundary, FoundationalTransitionCompileFailEvidence,
    FoundationalTransitionForgeProofApi, FoundationalTransitionForgeProofApiEvidence,
    FoundationalTransitionForgeProofForbiddenSurface, FoundationalTransitionForgeProofSurface,
    FoundationalTransitionMilestone5PhaseGate, FoundationalTransitionPhaseGateEvidence,
    FoundationalTransitionProductionReadinessScope, FoundationalTransitionResidualDebt,
    FoundationalTransitionRuntimeAssumption, FoundationalTransitionRuntimeNonAssumption,
    FoundationalTransitionSyntheticPressureEvidence,
    FoundationalTransitionSyntheticRuntimePressure,
};

#[cfg(test)]
mod tests {
    use forge_proof::AuthorityWitness;

    use super::FoundationalTransitionProductionReadinessAuthority;

    #[test]
    fn production_readiness_authority_is_crate_controlled() {
        let _authority = AuthorityWitness::from_authority_marker(
            FoundationalTransitionProductionReadinessAuthority::certification_boundary(),
        );
    }
}
