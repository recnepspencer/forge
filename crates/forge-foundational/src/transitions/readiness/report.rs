use super::inventory::{
    certified_surface_evidence, certified_surfaces, compile_fail_boundaries, compile_fail_evidence,
    forge_proof_api_appendix, forge_proof_api_evidence, forge_proof_forbidden_surfaces,
    forge_proof_required_surfaces, phase_gates, residual_debt, runtime_assumptions,
    runtime_non_assumptions, synthetic_pressure_evidence, synthetic_pressures,
};
use super::vocabulary::{
    FoundationalTransitionCertifiedSurface, FoundationalTransitionCertifiedSurfaceEvidence,
    FoundationalTransitionCompileFailBoundary, FoundationalTransitionCompileFailEvidence,
    FoundationalTransitionForgeProofApi, FoundationalTransitionForgeProofApiEvidence,
    FoundationalTransitionForgeProofForbiddenSurface, FoundationalTransitionForgeProofSurface,
    FoundationalTransitionMilestone5PhaseGate, FoundationalTransitionPhaseGateEvidence,
    FoundationalTransitionResidualDebt, FoundationalTransitionRuntimeAssumption,
    FoundationalTransitionRuntimeNonAssumption, FoundationalTransitionSyntheticPressureEvidence,
    FoundationalTransitionSyntheticRuntimePressure,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalTransitionProductionReadinessReport {
    certified_surfaces: Vec<FoundationalTransitionCertifiedSurface>,
    certified_surface_evidence: Vec<FoundationalTransitionCertifiedSurfaceEvidence>,
    synthetic_pressures: Vec<FoundationalTransitionSyntheticRuntimePressure>,
    synthetic_pressure_evidence: Vec<FoundationalTransitionSyntheticPressureEvidence>,
    compile_fail_boundaries: Vec<FoundationalTransitionCompileFailBoundary>,
    compile_fail_evidence: Vec<FoundationalTransitionCompileFailEvidence>,
    forge_proof_required_surfaces: Vec<FoundationalTransitionForgeProofSurface>,
    forge_proof_api_appendix: Vec<FoundationalTransitionForgeProofApi>,
    forge_proof_api_evidence: Vec<FoundationalTransitionForgeProofApiEvidence>,
    forge_proof_forbidden_surfaces: Vec<FoundationalTransitionForgeProofForbiddenSurface>,
    assumptions: Vec<FoundationalTransitionRuntimeAssumption>,
    non_assumptions: Vec<FoundationalTransitionRuntimeNonAssumption>,
    residual_debt: Vec<FoundationalTransitionResidualDebt>,
    phase_gates: Vec<FoundationalTransitionPhaseGateEvidence>,
}

impl FoundationalTransitionProductionReadinessReport {
    pub(super) fn new() -> Self {
        Self {
            certified_surfaces: certified_surfaces(),
            certified_surface_evidence: certified_surface_evidence(),
            synthetic_pressures: synthetic_pressures(),
            synthetic_pressure_evidence: synthetic_pressure_evidence(),
            compile_fail_boundaries: compile_fail_boundaries(),
            compile_fail_evidence: compile_fail_evidence(),
            forge_proof_required_surfaces: forge_proof_required_surfaces(),
            forge_proof_api_appendix: forge_proof_api_appendix(),
            forge_proof_api_evidence: forge_proof_api_evidence(),
            forge_proof_forbidden_surfaces: forge_proof_forbidden_surfaces(),
            assumptions: runtime_assumptions(),
            non_assumptions: runtime_non_assumptions(),
            residual_debt: residual_debt(),
            phase_gates: phase_gates(),
        }
    }

    pub fn certified_surfaces(&self) -> &[FoundationalTransitionCertifiedSurface] {
        &self.certified_surfaces
    }

    pub fn certified_surface_evidence(&self) -> &[FoundationalTransitionCertifiedSurfaceEvidence] {
        &self.certified_surface_evidence
    }

    pub fn synthetic_pressures(&self) -> &[FoundationalTransitionSyntheticRuntimePressure] {
        &self.synthetic_pressures
    }

    pub fn synthetic_pressure_evidence(
        &self,
    ) -> &[FoundationalTransitionSyntheticPressureEvidence] {
        &self.synthetic_pressure_evidence
    }

    pub fn compile_fail_boundaries(&self) -> &[FoundationalTransitionCompileFailBoundary] {
        &self.compile_fail_boundaries
    }

    pub fn compile_fail_evidence(&self) -> &[FoundationalTransitionCompileFailEvidence] {
        &self.compile_fail_evidence
    }

    pub fn forge_proof_required_surfaces(&self) -> &[FoundationalTransitionForgeProofSurface] {
        &self.forge_proof_required_surfaces
    }

    pub fn forge_proof_api_appendix(&self) -> &[FoundationalTransitionForgeProofApi] {
        &self.forge_proof_api_appendix
    }

    pub fn forge_proof_api_evidence(&self) -> &[FoundationalTransitionForgeProofApiEvidence] {
        &self.forge_proof_api_evidence
    }

    pub fn forge_proof_forbidden_surfaces(
        &self,
    ) -> &[FoundationalTransitionForgeProofForbiddenSurface] {
        &self.forge_proof_forbidden_surfaces
    }

    pub fn assumptions(&self) -> &[FoundationalTransitionRuntimeAssumption] {
        &self.assumptions
    }

    pub fn non_assumptions(&self) -> &[FoundationalTransitionRuntimeNonAssumption] {
        &self.non_assumptions
    }

    pub fn residual_debt(&self) -> &[FoundationalTransitionResidualDebt] {
        &self.residual_debt
    }

    pub fn phase_gates(&self) -> &[FoundationalTransitionPhaseGateEvidence] {
        &self.phase_gates
    }

    pub fn passes_readiness_checklist(&self) -> bool {
        self.has_all_certified_surfaces()
            && self.has_evidence_for_each_certified_surface()
            && self.has_all_synthetic_pressures()
            && self.has_evidence_for_each_synthetic_pressure()
            && self.has_all_compile_fail_boundaries()
            && self.has_evidence_for_each_compile_fail_boundary()
            && self.has_all_required_forge_proof_surfaces()
            && self.has_named_forge_proof_api_appendix()
            && self.has_evidence_for_each_forge_proof_api()
            && self.has_all_forbidden_forge_proof_surfaces()
            && self.has_runtime_assumption_boundary()
            && self.has_named_residual_debt()
            && self.has_linear_phase_gates()
    }

    fn has_all_certified_surfaces(&self) -> bool {
        [
            FoundationalTransitionCertifiedSurface::BranchLocalSeparation,
            FoundationalTransitionCertifiedSurface::MergeVerdictLaw,
            FoundationalTransitionCertifiedSurface::CommittedAuthorityTransitions,
            FoundationalTransitionCertifiedSurface::CommitReceiptsAndBundles,
            FoundationalTransitionCertifiedSurface::CanonicalBasisAndLocatorIntegration,
            FoundationalTransitionCertifiedSurface::ProfileRichnessAndCurrentBasisBehavior,
        ]
        .iter()
        .all(|surface| self.certified_surfaces.contains(surface))
    }

    fn has_evidence_for_each_certified_surface(&self) -> bool {
        self.certified_surfaces.iter().all(|surface| {
            self.certified_surface_evidence
                .iter()
                .filter(|evidence| evidence.surface() == *surface)
                .count()
                == 1
        })
    }

    fn has_all_synthetic_pressures(&self) -> bool {
        [
            FoundationalTransitionSyntheticRuntimePressure::AuthoritySeparation,
            FoundationalTransitionSyntheticRuntimePressure::MergeTopologyHonesty,
            FoundationalTransitionSyntheticRuntimePressure::NoOpVersusCommitClassification,
            FoundationalTransitionSyntheticRuntimePressure::ReceiptIssuanceBoundary,
            FoundationalTransitionSyntheticRuntimePressure::ReplayInterpretationBoundary,
            FoundationalTransitionSyntheticRuntimePressure::ReducedRichnessPreservation,
            FoundationalTransitionSyntheticRuntimePressure::AmbientBasisChoiceHostility,
            FoundationalTransitionSyntheticRuntimePressure::HiddenStrategyInfluenceHostility,
            FoundationalTransitionSyntheticRuntimePressure::ThinReceiptRejection,
            FoundationalTransitionSyntheticRuntimePressure::GenericTransitionResultBagRejection,
            FoundationalTransitionSyntheticRuntimePressure::CheapConvenienceBypassRejection,
        ]
        .iter()
        .all(|pressure| self.synthetic_pressures.contains(pressure))
    }

    fn has_evidence_for_each_synthetic_pressure(&self) -> bool {
        self.synthetic_pressures.iter().all(|pressure| {
            self.synthetic_pressure_evidence
                .iter()
                .filter(|evidence| evidence.pressure() == *pressure)
                .count()
                == 1
        })
    }

    fn has_all_compile_fail_boundaries(&self) -> bool {
        [
            FoundationalTransitionCompileFailBoundary::BranchLocalSurfacesCannotSatisfyAuthorityApis,
            FoundationalTransitionCompileFailBoundary::MergeAdmissionSurfacesRemainNonAuthoritative,
            FoundationalTransitionCompileFailBoundary::CommittedAuthorityRequiresProofBearingAdmission,
            FoundationalTransitionCompileFailBoundary::ReceiptAndCloseoutPreserveAuthoritySeparation,
            FoundationalTransitionCompileFailBoundary::Phase5BasisAndCurrentBasisRequireStrengthenedArtifacts,
            FoundationalTransitionCompileFailBoundary::TransitionReadinessRequiresCertifiedArtifact,
            FoundationalTransitionCompileFailBoundary::TransitionReadinessAuthorityCannotBeMinted,
        ]
        .iter()
        .all(|boundary| self.compile_fail_boundaries.contains(boundary))
    }

    fn has_evidence_for_each_compile_fail_boundary(&self) -> bool {
        self.compile_fail_boundaries.iter().all(|boundary| {
            self.compile_fail_evidence
                .iter()
                .filter(|evidence| evidence.boundary() == *boundary)
                .count()
                == 1
        })
    }

    fn has_all_required_forge_proof_surfaces(&self) -> bool {
        [
            FoundationalTransitionForgeProofSurface::TransitionOutcomeAdmissionLane,
            FoundationalTransitionForgeProofSurface::AuthorityWitnessScopedAdmission,
            FoundationalTransitionForgeProofSurface::ProofBearingCommittedAuthorityArtifact,
            FoundationalTransitionForgeProofSurface::ProofBearingCommitReceiptArtifact,
            FoundationalTransitionForgeProofSurface::CurrentBasisArtifactConstructor,
            FoundationalTransitionForgeProofSurface::BoundaryBridgeTrustBoundary,
            FoundationalTransitionForgeProofSurface::BoundaryReadmitWithAuthority,
            FoundationalTransitionForgeProofSurface::ProductionReadinessCertificationArtifact,
        ]
        .iter()
        .all(|surface| self.forge_proof_required_surfaces.contains(surface))
    }

    fn has_named_forge_proof_api_appendix(&self) -> bool {
        [
            FoundationalTransitionForgeProofApi::TransitionOutcomeStructuredCategories,
            FoundationalTransitionForgeProofApi::AuthorityWitnessFromAuthorityMarker,
            FoundationalTransitionForgeProofApi::ProofFromAuthorityWitness,
            FoundationalTransitionForgeProofApi::ArtifactWithProofsAndCurrentBasis,
            FoundationalTransitionForgeProofApi::ArtifactWithCurrentBasis,
            FoundationalTransitionForgeProofApi::ArtifactBridgeTrustBoundary,
            FoundationalTransitionForgeProofApi::ArtifactReadmitWithAuthority,
        ]
        .iter()
        .all(|api| self.forge_proof_api_appendix.contains(api))
    }

    fn has_evidence_for_each_forge_proof_api(&self) -> bool {
        self.forge_proof_api_appendix.iter().all(|api| {
            self.forge_proof_api_evidence
                .iter()
                .filter(|evidence| evidence.api() == *api)
                .count()
                == 1
        })
    }

    fn has_all_forbidden_forge_proof_surfaces(&self) -> bool {
        [
            FoundationalTransitionForgeProofForbiddenSurface::PlainBranchLocalVocabulary,
            FoundationalTransitionForgeProofForbiddenSurface::PlainMergeVerdictVocabulary,
            FoundationalTransitionForgeProofForbiddenSurface::PlainReceiptAndBundleVocabulary,
            FoundationalTransitionForgeProofForbiddenSurface::PlainCanonicalBasisAndLocatorVocabulary,
        ]
        .iter()
        .all(|surface| self.forge_proof_forbidden_surfaces.contains(surface))
    }

    fn has_runtime_assumption_boundary(&self) -> bool {
        self.assumptions.contains(
            &FoundationalTransitionRuntimeAssumption::Milestone2CanonicalizationRemainsAuthorityForTransitionBasisReadiness,
        ) && self.assumptions.contains(
            &FoundationalTransitionRuntimeAssumption::StrongerCommittedAuthorityAndReceiptClaimsUseForgeProof,
        ) && self.non_assumptions.contains(
            &FoundationalTransitionRuntimeNonAssumption::AdoptingRuntimeMergeStrategyParityAlreadyProven,
        ) && self.non_assumptions.contains(
            &FoundationalTransitionRuntimeNonAssumption::BoundaryCrossingPreservesCurrentBasisWithoutReadmission,
        )
    }

    fn has_named_residual_debt(&self) -> bool {
        [
            FoundationalTransitionResidualDebt::AdoptingRuntimeParityDeferred,
            FoundationalTransitionResidualDebt::LaterDiagnosticsAndProvenanceOntologyDeferred,
            FoundationalTransitionResidualDebt::RuntimeStrategyRegistryAndExecutionDeferred,
            FoundationalTransitionResidualDebt::FullLineageSupportBeyondTransitionRowsDeferred,
        ]
        .iter()
        .all(|debt| self.residual_debt.contains(debt))
    }

    fn has_linear_phase_gates(&self) -> bool {
        self.phase_gates.iter().map(|evidence| evidence.gate()).eq([
            FoundationalTransitionMilestone5PhaseGate::BranchLocalSeparation,
            FoundationalTransitionMilestone5PhaseGate::MergeVerdictLaw,
            FoundationalTransitionMilestone5PhaseGate::CommittedAuthorityTransitionLaw,
            FoundationalTransitionMilestone5PhaseGate::CommitReceiptsAndBundles,
            FoundationalTransitionMilestone5PhaseGate::CanonicalBasisLocatorAndProfileIntegration,
            FoundationalTransitionMilestone5PhaseGate::ProductionReadiness,
        ])
    }
}
