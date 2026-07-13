use super::inventory::{
    certified_surface_evidence, certified_surfaces, compile_fail_boundaries, phase_gates,
    public_surface_compile_fail_path, public_surface_evidence_path, public_surface_inventory,
    residual_debt, runtime_assumptions, runtime_non_assumptions, synthetic_pressures,
    worth_proof_api_appendix, worth_proof_forbidden_surfaces, worth_proof_required_surfaces,
};
use super::vocabulary::{
    FoundationalProfileCertifiedSurface, FoundationalProfileCertifiedSurfaceEvidence,
    FoundationalProfileCompileFailBoundary, FoundationalProfileMilestone3PhaseGate,
    FoundationalProfilePhaseGateEvidence, FoundationalProfileResidualDebt,
    FoundationalProfileRuntimeAssumption, FoundationalProfileRuntimeNonAssumption,
    FoundationalProfileSyntheticRuntimePressure, FoundationalProfileWORTHProofApi,
    FoundationalProfileWORTHProofForbiddenSurface, FoundationalProfileWORTHProofSurface,
};
use crate::profiles_api::{FoundationalProfilePublicLane, FoundationalProfilePublicSurfaceEntry};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalProfileProductionReadinessReport {
    certified_surfaces: Vec<FoundationalProfileCertifiedSurface>,
    certified_surface_evidence: Vec<FoundationalProfileCertifiedSurfaceEvidence>,
    synthetic_pressures: Vec<FoundationalProfileSyntheticRuntimePressure>,
    compile_fail_boundaries: Vec<FoundationalProfileCompileFailBoundary>,
    worth_proof_required_surfaces: Vec<FoundationalProfileWORTHProofSurface>,
    worth_proof_api_appendix: Vec<FoundationalProfileWORTHProofApi>,
    worth_proof_forbidden_surfaces: Vec<FoundationalProfileWORTHProofForbiddenSurface>,
    assumptions: Vec<FoundationalProfileRuntimeAssumption>,
    non_assumptions: Vec<FoundationalProfileRuntimeNonAssumption>,
    residual_debt: Vec<FoundationalProfileResidualDebt>,
    phase_gates: Vec<FoundationalProfilePhaseGateEvidence>,
    public_surface_inventory: Vec<FoundationalProfilePublicSurfaceEntry>,
    public_surface_evidence_path: &'static str,
    public_surface_compile_fail_path: &'static str,
}

impl FoundationalProfileProductionReadinessReport {
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
            public_surface_inventory: public_surface_inventory(),
            public_surface_evidence_path: public_surface_evidence_path(),
            public_surface_compile_fail_path: public_surface_compile_fail_path(),
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

    pub fn worth_proof_required_surfaces(&self) -> &[FoundationalProfileWORTHProofSurface] {
        &self.worth_proof_required_surfaces
    }

    pub fn worth_proof_api_appendix(&self) -> &[FoundationalProfileWORTHProofApi] {
        &self.worth_proof_api_appendix
    }

    pub fn worth_proof_forbidden_surfaces(
        &self,
    ) -> &[FoundationalProfileWORTHProofForbiddenSurface] {
        &self.worth_proof_forbidden_surfaces
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

    pub fn public_surface_inventory(&self) -> &[FoundationalProfilePublicSurfaceEntry] {
        &self.public_surface_inventory
    }

    pub fn public_surface_evidence_path(&self) -> &'static str {
        self.public_surface_evidence_path
    }

    pub fn public_surface_compile_fail_path(&self) -> &'static str {
        self.public_surface_compile_fail_path
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
            && self.has_exact_public_surface_inventory()
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
            FoundationalProfileCompileFailBoundary::IllegalTargetSurfaceInventoriesCannotBeWorthd,
            FoundationalProfileCompileFailBoundary::WrongStrengthProofBearingCertificationCannotSatisfyStrongerApis,
            FoundationalProfileCompileFailBoundary::ProfileReadinessRequiresCertifiedArtifact,
        ]
        .iter()
        .all(|boundary| self.compile_fail_boundaries.contains(boundary))
    }

    fn has_all_required_worth_proof_surfaces(&self) -> bool {
        [
            FoundationalProfileWORTHProofSurface::ArtifactCarrier,
            FoundationalProfileWORTHProofSurface::TransitionOutcome,
            FoundationalProfileWORTHProofSurface::AuthorityWitness,
            FoundationalProfileWORTHProofSurface::BoundaryBridgeTrustBoundary,
            FoundationalProfileWORTHProofSurface::BoundaryReadmitWithAuthority,
            FoundationalProfileWORTHProofSurface::CurrentBasisArtifactConstructor,
        ]
        .iter()
        .all(|surface| self.worth_proof_required_surfaces.contains(surface))
    }

    fn has_named_worth_proof_api_appendix(&self) -> bool {
        [
            FoundationalProfileWORTHProofApi::AuthorityWitnessFromAuthorityMarker,
            FoundationalProfileWORTHProofApi::ArtifactNew,
            FoundationalProfileWORTHProofApi::ArtifactWithCurrentBasis,
            FoundationalProfileWORTHProofApi::ArtifactWithProofsAndCurrentBasis,
            FoundationalProfileWORTHProofApi::TransitionOutcomeStructuredCategories,
            FoundationalProfileWORTHProofApi::ArtifactBridgeTrustBoundary,
            FoundationalProfileWORTHProofApi::ArtifactReadmitWithAuthority,
        ]
        .iter()
        .all(|api| self.worth_proof_api_appendix.contains(api))
    }

    fn has_all_forbidden_worth_proof_surfaces(&self) -> bool {
        [
            FoundationalProfileWORTHProofForbiddenSurface::PlainProfileFamilyVocabulary,
            FoundationalProfileWORTHProofForbiddenSurface::PlainProfileCompositionData,
            FoundationalProfileWORTHProofForbiddenSurface::PlainDescriptiveSurfaceVocabulary,
            FoundationalProfileWORTHProofForbiddenSurface::PlainProfileIdentityBasisEntries,
        ]
        .iter()
        .all(|surface| self.worth_proof_forbidden_surfaces.contains(surface))
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

    fn has_exact_public_surface_inventory(&self) -> bool {
        let paths: BTreeSet<_> = self
            .public_surface_inventory
            .iter()
            .map(|entry| entry.path())
            .collect();
        let common_path_count = self
            .public_surface_inventory
            .iter()
            .filter(|entry| entry.lane() == FoundationalProfilePublicLane::CommonPath)
            .count();
        let stronger_lane_count = self
            .public_surface_inventory
            .iter()
            .filter(|entry| entry.lane() == FoundationalProfilePublicLane::StrongerLane)
            .count();

        paths
            == BTreeSet::from([
                "worth_foundational::profiles_api::common_path",
                "worth_foundational::profiles_api::lower_lane::composition",
                "worth_foundational::profiles_api::lower_lane::progression",
                "worth_foundational::profiles_api::lower_lane::attachment",
                "worth_foundational::profiles_api::lower_lane::materialization",
                "worth_foundational::profiles_api::lower_lane::identity",
                "worth_foundational::profiles_api::lower_lane::certification",
                "worth_foundational::profiles_api::stronger_lane",
                "worth_foundational::profiles_api::stronger_lane::readiness",
            ])
            && self.public_surface_inventory.len() == paths.len()
            && common_path_count == 1
            && stronger_lane_count == 2
            && self.public_surface_inventory.iter().all(|entry| {
                !entry.teaches().trim().is_empty() && !entry.does_not_hide().trim().is_empty()
            })
    }
}
