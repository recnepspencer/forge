use super::inventory::{
    certified_surface_evidence, certified_surfaces, compile_fail_boundaries,
    forge_proof_api_appendix, forge_proof_forbidden_surfaces, forge_proof_required_surfaces,
    phase_gates, residual_debt, runtime_assumptions, runtime_non_assumptions, synthetic_pressures,
};
use super::vocabulary::{
    FoundationalProfileCertifiedSurface, FoundationalProfileCertifiedSurfaceEvidence,
    FoundationalProfileCompileFailBoundary, FoundationalProfileForgeProofApi,
    FoundationalProfileForgeProofForbiddenSurface, FoundationalProfileForgeProofSurface,
    FoundationalProfileMilestone3PhaseGate, FoundationalProfilePhaseGateEvidence,
    FoundationalProfileResidualDebt, FoundationalProfileRuntimeAssumption,
    FoundationalProfileRuntimeNonAssumption, FoundationalProfileSyntheticRuntimePressure,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalProfileProductionReadinessReport {
    certified_surfaces: Vec<FoundationalProfileCertifiedSurface>,
    certified_surface_evidence: Vec<FoundationalProfileCertifiedSurfaceEvidence>,
    synthetic_pressures: Vec<FoundationalProfileSyntheticRuntimePressure>,
    compile_fail_boundaries: Vec<FoundationalProfileCompileFailBoundary>,
    forge_proof_required_surfaces: Vec<FoundationalProfileForgeProofSurface>,
    forge_proof_api_appendix: Vec<FoundationalProfileForgeProofApi>,
    forge_proof_forbidden_surfaces: Vec<FoundationalProfileForgeProofForbiddenSurface>,
    assumptions: Vec<FoundationalProfileRuntimeAssumption>,
    non_assumptions: Vec<FoundationalProfileRuntimeNonAssumption>,
    residual_debt: Vec<FoundationalProfileResidualDebt>,
    phase_gates: Vec<FoundationalProfilePhaseGateEvidence>,
}

impl FoundationalProfileProductionReadinessReport {
    pub(super) fn new() -> Self {
        Self {
            certified_surfaces: certified_surfaces(),
            certified_surface_evidence: certified_surface_evidence(),
            synthetic_pressures: synthetic_pressures(),
            compile_fail_boundaries: compile_fail_boundaries(),
            forge_proof_required_surfaces: forge_proof_required_surfaces(),
            forge_proof_api_appendix: forge_proof_api_appendix(),
            forge_proof_forbidden_surfaces: forge_proof_forbidden_surfaces(),
            assumptions: runtime_assumptions(),
            non_assumptions: runtime_non_assumptions(),
            residual_debt: residual_debt(),
            phase_gates: phase_gates(),
        }
    }

    pub fn certified_surfaces(&self) -> &[FoundationalProfileCertifiedSurface] {
        &self.certified_surfaces
    }

    pub fn certified_surface_evidence(&self) -> &[FoundationalProfileCertifiedSurfaceEvidence] {
        &self.certified_surface_evidence
    }

    pub fn synthetic_pressures(&self) -> &[FoundationalProfileSyntheticRuntimePressure] {
        &self.synthetic_pressures
    }

    pub fn compile_fail_boundaries(&self) -> &[FoundationalProfileCompileFailBoundary] {
        &self.compile_fail_boundaries
    }

    pub fn forge_proof_required_surfaces(&self) -> &[FoundationalProfileForgeProofSurface] {
        &self.forge_proof_required_surfaces
    }

    pub fn forge_proof_api_appendix(&self) -> &[FoundationalProfileForgeProofApi] {
        &self.forge_proof_api_appendix
    }

    pub fn forge_proof_forbidden_surfaces(
        &self,
    ) -> &[FoundationalProfileForgeProofForbiddenSurface] {
        &self.forge_proof_forbidden_surfaces
    }

    pub fn assumptions(&self) -> &[FoundationalProfileRuntimeAssumption] {
        &self.assumptions
    }

    pub fn non_assumptions(&self) -> &[FoundationalProfileRuntimeNonAssumption] {
        &self.non_assumptions
    }

    pub fn residual_debt(&self) -> &[FoundationalProfileResidualDebt] {
        &self.residual_debt
    }

    pub fn phase_gates(&self) -> &[FoundationalProfilePhaseGateEvidence] {
        &self.phase_gates
    }

    pub fn passes_readiness_checklist(&self) -> bool {
        self.has_all_certified_surfaces()
            && self.has_evidence_for_each_certified_surface()
            && self.has_all_synthetic_pressures()
            && self.has_all_compile_fail_boundaries()
            && self.has_all_required_forge_proof_surfaces()
            && self.has_named_forge_proof_api_appendix()
            && self.has_all_forbidden_forge_proof_surfaces()
            && self.has_runtime_assumption_boundary()
            && self.has_named_residual_debt()
            && self.has_linear_phase_gates()
    }

    fn has_all_certified_surfaces(&self) -> bool {
        [
            FoundationalProfileCertifiedSurface::ProfileFamilies,
            FoundationalProfileCertifiedSurface::ProfileComposition,
            FoundationalProfileCertifiedSurface::ProgressionAndAttachment,
            FoundationalProfileCertifiedSurface::CanonicalIdentityAndDifference,
            FoundationalProfileCertifiedSurface::MaterializationAndElision,
            FoundationalProfileCertifiedSurface::CertificationStrengthening,
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
            FoundationalProfileSyntheticRuntimePressure::FamilyAdjacencyHostility,
            FoundationalProfileSyntheticRuntimePressure::IndependentConstructionParity,
            FoundationalProfileSyntheticRuntimePressure::ReducedRichnessSuppression,
            FoundationalProfileSyntheticRuntimePressure::AttachmentTargetLaw,
            FoundationalProfileSyntheticRuntimePressure::ProofBearingCertificationBoundary,
        ]
        .iter()
        .all(|pressure| self.synthetic_pressures.contains(pressure))
    }

    fn has_all_compile_fail_boundaries(&self) -> bool {
        [
            FoundationalProfileCompileFailBoundary::RawLabelsCannotSatisfyProfileFamilyApis,
            FoundationalProfileCompileFailBoundary::PartialOrBagConstructionCannotSatisfyProfileSetApis,
            FoundationalProfileCompileFailBoundary::PlainPayloadCannotSatisfyAttachmentApis,
            FoundationalProfileCompileFailBoundary::RawDigestCannotSatisfyProfileIdentityApis,
            FoundationalProfileCompileFailBoundary::IllegalTargetSurfaceInventoriesCannotBeForged,
            FoundationalProfileCompileFailBoundary::WrongStrengthProofBearingCertificationCannotSatisfyStrongerApis,
            FoundationalProfileCompileFailBoundary::ProfileReadinessRequiresCertifiedArtifact,
        ]
        .iter()
        .all(|boundary| self.compile_fail_boundaries.contains(boundary))
    }

    fn has_all_required_forge_proof_surfaces(&self) -> bool {
        [
            FoundationalProfileForgeProofSurface::ArtifactCarrier,
            FoundationalProfileForgeProofSurface::TransitionOutcome,
            FoundationalProfileForgeProofSurface::AuthorityWitness,
            FoundationalProfileForgeProofSurface::BoundaryBridgeTrustBoundary,
            FoundationalProfileForgeProofSurface::BoundaryReadmitWithAuthority,
            FoundationalProfileForgeProofSurface::CurrentBasisArtifactConstructor,
        ]
        .iter()
        .all(|surface| self.forge_proof_required_surfaces.contains(surface))
    }

    fn has_named_forge_proof_api_appendix(&self) -> bool {
        [
            FoundationalProfileForgeProofApi::AuthorityWitnessFromAuthorityMarker,
            FoundationalProfileForgeProofApi::ArtifactNew,
            FoundationalProfileForgeProofApi::ArtifactWithCurrentBasis,
            FoundationalProfileForgeProofApi::ArtifactWithProofsAndCurrentBasis,
            FoundationalProfileForgeProofApi::TransitionOutcomeStructuredCategories,
            FoundationalProfileForgeProofApi::ArtifactBridgeTrustBoundary,
            FoundationalProfileForgeProofApi::ArtifactReadmitWithAuthority,
        ]
        .iter()
        .all(|api| self.forge_proof_api_appendix.contains(api))
    }

    fn has_all_forbidden_forge_proof_surfaces(&self) -> bool {
        [
            FoundationalProfileForgeProofForbiddenSurface::PlainProfileFamilyVocabulary,
            FoundationalProfileForgeProofForbiddenSurface::PlainProfileCompositionData,
            FoundationalProfileForgeProofForbiddenSurface::PlainDescriptiveSurfaceVocabulary,
            FoundationalProfileForgeProofForbiddenSurface::PlainProfileIdentityBasisEntries,
        ]
        .iter()
        .all(|surface| self.forge_proof_forbidden_surfaces.contains(surface))
    }

    fn has_runtime_assumption_boundary(&self) -> bool {
        self.assumptions.contains(
            &FoundationalProfileRuntimeAssumption::CanonicalBasisLawCertified,
        ) && self.assumptions.contains(
            &FoundationalProfileRuntimeAssumption::ProofBearingCertificationUsesExplicitAuthorityProgression,
        ) && self.non_assumptions.contains(
            &FoundationalProfileRuntimeNonAssumption::AdoptingCrateLoweringParityAlreadyProven,
        ) && self.non_assumptions.contains(
            &FoundationalProfileRuntimeNonAssumption::BoundaryCrossingPreservesStrongerCertificationWithoutReadmission,
        )
    }

    fn has_named_residual_debt(&self) -> bool {
        [
            FoundationalProfileResidualDebt::AdoptingCrateParityDeferred,
            FoundationalProfileResidualDebt::RealRuntimePolicyLoweringDeferred,
            FoundationalProfileResidualDebt::LaterArtifactDiagnosticsAndProvenanceOntologyDeferred,
        ]
        .iter()
        .all(|debt| self.residual_debt.contains(debt))
    }

    fn has_linear_phase_gates(&self) -> bool {
        self.phase_gates.iter().map(|evidence| evidence.gate()).eq([
            FoundationalProfileMilestone3PhaseGate::TypedFamilies,
            FoundationalProfileMilestone3PhaseGate::ComposedProfileSet,
            FoundationalProfileMilestone3PhaseGate::ProgressionAndAttachment,
            FoundationalProfileMilestone3PhaseGate::CanonicalIdentityAndDifference,
            FoundationalProfileMilestone3PhaseGate::MaterializationAndElision,
            FoundationalProfileMilestone3PhaseGate::CertificationStrengthening,
            FoundationalProfileMilestone3PhaseGate::ProductionReadiness,
        ])
    }
}
