use super::inventory::{
    certified_surface_evidence, certified_surfaces, compile_fail_boundaries,
    worth_proof_api_appendix, worth_proof_forbidden_surfaces, worth_proof_required_surfaces,
    phase_gates, residual_debt, runtime_assumptions, runtime_non_assumptions, synthetic_pressures,
};
use super::vocabulary::{
    FoundationalBoundaryArtifactCertifiedSurface,
    FoundationalBoundaryArtifactCertifiedSurfaceEvidence,
    FoundationalBoundaryArtifactCompileFailBoundary, FoundationalBoundaryArtifactWORTHProofApi,
    FoundationalBoundaryArtifactWORTHProofForbiddenSurface,
    FoundationalBoundaryArtifactWORTHProofSurface, FoundationalBoundaryArtifactMilestone4PhaseGate,
    FoundationalBoundaryArtifactPhaseGateEvidence, FoundationalBoundaryArtifactResidualDebt,
    FoundationalBoundaryArtifactRuntimeAssumption,
    FoundationalBoundaryArtifactRuntimeNonAssumption,
    FoundationalBoundaryArtifactSyntheticRuntimePressure,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryArtifactProductionReadinessReport {
    certified_surfaces: Vec<FoundationalBoundaryArtifactCertifiedSurface>,
    certified_surface_evidence: Vec<FoundationalBoundaryArtifactCertifiedSurfaceEvidence>,
    synthetic_pressures: Vec<FoundationalBoundaryArtifactSyntheticRuntimePressure>,
    compile_fail_boundaries: Vec<FoundationalBoundaryArtifactCompileFailBoundary>,
    worth_proof_required_surfaces: Vec<FoundationalBoundaryArtifactWORTHProofSurface>,
    worth_proof_api_appendix: Vec<FoundationalBoundaryArtifactWORTHProofApi>,
    worth_proof_forbidden_surfaces: Vec<FoundationalBoundaryArtifactWORTHProofForbiddenSurface>,
    assumptions: Vec<FoundationalBoundaryArtifactRuntimeAssumption>,
    non_assumptions: Vec<FoundationalBoundaryArtifactRuntimeNonAssumption>,
    residual_debt: Vec<FoundationalBoundaryArtifactResidualDebt>,
    phase_gates: Vec<FoundationalBoundaryArtifactPhaseGateEvidence>,
}

impl FoundationalBoundaryArtifactProductionReadinessReport {
    pub(super) fn new() -> Self {
        Self {
            certified_surfaces: certified_surfaces(),
            certified_surface_evidence: certified_surface_evidence(),
            synthetic_pressures: synthetic_pressures(),
            compile_fail_boundaries: compile_fail_boundaries(),
            worth_proof_required_surfaces: worth_proof_required_surfaces(),
            worth_proof_api_appendix: worth_proof_api_appendix(),
            worth_proof_forbidden_surfaces: worth_proof_forbidden_surfaces(),
            assumptions: runtime_assumptions(),
            non_assumptions: runtime_non_assumptions(),
            residual_debt: residual_debt(),
            phase_gates: phase_gates(),
        }
    }

    pub fn certified_surfaces(&self) -> &[FoundationalBoundaryArtifactCertifiedSurface] {
        &self.certified_surfaces
    }

    pub fn certified_surface_evidence(
        &self,
    ) -> &[FoundationalBoundaryArtifactCertifiedSurfaceEvidence] {
        &self.certified_surface_evidence
    }

    pub fn synthetic_pressures(&self) -> &[FoundationalBoundaryArtifactSyntheticRuntimePressure] {
        &self.synthetic_pressures
    }

    pub fn compile_fail_boundaries(&self) -> &[FoundationalBoundaryArtifactCompileFailBoundary] {
        &self.compile_fail_boundaries
    }

    pub fn worth_proof_required_surfaces(
        &self,
    ) -> &[FoundationalBoundaryArtifactWORTHProofSurface] {
        &self.worth_proof_required_surfaces
    }

    pub fn worth_proof_api_appendix(&self) -> &[FoundationalBoundaryArtifactWORTHProofApi] {
        &self.worth_proof_api_appendix
    }

    pub fn worth_proof_forbidden_surfaces(
        &self,
    ) -> &[FoundationalBoundaryArtifactWORTHProofForbiddenSurface] {
        &self.worth_proof_forbidden_surfaces
    }

    pub fn assumptions(&self) -> &[FoundationalBoundaryArtifactRuntimeAssumption] {
        &self.assumptions
    }

    pub fn non_assumptions(&self) -> &[FoundationalBoundaryArtifactRuntimeNonAssumption] {
        &self.non_assumptions
    }

    pub fn residual_debt(&self) -> &[FoundationalBoundaryArtifactResidualDebt] {
        &self.residual_debt
    }

    pub fn phase_gates(&self) -> &[FoundationalBoundaryArtifactPhaseGateEvidence] {
        &self.phase_gates
    }

    pub fn passes_readiness_checklist(&self) -> bool {
        self.has_all_certified_surfaces()
            && self.has_evidence_for_each_certified_surface()
            && self.has_all_synthetic_pressures()
            && self.has_all_compile_fail_boundaries()
            && self.has_all_required_worth_proof_surfaces()
            && self.has_named_worth_proof_api_appendix()
            && self.has_all_forbidden_worth_proof_surfaces()
            && self.has_runtime_assumption_boundary()
            && self.has_named_residual_debt()
            && self.has_linear_phase_gates()
    }

    fn has_all_certified_surfaces(&self) -> bool {
        [
            FoundationalBoundaryArtifactCertifiedSurface::CategoryVocabulary,
            FoundationalBoundaryArtifactCertifiedSurface::RoleAndAuthorityLaw,
            FoundationalBoundaryArtifactCertifiedSurface::MaterializationAndBundles,
            FoundationalBoundaryArtifactCertifiedSurface::CanonicalBasisParticipation,
            FoundationalBoundaryArtifactCertifiedSurface::CurrentBasisProofLane,
            FoundationalBoundaryArtifactCertifiedSurface::DescriptiveExtensionLaw,
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
            FoundationalBoundaryArtifactSyntheticRuntimePressure::CategoryAdjacencyHostility,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::AuthorityDerivationSeparation,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::MaterializationSeamHonesty,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::CanonicalBasisParity,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::CurrentBasisReadmissionBoundary,
            FoundationalBoundaryArtifactSyntheticRuntimePressure::ReservedAuthorityTransitionFailClosedBoundary,
        ]
        .iter()
        .all(|pressure| self.synthetic_pressures.contains(pressure))
    }

    fn has_all_compile_fail_boundaries(&self) -> bool {
        [
            FoundationalBoundaryArtifactCompileFailBoundary::CategoryWrapperCollapseRejected,
            FoundationalBoundaryArtifactCompileFailBoundary::IllegalRoleAndAuthorityClaimsRejected,
            FoundationalBoundaryArtifactCompileFailBoundary::PlainPayloadCannotBypassMaterializationContracts,
            FoundationalBoundaryArtifactCompileFailBoundary::RawMaterializedOutputsCannotSatisfyCanonicalBasisApis,
            FoundationalBoundaryArtifactCompileFailBoundary::RawMaterializedOutputsCannotSatisfyCurrentBasisApis,
            FoundationalBoundaryArtifactCompileFailBoundary::DescriptiveExtensionsCannotSatisfyAuthorityOrReservedAuthorityApis,
            FoundationalBoundaryArtifactCompileFailBoundary::BoundaryArtifactReadinessRequiresCertifiedArtifact,
        ]
        .iter()
        .all(|boundary| self.compile_fail_boundaries.contains(boundary))
    }

    fn has_all_required_worth_proof_surfaces(&self) -> bool {
        [
            FoundationalBoundaryArtifactWORTHProofSurface::AuthorityWitness,
            FoundationalBoundaryArtifactWORTHProofSurface::AuthorityAdmissionProofBearingClaim,
            FoundationalBoundaryArtifactWORTHProofSurface::TransitionOutcome,
            FoundationalBoundaryArtifactWORTHProofSurface::CurrentBasisArtifactConstructor,
            FoundationalBoundaryArtifactWORTHProofSurface::BoundaryBridgeTrustBoundary,
            FoundationalBoundaryArtifactWORTHProofSurface::BoundaryReadmitWithAuthority,
            FoundationalBoundaryArtifactWORTHProofSurface::ProductionReadinessCertificationArtifact,
        ]
        .iter()
        .all(|surface| self.worth_proof_required_surfaces.contains(surface))
    }

    fn has_named_worth_proof_api_appendix(&self) -> bool {
        [
            FoundationalBoundaryArtifactWORTHProofApi::AuthorityWitnessFromAuthorityMarker,
            FoundationalBoundaryArtifactWORTHProofApi::ProofFromAuthorityWitness,
            FoundationalBoundaryArtifactWORTHProofApi::ArtifactWithCurrentBasisProofs,
            FoundationalBoundaryArtifactWORTHProofApi::ArtifactWithProofsAndCurrentBasis,
            FoundationalBoundaryArtifactWORTHProofApi::TransitionOutcomeStructuredCategories,
            FoundationalBoundaryArtifactWORTHProofApi::ArtifactBridgeTrustBoundary,
            FoundationalBoundaryArtifactWORTHProofApi::ArtifactReadmitWithAuthority,
        ]
        .iter()
        .all(|api| self.worth_proof_api_appendix.contains(api))
    }

    fn has_all_forbidden_worth_proof_surfaces(&self) -> bool {
        [
            FoundationalBoundaryArtifactWORTHProofForbiddenSurface::PlainCategoryVocabulary,
            FoundationalBoundaryArtifactWORTHProofForbiddenSurface::PlainRoleAndMaterializationVocabulary,
            FoundationalBoundaryArtifactWORTHProofForbiddenSurface::PlainBundleMembershipData,
            FoundationalBoundaryArtifactWORTHProofForbiddenSurface::PlainSameFamilyDescriptiveNouns,
        ]
        .iter()
        .all(|surface| self.worth_proof_forbidden_surfaces.contains(surface))
    }

    fn has_runtime_assumption_boundary(&self) -> bool {
        self.assumptions.contains(
            &FoundationalBoundaryArtifactRuntimeAssumption::Milestone2CanonicalizationRemainsAuthorityForBasisReadiness,
        ) && self.assumptions.contains(
            &FoundationalBoundaryArtifactRuntimeAssumption::Milestone3ProfilesGovernAttachmentAndElision,
        ) && self.non_assumptions.contains(
            &FoundationalBoundaryArtifactRuntimeNonAssumption::AdoptingCrateParityAlreadyProven,
        ) && self.non_assumptions.contains(
            &FoundationalBoundaryArtifactRuntimeNonAssumption::ReservedAuthorityTransitionOntologyAlreadyOwnedHere,
        )
    }

    fn has_named_residual_debt(&self) -> bool {
        [
            FoundationalBoundaryArtifactResidualDebt::AdoptingCrateParityDeferred,
            FoundationalBoundaryArtifactResidualDebt::ReservedAuthorityTransitionOntologyDeferred,
            FoundationalBoundaryArtifactResidualDebt::LaterDiagnosticsProvenanceAndReceiptSemanticsDeferred,
        ]
        .iter()
        .all(|debt| self.residual_debt.contains(debt))
    }

    fn has_linear_phase_gates(&self) -> bool {
        self.phase_gates.iter().map(|evidence| evidence.gate()).eq([
            FoundationalBoundaryArtifactMilestone4PhaseGate::Categories,
            FoundationalBoundaryArtifactMilestone4PhaseGate::RoleAndAuthority,
            FoundationalBoundaryArtifactMilestone4PhaseGate::MaterializationAndBundles,
            FoundationalBoundaryArtifactMilestone4PhaseGate::CanonicalBasisParticipation,
            FoundationalBoundaryArtifactMilestone4PhaseGate::CurrentBasisProofLane,
            FoundationalBoundaryArtifactMilestone4PhaseGate::DescriptiveExtensions,
            FoundationalBoundaryArtifactMilestone4PhaseGate::ProductionReadiness,
        ])
    }
}
