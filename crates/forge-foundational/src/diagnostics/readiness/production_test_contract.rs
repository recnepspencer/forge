#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticCanonicalGoldenArtifact {
    PrimitiveCategoryAndMaterializationMeaning,
    FamilyDistinctRowTopologyMeaning,
    MaterializationRichnessAndDebtMeaning,
    CanonicalBundleAndComparisonMeaning,
    CertifiedCoverageAndAttachmentMeaning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticCanonicalGoldenArtifactEvidence {
    artifact: FoundationalDiagnosticCanonicalGoldenArtifact,
    evidence_path: &'static str,
}

impl FoundationalDiagnosticCanonicalGoldenArtifactEvidence {
    pub(super) const fn new(
        artifact: FoundationalDiagnosticCanonicalGoldenArtifact,
        evidence_path: &'static str,
    ) -> Self {
        Self {
            artifact,
            evidence_path,
        }
    }

    pub const fn artifact(&self) -> FoundationalDiagnosticCanonicalGoldenArtifact {
        self.artifact
    }

    pub const fn evidence_path(&self) -> &'static str {
        self.evidence_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticPropertySeed {
    PrimitiveOrderingAndTokenCanonicalization,
    RowFamilyOrderingAndSemanticTieBreaks,
    RichnessElisionPreservesTruthUnderPartiality,
    IndependentProducerCanonicalParity,
    CertifiedCoverageNamedGapParity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticPropertySeedEvidence {
    seed: FoundationalDiagnosticPropertySeed,
    owning_test_path: &'static str,
    hostile_dimension: &'static str,
}

impl FoundationalDiagnosticPropertySeedEvidence {
    pub(super) const fn new(
        seed: FoundationalDiagnosticPropertySeed,
        owning_test_path: &'static str,
        hostile_dimension: &'static str,
    ) -> Self {
        Self {
            seed,
            owning_test_path,
            hostile_dimension,
        }
    }

    pub const fn seed(&self) -> FoundationalDiagnosticPropertySeed {
        self.seed
    }

    pub const fn owning_test_path(&self) -> &'static str {
        self.owning_test_path
    }

    pub const fn hostile_dimension(&self) -> &'static str {
        self.hostile_dimension
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticHarnessExpansionPoint {
    IndependentProducerDiagnosticParityMatrix,
    RichnessAvailabilityAndFallbackReplayMatrix,
    BlindConsumerInterpretationReplaySuite,
    CertifiedCoverageAttachmentParityMatrix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticHarnessExpansionEvidence {
    point: FoundationalDiagnosticHarnessExpansionPoint,
    owning_test_path: &'static str,
}

impl FoundationalDiagnosticHarnessExpansionEvidence {
    pub(super) const fn new(
        point: FoundationalDiagnosticHarnessExpansionPoint,
        owning_test_path: &'static str,
    ) -> Self {
        Self {
            point,
            owning_test_path,
        }
    }

    pub const fn point(&self) -> FoundationalDiagnosticHarnessExpansionPoint {
        self.point
    }

    pub const fn owning_test_path(&self) -> &'static str {
        self.owning_test_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticRuntimeAdoptionFailurePressure {
    RuntimeLoweringMayMisclassifyEvidencePosture,
    RuntimeMaterializersMayOverclaimDurableOrCertifiedSupport,
    RuntimeCanonicalRowOrderingMayDriftAcrossStorageLayouts,
    RuntimeCoverageMatricesMayOmitRequiredFamilies,
    RuntimeProvenanceReadyRowsMayCollapseIntoExplanationRows,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticAdoptionShapedFollowthrough {
    ForgeHarnessDiagnosticProducerParityMatrix,
    ForgeHarnessRichnessAvailabilityAndFallbackReplaySuite,
    AdoptingRuntimeDiagnosticLoweringParityPressure,
    AdoptingRuntimeCertifiedCoverageAndAttachmentHostility,
}
