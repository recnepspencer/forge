#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkBackendProfileEvidence {
    SimulatedStrictDurable,
    PosixFileFsyncDirSync,
    WindowsFlushFileBuffers,
    MmapFlushNotDurabilityCertified,
    AdversarialLostFlush,
    AdversarialReorderedFlush,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkBackendEvidenceClass {
    DeclaredByConfig,
    ObservedByProbe,
    EstablishedByFilesystemAdmission,
    ExternallyGuaranteed,
    UnverifiableAssumption,
    CertifiedBackendProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkSchedulerEvidence {
    backend_profile: PhysicalWorkBackendProfileEvidence,
    evidence_class: PhysicalWorkBackendEvidenceClass,
    grouped_writes: u32,
    primary_backend_requirement: u8,
    secondary_present: bool,
}

impl PhysicalWorkSchedulerEvidence {
    pub const fn backend_profile(self) -> PhysicalWorkBackendProfileEvidence {
        self.backend_profile
    }

    pub const fn evidence_class(self) -> PhysicalWorkBackendEvidenceClass {
        self.evidence_class
    }

    pub const fn grouped_writes(self) -> u32 {
        self.grouped_writes
    }

    pub const fn primary_backend_requirement(self) -> u8 {
        self.primary_backend_requirement
    }

    pub const fn secondary_present(self) -> bool {
        self.secondary_present
    }

    pub(super) const fn new(
        backend_profile: PhysicalWorkBackendProfileEvidence,
        evidence_class: PhysicalWorkBackendEvidenceClass,
        grouped_writes: u32,
        primary_backend_requirement: u8,
        secondary_present: bool,
    ) -> Self {
        Self {
            backend_profile,
            evidence_class,
            grouped_writes,
            primary_backend_requirement,
            secondary_present,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkFamilyEvidence {
    ArtifactMetadataRead,
    ArtifactRangeRead,
    ArtifactRangeWrite,
    ArtifactPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkPressureEvidence {
    Unscheduled,
    ForegroundPointRead,
    ForegroundRangeRead,
    ForegroundInteractiveRead,
    ForegroundInternalRead,
    ForegroundMutation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkCounterStageEvidence {
    Declared,
    Blocked,
    Ready,
    Queued,
    Dispatched,
    Settling,
    Terminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkCounterEvidence {
    family: PhysicalWorkFamilyEvidence,
    pressure: PhysicalWorkPressureEvidence,
    stage: PhysicalWorkCounterStageEvidence,
    count: u64,
}

impl PhysicalWorkCounterEvidence {
    pub const fn family(self) -> PhysicalWorkFamilyEvidence {
        self.family
    }

    pub const fn pressure(self) -> PhysicalWorkPressureEvidence {
        self.pressure
    }

    pub const fn stage(self) -> PhysicalWorkCounterStageEvidence {
        self.stage
    }

    pub const fn count(self) -> u64 {
        self.count
    }

    pub(super) const fn new(
        family: PhysicalWorkFamilyEvidence,
        pressure: PhysicalWorkPressureEvidence,
        stage: PhysicalWorkCounterStageEvidence,
        count: u64,
    ) -> Self {
        Self {
            family,
            pressure,
            stage,
            count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkEffectFateEvidence {
    ProvenNoEffect,
    ReadCompleted,
    ReadIncomplete,
    WriteCompleted,
    PublicationCompleted,
    WrittenButSchedulerRejected,
    Indeterminate,
    StaleOrForeignOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkRecoveryEvidence {
    NoEffect,
    RetryExact,
    ContinueSettlement,
    InspectionRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkSignalSettlementEvidence {
    Committed,
    ReconciledFromPhysicalTruth,
    DerivedStateUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkCausalEvidence {
    operation: u64,
    signal_request: u64,
    signal_generation: u64,
    signal_predecessor_request: Option<u64>,
    signal_predecessor_generation: Option<u64>,
    signal_attempt: u64,
    scheduler: PhysicalWorkSchedulerEvidence,
    backend_operation: Option<u64>,
    effect_fate: PhysicalWorkEffectFateEvidence,
    recovery: PhysicalWorkRecoveryEvidence,
    signal_settlement: Option<PhysicalWorkSignalSettlementEvidence>,
    counters: Box<[PhysicalWorkCounterEvidence]>,
}

impl PhysicalWorkCausalEvidence {
    pub const fn operation(&self) -> u64 {
        self.operation
    }

    pub const fn signal_request(&self) -> u64 {
        self.signal_request
    }

    pub const fn signal_generation(&self) -> u64 {
        self.signal_generation
    }

    pub const fn signal_predecessor_request(&self) -> Option<u64> {
        self.signal_predecessor_request
    }

    pub const fn signal_predecessor_generation(&self) -> Option<u64> {
        self.signal_predecessor_generation
    }

    pub const fn signal_attempt(&self) -> u64 {
        self.signal_attempt
    }

    pub const fn scheduler(&self) -> PhysicalWorkSchedulerEvidence {
        self.scheduler
    }

    pub const fn backend_operation(&self) -> Option<u64> {
        self.backend_operation
    }

    pub const fn effect_fate(&self) -> PhysicalWorkEffectFateEvidence {
        self.effect_fate
    }

    pub const fn recovery(&self) -> PhysicalWorkRecoveryEvidence {
        self.recovery
    }

    pub const fn signal_settlement(&self) -> Option<PhysicalWorkSignalSettlementEvidence> {
        self.signal_settlement
    }

    pub const fn counters(&self) -> &[PhysicalWorkCounterEvidence] {
        &self.counters
    }

    pub(super) fn new(
        identity: PhysicalWorkCausalIdentity,
        scheduler: PhysicalWorkSchedulerEvidence,
        outcome: PhysicalWorkCausalOutcome,
        counters: Box<[PhysicalWorkCounterEvidence]>,
    ) -> Self {
        Self {
            operation: identity.operation,
            signal_request: identity.signal_request,
            signal_generation: identity.signal_generation,
            signal_predecessor_request: identity.signal_predecessor_request,
            signal_predecessor_generation: identity.signal_predecessor_generation,
            signal_attempt: identity.signal_attempt,
            scheduler,
            backend_operation: outcome.backend_operation,
            effect_fate: outcome.effect_fate,
            recovery: outcome.recovery,
            signal_settlement: outcome.signal_settlement,
            counters,
        }
    }
}

pub(super) struct PhysicalWorkCausalIdentity {
    pub(super) operation: u64,
    pub(super) signal_request: u64,
    pub(super) signal_generation: u64,
    pub(super) signal_predecessor_request: Option<u64>,
    pub(super) signal_predecessor_generation: Option<u64>,
    pub(super) signal_attempt: u64,
}

pub(super) struct PhysicalWorkCausalOutcome {
    pub(super) backend_operation: Option<u64>,
    pub(super) effect_fate: PhysicalWorkEffectFateEvidence,
    pub(super) recovery: PhysicalWorkRecoveryEvidence,
    pub(super) signal_settlement: Option<PhysicalWorkSignalSettlementEvidence>,
}
