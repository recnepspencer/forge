use forge_store_blob_chunks::{
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass,
    BlobHarnessChunkTopology, BlobHarnessFailurePoint, BlobHarnessPlacementClass,
    BlobHarnessSecurityScopeClass, BlobHarnessSizeClass,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalScenarioExpectation {
    kind: PhysicalScenarioExpectationKind,
    non_claims: Vec<PhysicalScenarioNonClaim>,
    blob_harness_topology: Option<BlobHarnessChunkTopology>,
    blob_harness_metadata: Option<BlobHarnessScenarioMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobHarnessScenarioMetadata {
    size_class: BlobHarnessSizeClass,
    chunk_size_class: BlobHarnessChunkSizeClass,
    placement_class: BlobHarnessPlacementClass,
    security_scope_class: BlobHarnessSecurityScopeClass,
    access_mode: BlobHarnessAccessMode,
    failure_point: BlobHarnessFailurePoint,
    actor_mix: BlobHarnessActorMix,
}

impl PhysicalScenarioExpectation {
    pub fn recovery_dogfood() -> Self {
        Self::new(PhysicalScenarioExpectationKind::RecoveryDogfood, Vec::new())
    }

    pub fn shortcut_rejection_dogfood() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::ShortcutRejectionDogfood,
            Vec::new(),
        )
    }

    pub fn non_claiming_physical_isolation_readiness_shape() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::PhysicalIsolationReadinessShapeProbe,
            vec![PhysicalScenarioNonClaim::NoPhysicalIsolationCorrectnessClaim],
        )
    }

    pub fn non_claiming_physical_isolation_readiness_with_shortcut_rejection() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::PhysicalIsolationReadinessWithShortcutRejectionProbe,
            vec![PhysicalScenarioNonClaim::NoPhysicalIsolationCorrectnessClaim],
        )
    }

    pub fn non_claiming_physical_isolation_checkpoint_publication_crash_replay() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::PhysicalIsolationCheckpointPublicationCrashReplay,
            vec![PhysicalScenarioNonClaim::NoPhysicalIsolationCorrectnessClaim],
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

    pub fn physical_isolation_interleaving() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::PhysicalIsolationInterleaving,
            Vec::new(),
        )
    }

    pub fn non_claiming_blob_harness_seed(
        topology: BlobHarnessChunkTopology,
        metadata: BlobHarnessScenarioMetadata,
    ) -> Self {
        Self {
            kind: PhysicalScenarioExpectationKind::BlobHarnessSeed,
            non_claims: vec![PhysicalScenarioNonClaim::NoBlobOperationCorrectnessClaim],
            blob_harness_topology: Some(topology),
            blob_harness_metadata: Some(metadata),
        }
    }

    pub fn physical_isolation_denial() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::PhysicalIsolationDenial,
            Vec::new(),
        )
    }

    pub fn io_pressure_simulation() -> Self {
        Self::new(
            PhysicalScenarioExpectationKind::IoPressureSimulation,
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

    pub const fn blob_harness_topology(&self) -> Option<BlobHarnessChunkTopology> {
        self.blob_harness_topology
    }

    pub const fn blob_harness_metadata(&self) -> Option<BlobHarnessScenarioMetadata> {
        self.blob_harness_metadata
    }

    fn new(
        kind: PhysicalScenarioExpectationKind,
        non_claims: Vec<PhysicalScenarioNonClaim>,
    ) -> Self {
        Self {
            kind,
            non_claims,
            blob_harness_topology: None,
            blob_harness_metadata: None,
        }
    }
}

impl BlobHarnessScenarioMetadata {
    pub const fn new(
        size_class: BlobHarnessSizeClass,
        chunk_size_class: BlobHarnessChunkSizeClass,
        placement_class: BlobHarnessPlacementClass,
        security_scope_class: BlobHarnessSecurityScopeClass,
        access_mode: BlobHarnessAccessMode,
        failure_point: BlobHarnessFailurePoint,
        actor_mix: BlobHarnessActorMix,
    ) -> Self {
        Self {
            size_class,
            chunk_size_class,
            placement_class,
            security_scope_class,
            access_mode,
            failure_point,
            actor_mix,
        }
    }

    pub const fn size_class(self) -> BlobHarnessSizeClass {
        self.size_class
    }

    pub const fn chunk_size_class(self) -> BlobHarnessChunkSizeClass {
        self.chunk_size_class
    }

    pub const fn placement_class(self) -> BlobHarnessPlacementClass {
        self.placement_class
    }

    pub const fn security_scope_class(self) -> BlobHarnessSecurityScopeClass {
        self.security_scope_class
    }

    pub const fn access_mode(self) -> BlobHarnessAccessMode {
        self.access_mode
    }

    pub const fn failure_point(self) -> BlobHarnessFailurePoint {
        self.failure_point
    }

    pub const fn actor_mix(self) -> BlobHarnessActorMix {
        self.actor_mix
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioExpectationKind {
    RecoveryDogfood,
    PhysicalIsolationReadinessShapeProbe,
    PhysicalIsolationReadinessWithShortcutRejectionProbe,
    PhysicalIsolationCheckpointPublicationCrashReplay,
    StableReadPlanCounterContracts,
    StableReadPlanTranscriptReplay,
    StableReadPlanDenial,
    PhysicalIsolationInterleaving,
    PhysicalIsolationDenial,
    IoPressureSimulation,
    BlobHarnessSeed,
    ShortcutRejectionDogfood,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioNonClaim {
    NoPhysicalIsolationCorrectnessClaim,
    NoRealBackendSafetyQualification,
    NoBlobOperationCorrectnessClaim,
    FutureExtensionSlotDoesNotImplementFutureBehavior,
}
