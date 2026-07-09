#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileProductionReadinessScope {
    milestone: &'static str,
}

impl FoundationalProfileProductionReadinessScope {
    pub(super) const fn milestone_3() -> Self {
        Self {
            milestone: "worth-foundational.milestone-3",
        }
    }

    pub const fn milestone(&self) -> &'static str {
        self.milestone
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileCertifiedSurface {
    ProfileFamilies,
    ProfileComposition,
    ProgressionAndAttachment,
    CanonicalIdentityAndDifference,
    MaterializationAndElision,
    CertificationStrengthening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileCertifiedSurfaceEvidence {
    surface: FoundationalProfileCertifiedSurface,
    hostile_pressure: FoundationalProfileSyntheticRuntimePressure,
    compile_fail_boundary: FoundationalProfileCompileFailBoundary,
    owning_test_path: &'static str,
    compile_fail_evidence_path: &'static str,
}

impl FoundationalProfileCertifiedSurfaceEvidence {
    pub(super) const fn new(
        surface: FoundationalProfileCertifiedSurface,
        hostile_pressure: FoundationalProfileSyntheticRuntimePressure,
        compile_fail_boundary: FoundationalProfileCompileFailBoundary,
        owning_test_path: &'static str,
        compile_fail_evidence_path: &'static str,
    ) -> Self {
        Self {
            surface,
            hostile_pressure,
            compile_fail_boundary,
            owning_test_path,
            compile_fail_evidence_path,
        }
    }

    pub const fn surface(&self) -> FoundationalProfileCertifiedSurface {
        self.surface
    }

    pub const fn hostile_pressure(&self) -> FoundationalProfileSyntheticRuntimePressure {
        self.hostile_pressure
    }

    pub const fn compile_fail_boundary(&self) -> FoundationalProfileCompileFailBoundary {
        self.compile_fail_boundary
    }

    pub const fn owning_test_path(&self) -> &'static str {
        self.owning_test_path
    }

    pub const fn compile_fail_evidence_path(&self) -> &'static str {
        self.compile_fail_evidence_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileSyntheticRuntimePressure {
    FamilyAdjacencyHostility,
    IndependentConstructionParity,
    ReducedRichnessSuppression,
    AttachmentTargetLaw,
    ProofBearingCertificationBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileCompileFailBoundary {
    RawLabelsCannotSatisfyProfileFamilyApis,
    PartialOrBagConstructionCannotSatisfyProfileSetApis,
    PlainPayloadCannotSatisfyAttachmentApis,
    RawDigestCannotSatisfyProfileIdentityApis,
    IllegalTargetSurfaceInventoriesCannotBeWorthd,
    WrongStrengthProofBearingCertificationCannotSatisfyStrongerApis,
    ProfileReadinessRequiresCertifiedArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileWORTHProofSurface {
    ArtifactCarrier,
    TransitionOutcome,
    AuthorityWitness,
    BoundaryBridgeTrustBoundary,
    BoundaryReadmitWithAuthority,
    CurrentBasisArtifactConstructor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileWORTHProofApi {
    AuthorityWitnessFromAuthorityMarker,
    ArtifactNew,
    ArtifactWithCurrentBasis,
    ArtifactWithProofsAndCurrentBasis,
    TransitionOutcomeStructuredCategories,
    ArtifactBridgeTrustBoundary,
    ArtifactReadmitWithAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileWORTHProofForbiddenSurface {
    PlainProfileFamilyVocabulary,
    PlainProfileCompositionData,
    PlainDescriptiveSurfaceVocabulary,
    PlainProfileIdentityBasisEntries,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileRuntimeAssumption {
    CanonicalBasisLawCertified,
    ProfileMeaningRemainsFacadeControlled,
    ReducedRichnessAffectsOnlyOptionalDescriptiveSurfaces,
    ProofBearingCertificationUsesExplicitAuthorityProgression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileRuntimeNonAssumption {
    RuntimePolicyExecutionExistsInFoundational,
    AdoptingCrateLoweringParityAlreadyProven,
    DiagnosticsOrProvenanceOntologyAlreadyOwnedHere,
    BoundaryCrossingPreservesStrongerCertificationWithoutReadmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileResidualDebt {
    AdoptingCrateParityDeferred,
    RealRuntimePolicyLoweringDeferred,
    LaterArtifactDiagnosticsAndProvenanceOntologyDeferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileMilestone3PhaseGate {
    TypedFamilies,
    ComposedProfileSet,
    ProgressionAndAttachment,
    CanonicalIdentityAndDifference,
    MaterializationAndElision,
    CertificationStrengthening,
    ProductionReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfilePhaseGateEvidence {
    gate: FoundationalProfileMilestone3PhaseGate,
    evidence_path: &'static str,
}

impl FoundationalProfilePhaseGateEvidence {
    pub(super) const fn new(
        gate: FoundationalProfileMilestone3PhaseGate,
        evidence_path: &'static str,
    ) -> Self {
        Self {
            gate,
            evidence_path,
        }
    }

    pub const fn gate(&self) -> FoundationalProfileMilestone3PhaseGate {
        self.gate
    }

    pub const fn evidence_path(&self) -> &'static str {
        self.evidence_path
    }
}
