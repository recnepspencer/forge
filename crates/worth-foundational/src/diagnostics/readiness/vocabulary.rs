#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticProductionReadinessScope {
    milestone: &'static str,
}

impl FoundationalDiagnosticProductionReadinessScope {
    pub(super) const fn milestone_6() -> Self {
        Self {
            milestone: "worth-foundational.milestone-6",
        }
    }

    pub const fn milestone(&self) -> &'static str {
        self.milestone
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticCertifiedSurface {
    PrimitiveAndCategoryLaw,
    OutcomeSubjectAndRowTopology,
    MaterializationSupportAndNamedGapLaw,
    CanonicalBasisAndComparisonLaw,
    CertifiedBundleAndAttachmentCompatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticCertifiedSurfaceEvidence {
    surface: FoundationalDiagnosticCertifiedSurface,
    hostile_pressure: FoundationalDiagnosticSyntheticRuntimePressure,
    compile_fail_boundary: FoundationalDiagnosticCompileFailBoundary,
    owning_test_path: &'static str,
    compile_fail_evidence_path: &'static str,
    blind_consumer_evidence_path: &'static str,
}

impl FoundationalDiagnosticCertifiedSurfaceEvidence {
    pub(super) const fn new(
        surface: FoundationalDiagnosticCertifiedSurface,
        hostile_pressure: FoundationalDiagnosticSyntheticRuntimePressure,
        compile_fail_boundary: FoundationalDiagnosticCompileFailBoundary,
        owning_test_path: &'static str,
        compile_fail_evidence_path: &'static str,
        blind_consumer_evidence_path: &'static str,
    ) -> Self {
        Self {
            surface,
            hostile_pressure,
            compile_fail_boundary,
            owning_test_path,
            compile_fail_evidence_path,
            blind_consumer_evidence_path,
        }
    }

    pub const fn surface(&self) -> FoundationalDiagnosticCertifiedSurface {
        self.surface
    }

    pub const fn hostile_pressure(&self) -> FoundationalDiagnosticSyntheticRuntimePressure {
        self.hostile_pressure
    }

    pub const fn compile_fail_boundary(&self) -> FoundationalDiagnosticCompileFailBoundary {
        self.compile_fail_boundary
    }

    pub const fn owning_test_path(&self) -> &'static str {
        self.owning_test_path
    }

    pub const fn compile_fail_evidence_path(&self) -> &'static str {
        self.compile_fail_evidence_path
    }

    pub const fn blind_consumer_evidence_path(&self) -> &'static str {
        self.blind_consumer_evidence_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticSyntheticRuntimePressure {
    PrimitiveNonSubstitution,
    GenericRowCollapseRejection,
    HiddenRediscoveryDebtRejection,
    ThinOrEmptySupportOverclaimRejection,
    BlindConsumerCanonicalParity,
    HiddenSourceDigestOrCoverageWORTHry,
    ExplanationProvenanceBoundaryPreservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticSyntheticPressureEvidence {
    pressure: FoundationalDiagnosticSyntheticRuntimePressure,
    owning_test_path: &'static str,
}

impl FoundationalDiagnosticSyntheticPressureEvidence {
    pub(super) const fn new(
        pressure: FoundationalDiagnosticSyntheticRuntimePressure,
        owning_test_path: &'static str,
    ) -> Self {
        Self {
            pressure,
            owning_test_path,
        }
    }

    pub const fn pressure(&self) -> FoundationalDiagnosticSyntheticRuntimePressure {
        self.pressure
    }

    pub const fn owning_test_path(&self) -> &'static str {
        self.owning_test_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticCompileFailBoundary {
    PrimitiveAndCategoryPreserveNonSubstitution,
    RowTopologyPreservesFamilyAndLocatorLaw,
    MaterializationAndSupportPreserveExplicitSeams,
    BasisAndComparisonPreserveBlindConsumerCanonicalLaw,
    CertifiedBundleAndAttachmentReuseProofLane,
    DiagnosticReadinessRequiresCertifiedArtifact,
    DiagnosticReadinessAuthorityCannotBeMinted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticCompileFailEvidence {
    boundary: FoundationalDiagnosticCompileFailBoundary,
    evidence_path: &'static str,
}

impl FoundationalDiagnosticCompileFailEvidence {
    pub(super) const fn new(
        boundary: FoundationalDiagnosticCompileFailBoundary,
        evidence_path: &'static str,
    ) -> Self {
        Self {
            boundary,
            evidence_path,
        }
    }

    pub const fn boundary(&self) -> FoundationalDiagnosticCompileFailBoundary {
        self.boundary
    }

    pub const fn evidence_path(&self) -> &'static str {
        self.evidence_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticWORTHProofSurface {
    CertifiedDiagnosticAttachmentAuthority,
    ProofBearingCertifiedDiagnosticBundle,
    CertifiedBundleBoundaryBridge,
    CertifiedBundleReadmitWithAuthority,
    ProductionReadinessCertificationArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticWORTHProofApi {
    AuthorityWitnessFromAuthorityMarker,
    ProofFromAuthorityWitness,
    ArtifactWithProofsAndCurrentBasis,
    ArtifactBridgeTrustBoundary,
    ArtifactReadmitWithAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticWORTHProofApiEvidence {
    api: FoundationalDiagnosticWORTHProofApi,
    source_path: &'static str,
    source_snippet: &'static str,
}

impl FoundationalDiagnosticWORTHProofApiEvidence {
    pub(super) const fn new(
        api: FoundationalDiagnosticWORTHProofApi,
        source_path: &'static str,
        source_snippet: &'static str,
    ) -> Self {
        Self {
            api,
            source_path,
            source_snippet,
        }
    }

    pub const fn api(&self) -> FoundationalDiagnosticWORTHProofApi {
        self.api
    }

    pub const fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub const fn source_snippet(&self) -> &'static str {
        self.source_snippet
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticWORTHProofForbiddenSurface {
    PlainDiagnosticPrimitives,
    PlainDiagnosticRowsAndBundles,
    PlainMaterializationVocabulary,
    PlainCanonicalComparisonVocabulary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticRuntimeAssumption {
    Milestone2CanonicalizationRemainsAuthorityForDiagnosticBasis,
    Milestone3ProfilesGovernRichnessSupportAndCertificationPosture,
    Milestone4ArtifactLawGovernsDiagnosticCategoryAndDeliveryMeaning,
    Milestone5TransitionAndCurrentBasisSurfacesRemainAuthorityForTransitionAttachedDiagnostics,
    CertifiedDiagnosticBundlesReuseWORTHProofLane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticRuntimeNonAssumption {
    Milestone7ProvenanceAndReceiptOntologyAlreadyOwnedHere,
    OneDiagnosticsStoreOrReplayEngineExistsInFoundational,
    AdoptingRuntimeCoverageParityAlreadyProven,
    DescriptiveDiagnosticsBecomeAuthority,
    BoundaryCrossingPreservesCertifiedCurrentBasisWithoutReadmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticResidualDebt {
    AdoptingRuntimeParityDeferred,
    Milestone7ProvenanceAndReceiptDeepeningDeferred,
    RuntimeSpecificSupportTaxonomiesDeferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticMilestone6PhaseGate {
    PrimitiveAndCategoryLaw,
    OutcomeSubjectAndRowTopology,
    MaterializationSupportAndNamedGapLaw,
    CanonicalBasisAndComparisonLaw,
    CertifiedBundleAndAttachmentCompatibility,
    ProductionReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticPhaseGateEvidence {
    gate: FoundationalDiagnosticMilestone6PhaseGate,
    evidence_path: &'static str,
}

impl FoundationalDiagnosticPhaseGateEvidence {
    pub(super) const fn new(
        gate: FoundationalDiagnosticMilestone6PhaseGate,
        evidence_path: &'static str,
    ) -> Self {
        Self {
            gate,
            evidence_path,
        }
    }

    pub const fn gate(&self) -> FoundationalDiagnosticMilestone6PhaseGate {
        self.gate
    }

    pub const fn evidence_path(&self) -> &'static str {
        self.evidence_path
    }
}
