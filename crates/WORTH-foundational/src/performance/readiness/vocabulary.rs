#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceProductionReadinessScope {
    milestone: &'static str,
}

impl FoundationalPerformanceProductionReadinessScope {
    pub(super) const fn milestone_8() -> Self {
        Self {
            milestone: "worth-foundational.milestone-8",
        }
    }

    pub const fn milestone(&self) -> &'static str {
        self.milestone
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceCertifiedSurface {
    PrimitiveAndCategoryLaw,
    ClaimBoundaryAndEvidenceStrengthLaw,
    LayoutIntentAndRepresentationFreedom,
    PolicyAdmissionAndBudgetLaw,
    CanonicalBundleAndCounterReceiptLaw,
    ReportAttachmentAndMaterializationLaw,
    CertifiedBundleAndReadmissionLaw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceCertifiedSurfaceEvidence {
    surface: FoundationalPerformanceCertifiedSurface,
    hostile_pressure: FoundationalPerformanceSyntheticRuntimePressure,
    compile_fail_boundary: FoundationalPerformanceCompileFailBoundary,
    owning_test_path: &'static str,
    compile_fail_evidence_path: &'static str,
}

impl FoundationalPerformanceCertifiedSurfaceEvidence {
    pub(super) const fn new(
        surface: FoundationalPerformanceCertifiedSurface,
        hostile_pressure: FoundationalPerformanceSyntheticRuntimePressure,
        compile_fail_boundary: FoundationalPerformanceCompileFailBoundary,
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

    pub const fn surface(&self) -> FoundationalPerformanceCertifiedSurface {
        self.surface
    }

    pub const fn hostile_pressure(&self) -> FoundationalPerformanceSyntheticRuntimePressure {
        self.hostile_pressure
    }

    pub const fn compile_fail_boundary(&self) -> FoundationalPerformanceCompileFailBoundary {
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
pub enum FoundationalPerformanceSyntheticRuntimePressure {
    PrimitiveFamilyNonSubstitution,
    ClaimStrengthAndLaneCollapseRejection,
    RepresentationEquivalenceOverclaimRejection,
    PreExecutionMasqueradeRejection,
    CanonicalCounterLoweringRejection,
    HiddenSupportExpansionRejection,
    CertifiedProofLaneBoundary,
    GroupedStrongerLaneBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceCompileFailBoundary {
    PrimitiveFamiliesAndCommonPathBoundaries,
    ClaimLaneBoundaries,
    LayoutAttachmentBoundaries,
    PolicyPreExecutionBoundaries,
    BundleAndCounterReceiptLoweringBoundaries,
    ReportMaterializationBoundaries,
    CertifiedBundleAndReadmissionProofLane,
    PerformanceReadinessRequiresCertifiedArtifact,
    PerformanceReadinessAuthorityCannotBeMinted,
    GroupedStrongerLaneRequiresCertifiedReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceWORTHProofSurface {
    ProductionReadinessCertificationArtifact,
    AuthorityWitness,
    ProofFromAuthorityWitness,
    ArtifactWithProofsAndCurrentBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceWORTHProofApi {
    AuthorityWitnessFromAuthorityMarker,
    ProofFromAuthorityWitness,
    ArtifactWithProofsAndCurrentBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceWORTHProofForbiddenSurface {
    PlainPerformanceVocabulary,
    PlainPerformanceLowerLaneArtifacts,
    PlainPerformanceReportPlanningVocabulary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceRuntimeAssumption {
    WORTHProofAuthorityLaneRemainsAvailable,
    ProfileLawRemainsAuthorityForReportElision,
    PhaseEvidencePathsRemainOwnedWithinFoundational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceRuntimeNonAssumption {
    WorkspaceWideTelemetryEngineIsOwnedHere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceResidualDebt {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceMilestone8PhaseGate {
    PrimitiveAndCategoryLaw,
    ClaimBoundaryAndEvidenceStrengthLaw,
    LayoutIntentAccessAndAllocationLaw,
    RuntimePolicyBudgetAndFallbackLaw,
    CanonicalBasisCounterAndComparisonLaw,
    AttachmentMaterializationAndBundleLaw,
    ProductionReadiness,
    FeatureDocsCrateDocIntegrationAndPublicationClosure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformancePhaseGateEvidence {
    gate: FoundationalPerformanceMilestone8PhaseGate,
    evidence_path: &'static str,
}

impl FoundationalPerformancePhaseGateEvidence {
    pub(super) const fn new(
        gate: FoundationalPerformanceMilestone8PhaseGate,
        evidence_path: &'static str,
    ) -> Self {
        Self {
            gate,
            evidence_path,
        }
    }

    pub const fn gate(&self) -> FoundationalPerformanceMilestone8PhaseGate {
        self.gate
    }

    pub const fn evidence_path(&self) -> &'static str {
        self.evidence_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformancePublicSurfaceDocumentationCoverage {
    public_surface_path: &'static str,
    primary_documentation_path: &'static str,
}

impl FoundationalPerformancePublicSurfaceDocumentationCoverage {
    pub(super) const fn new(
        public_surface_path: &'static str,
        primary_documentation_path: &'static str,
    ) -> Self {
        Self {
            public_surface_path,
            primary_documentation_path,
        }
    }

    pub const fn public_surface_path(&self) -> &'static str {
        self.public_surface_path
    }

    pub const fn primary_documentation_path(&self) -> &'static str {
        self.primary_documentation_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceRuntimeAdoptionPressure {
    CrossCrateMeaningParityMatrix,
    CertifiedBundleSourceCompatibilityMatrix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceRuntimeAdoptionPressureEvidence {
    pressure: FoundationalPerformanceRuntimeAdoptionPressure,
    evidence_path: &'static str,
}

impl FoundationalPerformanceRuntimeAdoptionPressureEvidence {
    pub(super) const fn new(
        pressure: FoundationalPerformanceRuntimeAdoptionPressure,
        evidence_path: &'static str,
    ) -> Self {
        Self {
            pressure,
            evidence_path,
        }
    }

    pub const fn pressure(&self) -> FoundationalPerformanceRuntimeAdoptionPressure {
        self.pressure
    }

    pub const fn evidence_path(&self) -> &'static str {
        self.evidence_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceHarnessExpansionPoint {
    PolicyUnavailableSectionMatrix,
}
