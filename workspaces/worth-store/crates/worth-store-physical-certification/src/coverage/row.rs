use crate::{
    CounterContractKind, FixtureClassKind, IoPressureEvidenceMaturity, IoPressureFaultKind,
    IoPressureHarnessSecureIoPosture, ObserverKind, OracleFamilyKind, PhysicalDriverKind,
    PhysicalFaultEvidenceClass, PhysicalIsolationCompactionMutationKind,
    PhysicalIsolationMutationKind, PhysicalProofOracleKind, PhysicalScenarioActorRole,
    PhysicalScenarioFaultKind, PhysicalSimulationProfile,
};
use worth_store_blob_chunks::{
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass, BlobHarnessFailurePoint,
    BlobHarnessPlacementClass, BlobHarnessSecurityScopeClass, BlobHarnessSizeClass,
};
use worth_store_budgets::BlobHarnessEnvelopeProfile;
use worth_store_io_scheduler::{
    foreground_reservation::ForegroundIoLaneKind, BackgroundIoPressureClass,
};
use worth_store_physical_backend::BackendTargetProfile;

use super::{HarnessCoverageStage, HarnessSubsystem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CoverageSurfaceKind {
    Scenario,
    Plan,
    YieldpointSchedule,
    Actor,
    Driver,
    Oracle,
    Counter,
    Transcript,
    MutationResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageRowDimension {
    ArtifactClass(FixtureClassKind),
    ActorRole(PhysicalScenarioActorRole),
    ProductionBoundaryYieldpoint(String),
    FaultPhase(PhysicalScenarioFaultKind),
    ResourceEnvelopeProfile(PhysicalSimulationProfile),
    BackgroundInterference(PhysicalDriverKind),
    AuthorityFamily(OracleFamilyKind),
    Oracle(PhysicalProofOracleKind),
    OfflineVerifier(ObserverKind),
    CounterContract(CounterContractKind),
    TranscriptOutput,
    MutationValidationPosture(MutationValidationPosture),
    CompactionMutation(PhysicalIsolationCompactionMutationKind),
    PhysicalIsolationMutation(PhysicalIsolationMutationKind),
    IoPressureBackendTarget(BackendTargetProfile),
    IoPressureForegroundLane(ForegroundIoLaneKind),
    IoPressureBackgroundPressure(BackgroundIoPressureClass),
    SecureIoPosture(IoPressureHarnessSecureIoPosture),
    IoPressureFaultKind(IoPressureFaultKind),
    IoPressureFaultEvidenceClass(PhysicalFaultEvidenceClass),
    IoPressureEvidenceMaturity(IoPressureEvidenceMaturity),
    BlobSizeClass(BlobHarnessSizeClass),
    BlobChunkCount(u64),
    BlobChunkSizeClass(BlobHarnessChunkSizeClass),
    BlobSecurityScopeClass(BlobHarnessSecurityScopeClass),
    BlobPlacementClass(BlobHarnessPlacementClass),
    BlobAccessMode(BlobHarnessAccessMode),
    BlobFailurePoint(BlobHarnessFailurePoint),
    BlobMemoryEnvelopeProfile(BlobHarnessEnvelopeProfile),
    BlobActorMix(BlobHarnessActorMix),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MutationValidationPosture {
    ExpectedFailureObserved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalCoverageMatrixRow {
    sequence: HarnessCoverageStage,
    subsystem: HarnessSubsystem,
    surface: CoverageSurfaceKind,
    source_identity: [u8; 32],
    dimensions: Vec<CoverageRowDimension>,
    receipt: CoverageRowSatisfiedReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageRowSatisfiedReceipt {
    surface: CoverageSurfaceKind,
    source_identity: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredScenarioCoverageRow(PhysicalCoverageMatrixRow);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredOracleCoverageRow(PhysicalCoverageMatrixRow);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredCounterCoverageRow(PhysicalCoverageMatrixRow);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredTranscriptCoverageRow(PhysicalCoverageMatrixRow);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationResultCoverageRow(PhysicalCoverageMatrixRow);

impl PhysicalCoverageMatrixRow {
    pub(crate) fn generated(
        sequence: HarnessCoverageStage,
        surface: CoverageSurfaceKind,
        source_identity: [u8; 32],
        dimensions: impl IntoIterator<Item = CoverageRowDimension>,
    ) -> Self {
        let subsystem = subsystem_for_surface(surface);
        let mut admitted_dimensions = Vec::new();
        for dimension in dimensions.into_iter() {
            if !admitted_dimensions.contains(&dimension) {
                admitted_dimensions.push(dimension);
            }
        }
        Self {
            sequence,
            subsystem,
            surface,
            source_identity,
            dimensions: admitted_dimensions,
            receipt: CoverageRowSatisfiedReceipt {
                surface,
                source_identity,
            },
        }
    }

    pub const fn sequence(&self) -> HarnessCoverageStage {
        self.sequence
    }

    pub const fn subsystem(&self) -> HarnessSubsystem {
        self.subsystem
    }

    pub const fn surface(&self) -> CoverageSurfaceKind {
        self.surface
    }

    pub const fn source_identity(&self) -> &[u8; 32] {
        &self.source_identity
    }

    pub fn dimensions(&self) -> &[CoverageRowDimension] {
        &self.dimensions
    }

    pub fn has_dimension(&self, dimension: &CoverageRowDimension) -> bool {
        self.dimensions.contains(dimension)
    }

    pub const fn receipt(&self) -> &CoverageRowSatisfiedReceipt {
        &self.receipt
    }
}

impl CoverageRowSatisfiedReceipt {
    pub const fn surface(&self) -> CoverageSurfaceKind {
        self.surface
    }

    pub const fn source_identity(&self) -> &[u8; 32] {
        &self.source_identity
    }
}

impl RegisteredScenarioCoverageRow {
    pub const fn row(&self) -> &PhysicalCoverageMatrixRow {
        &self.0
    }
}

impl RegisteredOracleCoverageRow {
    pub const fn row(&self) -> &PhysicalCoverageMatrixRow {
        &self.0
    }
}

impl RegisteredCounterCoverageRow {
    pub const fn row(&self) -> &PhysicalCoverageMatrixRow {
        &self.0
    }
}

impl RegisteredTranscriptCoverageRow {
    pub const fn row(&self) -> &PhysicalCoverageMatrixRow {
        &self.0
    }
}

impl MutationResultCoverageRow {
    pub const fn row(&self) -> &PhysicalCoverageMatrixRow {
        &self.0
    }
}

const fn subsystem_for_surface(surface: CoverageSurfaceKind) -> HarnessSubsystem {
    match surface {
        CoverageSurfaceKind::Scenario | CoverageSurfaceKind::Plan => {
            HarnessSubsystem::ScenarioDefinitions
        }
        CoverageSurfaceKind::YieldpointSchedule => HarnessSubsystem::DeterministicScheduler,
        CoverageSurfaceKind::Actor => HarnessSubsystem::ActorModel,
        CoverageSurfaceKind::Driver => HarnessSubsystem::ProductionDriverContracts,
        CoverageSurfaceKind::Oracle => HarnessSubsystem::CertificationOracleFamilies,
        CoverageSurfaceKind::Counter => HarnessSubsystem::CounterStrengthContracts,
        CoverageSurfaceKind::Transcript => HarnessSubsystem::ReplayableTranscripts,
        CoverageSurfaceKind::MutationResult => HarnessSubsystem::MutationValidation,
    }
}
