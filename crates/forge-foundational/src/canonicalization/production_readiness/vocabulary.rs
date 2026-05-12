#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalProductionReadinessScope {
    milestone: &'static str,
}

impl CanonicalProductionReadinessScope {
    pub(super) const fn milestone_2() -> Self {
        Self {
            milestone: "forge-foundational.milestone-2",
        }
    }

    pub const fn milestone(&self) -> &'static str {
        self.milestone
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalCertifiedSurface {
    BasisGrammar,
    MilestoneOneBasisBuilders,
    EquivalenceBasis,
    MismatchBasis,
    ExportBundles,
    DigestAlgorithmSlots,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalSyntheticRuntimePressure {
    OrderedAuthorityProducer,
    ReorderedCompatibilityProducer,
    SupportExportConsumer,
    CategoryAdjacentHostileProducer,
    BlindMismatchConsumer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalCompileFailBoundary {
    RawBasisCannotSatisfyReadiness,
    RawComparisonCannotSatisfyEquivalence,
    BoundaryExportRequiresReadmission,
    DigestDerivationRequiresReadyArtifact,
    ProductionReadinessRequiresCertifiedArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalGoldenArtifactEvidence {
    ValueFamilies,
    BoundaryDigestBases,
    IdentityAndLocator,
    EquivalenceBasis,
    MismatchBasis,
    ExportBundleManifest,
    DigestSlotDerivedValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalCostCounterEvidence {
    BasisSequenceConstruction,
    MilestoneOneSurfaceLowering,
    ExportManifestRows,
    DigestInputMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalPropertySeed {
    OrderingIndependence,
    CategoryAdjacency,
    CompatibilityLoweringParity,
    EquivalenceScope,
    MismatchLocus,
    DigestSlotHostility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalHarnessExpansionPoint {
    CanonicalBasisReplayLane,
    ExportFixtureReplayLane,
    DigestSlotHostilityLane,
    RuntimeParityRunMatrix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalRuntimeAssumption {
    FoundationalBasisLawCertified,
    CanonicalOrderingStable,
    ComparisonAndMismatchEvidenceSelfDescribing,
    DigestDerivationGatedByReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalRuntimeNonAssumption {
    RealRuntimeLoweringCorrect,
    FinalCryptographicPolicySelected,
    ProfilesReceiptsDiagnosticsOrBranchOntologyExists,
    DigestEqualityAuthorizesSemanticEquivalence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalResidualDebt {
    FinalCryptographicPolicyDeferred,
    RealRuntimeAdoptionParityDeferred,
    LaterMilestoneOntologyDeferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalMilestone2PhaseGate {
    BasisGrammar,
    MilestoneOneBasisBuilders,
    EquivalenceAndMismatch,
    ExportFixtures,
    DigestSlots,
    ProductionReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalCertifiedSurfaceEvidence {
    surface: CanonicalCertifiedSurface,
    hostile_pressure: CanonicalSyntheticRuntimePressure,
    blind_consumer_evidence_path: &'static str,
    compile_fail_boundary: CanonicalCompileFailBoundary,
    compile_fail_evidence_path: &'static str,
    cost_counter_evidence: CanonicalCostCounterEvidence,
    golden_artifact: Option<CanonicalGoldenArtifactEvidence>,
}

impl CanonicalCertifiedSurfaceEvidence {
    pub(super) const fn new(
        surface: CanonicalCertifiedSurface,
        hostile_pressure: CanonicalSyntheticRuntimePressure,
        blind_consumer_evidence_path: &'static str,
        compile_fail_boundary: CanonicalCompileFailBoundary,
        compile_fail_evidence_path: &'static str,
        cost_counter_evidence: CanonicalCostCounterEvidence,
        golden_artifact: Option<CanonicalGoldenArtifactEvidence>,
    ) -> Self {
        Self {
            surface,
            hostile_pressure,
            blind_consumer_evidence_path,
            compile_fail_boundary,
            compile_fail_evidence_path,
            cost_counter_evidence,
            golden_artifact,
        }
    }

    pub const fn surface(&self) -> CanonicalCertifiedSurface {
        self.surface
    }

    pub const fn hostile_pressure(&self) -> CanonicalSyntheticRuntimePressure {
        self.hostile_pressure
    }

    pub const fn blind_consumer_evidence_path(&self) -> &'static str {
        self.blind_consumer_evidence_path
    }

    pub const fn compile_fail_boundary(&self) -> CanonicalCompileFailBoundary {
        self.compile_fail_boundary
    }

    pub const fn compile_fail_evidence_path(&self) -> &'static str {
        self.compile_fail_evidence_path
    }

    pub const fn cost_counter_evidence(&self) -> CanonicalCostCounterEvidence {
        self.cost_counter_evidence
    }

    pub const fn golden_artifact(&self) -> Option<CanonicalGoldenArtifactEvidence> {
        self.golden_artifact
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalPhaseGateEvidence {
    gate: CanonicalMilestone2PhaseGate,
    evidence_path: &'static str,
}

impl CanonicalPhaseGateEvidence {
    pub(super) const fn new(
        gate: CanonicalMilestone2PhaseGate,
        evidence_path: &'static str,
    ) -> Self {
        Self {
            gate,
            evidence_path,
        }
    }

    pub const fn gate(&self) -> CanonicalMilestone2PhaseGate {
        self.gate
    }

    pub const fn evidence_path(&self) -> &'static str {
        self.evidence_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalFixtureManifestEvidence {
    artifact: CanonicalGoldenArtifactEvidence,
    owning_test: &'static str,
    harness_lane: CanonicalHarnessExpansionPoint,
}

impl CanonicalFixtureManifestEvidence {
    pub(super) const fn new(
        artifact: CanonicalGoldenArtifactEvidence,
        owning_test: &'static str,
        harness_lane: CanonicalHarnessExpansionPoint,
    ) -> Self {
        Self {
            artifact,
            owning_test,
            harness_lane,
        }
    }

    pub const fn artifact(&self) -> CanonicalGoldenArtifactEvidence {
        self.artifact
    }

    pub const fn owning_test(&self) -> &'static str {
        self.owning_test
    }

    pub const fn harness_lane(&self) -> CanonicalHarnessExpansionPoint {
        self.harness_lane
    }
}
