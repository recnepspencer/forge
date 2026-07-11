#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIsolationInterleavingHarnessCapability {
    DeterministicReplaySchedule,
    ProtectBeforeObserveShapeProbe,
    RootKindSeparationShapeProbe,
    TraversalAdmissionShapeProbe,
    ByteGuardUsageShapeProbe,
    NoHiddenLatchIoShapeProbe,
    PublicationMemoryOrderingShapeProbe,
    LeaseExpiryNonAuthorityShapeProbe,
    FreeReuseGenerationFenceShapeProbe,
    RestartDuringCutoverShapeProbe,
    ReadDuringCompactionShapeProbe,
    CompactionRangeInterlockShapeProbe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIsolationMaintenanceActorCapability {
    ReclaimBarrierParticipant,
    RestartParticipant,
    CompactionCutoverParticipant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIsolationRequiredYieldpoint {
    RootPublicationBeforeObserve,
    RootSwapPublication,
    ByteGuardAdmission,
    ReclaimBarrier,
    RestartDuringCutover,
    CompactionCutover,
    ShortcutRejectionBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIsolationProductionDriverCapability {
    ProductionBoundaryYieldpoint,
    ShortcutRejectionBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIsolationReusableOracleReadiness {
    PhysicalIsolationReadinessShape,
    TranscriptReplayEvidence,
    ForbiddenShortcutRejection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIsolationCounterContractReadiness {
    ActorStepExact,
    ReplayIdentityExact,
    ForbiddenShortcutExact,
    ProfileResourceEnvelope,
    LatchWaits,
    EpochRetries,
    ProtectedReferences,
    BlockedReclaimAttempts,
    PublicationSwaps,
    FutureS5SpecificCountersReserved,
    CompactionCandidateRanges,
    CopiedPages,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalIsolationHarnessFutureExtensionSlot {
    BlobLifecycle,
    TenantSecurity,
    RepairPitr,
    HardwareQualification,
    FullS12Campaign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationHarnessFutureExtensionReservation {
    slot: PhysicalIsolationHarnessFutureExtensionSlot,
}

impl PhysicalIsolationHarnessFutureExtensionReservation {
    pub const fn reserved(slot: PhysicalIsolationHarnessFutureExtensionSlot) -> Self {
        Self { slot }
    }

    pub const fn slot(&self) -> PhysicalIsolationHarnessFutureExtensionSlot {
        self.slot
    }
}
