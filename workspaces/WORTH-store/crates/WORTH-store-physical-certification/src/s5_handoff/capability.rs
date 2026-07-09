#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S5InterleavingHarnessCapability {
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
pub enum S5MaintenanceActorCapability {
    ReclaimBarrierParticipant,
    RestartParticipant,
    CompactionCutoverParticipant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S5RequiredYieldpoint {
    RootPublicationBeforeObserve,
    RootSwapPublication,
    ByteGuardAdmission,
    ReclaimBarrier,
    RestartDuringCutover,
    CompactionCutover,
    ShortcutRejectionBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S5ProductionDriverCapability {
    ProductionBoundaryYieldpoint,
    ShortcutRejectionBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S5ReusableOracleReadiness {
    S5ReadinessShape,
    TranscriptReplayEvidence,
    ForbiddenShortcutRejection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S5CounterContractReadiness {
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
pub enum S5HarnessFutureExtensionSlot {
    BlobLifecycle,
    TenantSecurity,
    RepairPitr,
    HardwareQualification,
    FullS12Campaign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5HarnessFutureExtensionReservation {
    slot: S5HarnessFutureExtensionSlot,
}

impl S5HarnessFutureExtensionReservation {
    pub const fn reserved(slot: S5HarnessFutureExtensionSlot) -> Self {
        Self { slot }
    }

    pub const fn slot(&self) -> S5HarnessFutureExtensionSlot {
        self.slot
    }
}
