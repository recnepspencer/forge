use forge_store_blob_chunks::BlobHarnessChunkTopology;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalScenarioExpectation {
    kind: PhysicalScenarioExpectationKind,
    non_claims: Vec<PhysicalScenarioNonClaim>,
    s7_blob_harness_topology: Option<BlobHarnessChunkTopology>,
}

impl PhysicalScenarioExpectation {
    pub fn s4_recovery_dogfood() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::S4RecoveryDogfood,
            Vec::new(),
        )
    }

    pub fn shortcut_rejection_dogfood() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::ShortcutRejectionDogfood,
            Vec::new(),
        )
    }

    pub fn non_claiming_s5_readiness_shape() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::S5ReadinessShapeProbe,
            vec![PhysicalScenarioNonClaim::NoS5PhysicalIsolationCorrectnessClaim],
        )
    }

    pub fn non_claiming_s5_readiness_with_shortcut_rejection() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::S5ReadinessWithShortcutRejectionProbe,
            vec![PhysicalScenarioNonClaim::NoS5PhysicalIsolationCorrectnessClaim],
        )
    }

    pub fn non_claiming_s5_checkpoint_publication_crash_replay() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::S5CheckpointPublicationCrashReplay,
            vec![PhysicalScenarioNonClaim::NoS5PhysicalIsolationCorrectnessClaim],
        )
    }

    pub fn stable_read_plan_counter_contracts() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::StableReadPlanCounterContracts,
            Vec::new(),
        )
    }

    pub fn stable_read_plan_transcript_replay() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::StableReadPlanTranscriptReplay,
            Vec::new(),
        )
    }

    pub fn stable_read_plan_denial() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::StableReadPlanDenial,
            Vec::new(),
        )
    }

    pub fn s5_physical_isolation_interleaving() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::S5PhysicalIsolationInterleaving,
            Vec::new(),
        )
    }

    pub fn non_claiming_s7_blob_harness_seed(topology: BlobHarnessChunkTopology) -> Self {
        Self {
            kind: PhysicalScenarioExpectationKind::S7BlobHarnessSeed,
            non_claims: vec![PhysicalScenarioNonClaim::NoS7BlobOperationCorrectnessClaim],
            s7_blob_harness_topology: Some(topology),
        }
    }

    pub fn s5_physical_isolation_denial() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::S5PhysicalIsolationDenial,
            Vec::new(),
        )
    }

    pub fn s6_io_pressure_simulation() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::S6IoPressureSimulation,
            vec![PhysicalScenarioNonClaim::NoRealBackendSafetyQualification],
        )
    }

    pub fn with_future_extension_non_claim(mut self) -> Self {
        let non_claim = PhysicalScenarioNonClaim::FutureExtensionSlotDoesNotImplementFutureBehavior;
        if !self.non_claims.contains(&non_claim) {
            self.non_claims.push(non_claim);
        }
        self
    }

    pub const fn kind(&self) -> PhysicalScenarioExpectationKind {
        self.kind
    }

    pub fn non_claims(&self) -> &[PhysicalScenarioNonClaim] {
        &self.non_claims
    }

    pub const fn s7_blob_harness_topology(&self) -> Option<BlobHarnessChunkTopology> {
        self.s7_blob_harness_topology
    }

    fn new(
        kind: PhysicalScenarioExpectationKind,
        non_claims: Vec<PhysicalScenarioNonClaim>,
    ) -> Self {
        Self {
            kind,
            non_claims,
            s7_blob_harness_topology: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioExpectationKind {
    S4RecoveryDogfood,
    S5ReadinessShapeProbe,
    S5ReadinessWithShortcutRejectionProbe,
    S5CheckpointPublicationCrashReplay,
    StableReadPlanCounterContracts,
    StableReadPlanTranscriptReplay,
    StableReadPlanDenial,
    S5PhysicalIsolationInterleaving,
    S5PhysicalIsolationDenial,
    S6IoPressureSimulation,
    S7BlobHarnessSeed,
    ShortcutRejectionDogfood,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioNonClaim {
    NoS5PhysicalIsolationCorrectnessClaim,
    NoRealBackendSafetyQualification,
    NoS7BlobOperationCorrectnessClaim,
    FutureExtensionSlotDoesNotImplementFutureBehavior,
}
