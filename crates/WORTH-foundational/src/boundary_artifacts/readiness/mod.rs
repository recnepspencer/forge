mod authority;
mod certification;
mod inventory;
mod report;
mod vocabulary;

pub use authority::FoundationalBoundaryArtifactProductionReadinessAuthority;
pub use certification::{
    certify_foundational_boundary_artifact_milestone4_production_test_readiness,
    foundational_boundary_artifact_milestone4_readiness_report,
    require_foundational_boundary_artifact_milestone4_production_test_readiness,
    FoundationalBoundaryArtifactProductionTestReadyArtifact,
};
pub use report::FoundationalBoundaryArtifactProductionReadinessReport;
pub use vocabulary::{
    FoundationalBoundaryArtifactCertifiedSurface,
    FoundationalBoundaryArtifactCertifiedSurfaceEvidence,
    FoundationalBoundaryArtifactCompileFailBoundary, FoundationalBoundaryArtifactWORTHProofApi,
    FoundationalBoundaryArtifactWORTHProofForbiddenSurface,
    FoundationalBoundaryArtifactWORTHProofSurface, FoundationalBoundaryArtifactMilestone4PhaseGate,
    FoundationalBoundaryArtifactPhaseGateEvidence,
    FoundationalBoundaryArtifactProductionReadinessScope, FoundationalBoundaryArtifactResidualDebt,
    FoundationalBoundaryArtifactRuntimeAssumption,
    FoundationalBoundaryArtifactRuntimeNonAssumption,
    FoundationalBoundaryArtifactSyntheticRuntimePressure,
};

#[cfg(test)]
mod tests {
    use worth_proof::AuthorityWitness;

    use super::FoundationalBoundaryArtifactProductionReadinessAuthority;

    #[test]
    fn production_readiness_authority_is_crate_controlled() {
        let _authority = AuthorityWitness::from_authority_marker(
            FoundationalBoundaryArtifactProductionReadinessAuthority::certification_boundary(),
        );
    }
}
