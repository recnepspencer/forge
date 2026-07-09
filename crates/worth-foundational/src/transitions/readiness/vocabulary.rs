#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionProductionReadinessScope {
    milestone: &'static str,
}

impl FoundationalTransitionProductionReadinessScope {
    pub(super) const fn milestone_5() -> Self {
        Self {
            milestone: "worth-foundational.milestone-5",
        }
    }

    pub(super) const fn milestone_9_scoped_merge() -> Self {
        Self {
            milestone: "worth-foundational.milestone-9.scoped-merge",
        }
    }

    pub const fn milestone(&self) -> &'static str {
        self.milestone
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalTransitionCertifiedSurface {
    BranchLocalSeparation,
    MergeVerdictLaw,
    CommittedAuthorityTransitions,
    CommitReceiptsAndBundles,
    CanonicalBasisAndLocatorIntegration,
    ProfileRichnessAndCurrentBasisBehavior,
    ScopedMergeRequestVocabulary,
    ScopedMergeAdmissionEvidence,
    ScopedMergeDenialUnavailableTopology,
    ScopedMergeCanonicalLocatorDiagnostics,
    ScopedMergeAdoptionContract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionCertifiedSurfaceEvidence {
    surface: FoundationalTransitionCertifiedSurface,
    hostile_pressure: FoundationalTransitionSyntheticRuntimePressure,
    compile_fail_boundary: FoundationalTransitionCompileFailBoundary,
    owning_test_path: &'static str,
    compile_fail_evidence_path: &'static str,
    blind_consumer_evidence_path: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionSyntheticPressureEvidence {
    pressure: FoundationalTransitionSyntheticRuntimePressure,
    owning_test_path: &'static str,
}

impl FoundationalTransitionSyntheticPressureEvidence {
    pub(super) const fn new(
        pressure: FoundationalTransitionSyntheticRuntimePressure,
        owning_test_path: &'static str,
    ) -> Self {
        Self {
            pressure,
            owning_test_path,
        }
    }

    pub const fn pressure(&self) -> FoundationalTransitionSyntheticRuntimePressure {
        self.pressure
    }

    pub const fn owning_test_path(&self) -> &'static str {
        self.owning_test_path
    }
}

impl FoundationalTransitionCertifiedSurfaceEvidence {
    pub(super) const fn new(
        surface: FoundationalTransitionCertifiedSurface,
        hostile_pressure: FoundationalTransitionSyntheticRuntimePressure,
        compile_fail_boundary: FoundationalTransitionCompileFailBoundary,
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

    pub const fn surface(&self) -> FoundationalTransitionCertifiedSurface {
        self.surface
    }

    pub const fn hostile_pressure(&self) -> FoundationalTransitionSyntheticRuntimePressure {
        self.hostile_pressure
    }

    pub const fn compile_fail_boundary(&self) -> FoundationalTransitionCompileFailBoundary {
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
pub enum FoundationalTransitionSyntheticRuntimePressure {
    AuthoritySeparation,
    MergeTopologyHonesty,
    NoOpVersusCommitClassification,
    ReceiptIssuanceBoundary,
    ReplayInterpretationBoundary,
    ReducedRichnessPreservation,
    AmbientBasisChoiceHostility,
    HiddenStrategyInfluenceHostility,
    ThinReceiptRejection,
    GenericTransitionResultBagRejection,
    CheapConvenienceBypassRejection,
    ScopedMergeCategorySubstitutionHostility,
    ScopedMergeProducerDiversityHostility,
    ScopedMergeUnavailableDenialHonesty,
    ScopedMergeCanonicalLocatorStability,
    ScopedMergeRuntimeBoundaryHonesty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalTransitionCompileFailBoundary {
    BranchLocalSurfacesCannotSatisfyAuthorityApis,
    MergeAdmissionSurfacesRemainNonAuthoritative,
    CommittedAuthorityRequiresProofBearingAdmission,
    ReceiptAndCloseoutPreserveAuthoritySeparation,
    Phase5BasisAndCurrentBasisRequireStrengthenedArtifacts,
    TransitionReadinessRequiresCertifiedArtifact,
    TransitionReadinessAuthorityCannotBeMinted,
    ScopedMergeScopeRequiresTypedLoci,
    SelectedScopeLocatorRequiresTypedLoci,
    SelectedNodeAndAspectRequestsAreNotSubstitutable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalTransitionWORTHProofSurface {
    TransitionOutcomeAdmissionLane,
    AuthorityWitnessScopedAdmission,
    ProofBearingCommittedAuthorityArtifact,
    ProofBearingCommitReceiptArtifact,
    CurrentBasisArtifactConstructor,
    BoundaryBridgeTrustBoundary,
    BoundaryReadmitWithAuthority,
    ProductionReadinessCertificationArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalTransitionWORTHProofApi {
    TransitionOutcomeStructuredCategories,
    AuthorityWitnessFromAuthorityMarker,
    ProofFromAuthorityWitness,
    ArtifactWithProofsAndCurrentBasis,
    ArtifactWithCurrentBasis,
    ArtifactBridgeTrustBoundary,
    ArtifactReadmitWithAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionWORTHProofApiEvidence {
    api: FoundationalTransitionWORTHProofApi,
    source_path: &'static str,
    source_snippet: &'static str,
}

impl FoundationalTransitionWORTHProofApiEvidence {
    pub(super) const fn new(
        api: FoundationalTransitionWORTHProofApi,
        source_path: &'static str,
        source_snippet: &'static str,
    ) -> Self {
        Self {
            api,
            source_path,
            source_snippet,
        }
    }

    pub const fn api(&self) -> FoundationalTransitionWORTHProofApi {
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
pub enum FoundationalTransitionWORTHProofForbiddenSurface {
    PlainBranchLocalVocabulary,
    PlainMergeVerdictVocabulary,
    PlainReceiptAndBundleVocabulary,
    PlainCanonicalBasisAndLocatorVocabulary,
    PlainScopedMergeStrings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalTransitionRuntimeAssumption {
    Milestone2CanonicalizationRemainsAuthorityForTransitionBasisReadiness,
    Milestone3ProfilesGovernTransitionAttachmentAndElision,
    StrongerCommittedAuthorityAndReceiptClaimsUseWORTHProof,
    TransitionMeaningRemainsFacadeControlled,
    ScopedMergeVocabularyMustPrecedeRuntimeExecution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalTransitionRuntimeNonAssumption {
    DiagnosticsOntologyAlreadyOwnedHere,
    ProvenanceOntologyBeyondTransitionRowsAlreadyOwnedHere,
    AdoptingRuntimeMergeStrategyParityAlreadyProven,
    BoundaryCrossingPreservesCurrentBasisWithoutReadmission,
    GenericBranchOrMergeEngineExistsInFoundational,
    FoundationalExecutesScopedMergeOrCherryPick,
    AdoptingCratesMayInventScopedMergeDialect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalTransitionResidualDebt {
    AdoptingRuntimeParityDeferred,
    LaterDiagnosticsAndProvenanceOntologyDeferred,
    RuntimeStrategyRegistryAndExecutionDeferred,
    FullLineageSupportBeyondTransitionRowsDeferred,
    AdoptingCrateScopedMergeExecutionDeferred,
    NativeCherryPickExecutionDeferred,
    RuntimeConflictMaterializationDeferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalTransitionMilestone5PhaseGate {
    BranchLocalSeparation,
    MergeVerdictLaw,
    CommittedAuthorityTransitionLaw,
    CommitReceiptsAndBundles,
    CanonicalBasisLocatorAndProfileIntegration,
    ProductionReadiness,
    ScopedMergeRequestVocabulary,
    ScopedMergeAdmissionEvidence,
    ScopedMergeDenialUnavailableTopology,
    ScopedMergeCanonicalLocatorDiagnostics,
    ScopedMergeProductionReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionPhaseGateEvidence {
    gate: FoundationalTransitionMilestone5PhaseGate,
    evidence_path: &'static str,
}

impl FoundationalTransitionPhaseGateEvidence {
    pub(super) const fn new(
        gate: FoundationalTransitionMilestone5PhaseGate,
        evidence_path: &'static str,
    ) -> Self {
        Self {
            gate,
            evidence_path,
        }
    }

    pub const fn gate(&self) -> FoundationalTransitionMilestone5PhaseGate {
        self.gate
    }

    pub const fn evidence_path(&self) -> &'static str {
        self.evidence_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionCompileFailEvidence {
    boundary: FoundationalTransitionCompileFailBoundary,
    evidence_path: &'static str,
}

impl FoundationalTransitionCompileFailEvidence {
    pub(super) const fn new(
        boundary: FoundationalTransitionCompileFailBoundary,
        evidence_path: &'static str,
    ) -> Self {
        Self {
            boundary,
            evidence_path,
        }
    }

    pub const fn boundary(&self) -> FoundationalTransitionCompileFailBoundary {
        self.boundary
    }

    pub const fn evidence_path(&self) -> &'static str {
        self.evidence_path
    }
}
