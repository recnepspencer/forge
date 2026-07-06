#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalSimulationScenarioFamily {
    S4RecoveryDogfood,
    S5ReadinessShapeProbe,
    S5StableReadPlanAdmission,
    S5CompactionInterlock,
    S5CheckpointPublicationInterlock,
    S5ReclaimReachability,
    S5TierMovementStability,
    S5FutureChunkStability,
    S5RestartDuringCutover,
    S6IoPressureHarness,
    S7BlobHarnessSeed,
    ShortcutRejectionDogfood,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioIntent {
    RecoveryReplayDogfood,
    ProtectBeforeObserveShape,
    StableReadPlanCounterContracts,
    StableReadPlanTranscriptReplay,
    StableReadPlanStaleGenerationMutant,
    StableReadPlanMissingReleaseMutant,
    StableReadPlanExecutionTimeDiscoveryMutant,
    StableReadPlanUnboundedFootprintMutant,
    S5CompactionEarlyReclaimMutant,
    S5CompactionStaleEpochReuseMutant,
    S5CompactionInPlaceOverwriteMutant,
    S5MixedRootReadMutant,
    S5CheckpointPublicationInterlock,
    S5ReclaimReachabilityBarrier,
    S5TierMovementStabilityOnly,
    S5FutureChunkStabilityOnly,
    S5RestartDuringCutover,
    S6IoPressureSimulation,
    S7BlobHarnessSeed,
    ForbiddenShortcutRejectionShape,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalScenarioActor {
    id: String,
    role: PhysicalScenarioActorRole,
}

impl PhysicalScenarioActor {
    pub fn foreground_reader(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::ForegroundReader)
    }

    pub fn foreground_writer(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::ForegroundWriter)
    }

    pub fn checkpoint_driver(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::CheckpointDriver)
    }

    pub fn compaction_driver(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::CompactionDriver)
    }

    pub fn maintenance_reclaimer(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::MaintenanceReclaimer)
    }

    pub fn recovery_driver(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::RecoveryDriver)
    }

    pub fn scrub_driver(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::ScrubDriver)
    }

    pub fn offline_verifier(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::OfflineVerifier)
    }

    pub fn shortcut_rejection_probe(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::ShortcutRejectionProbe)
    }

    pub fn future_extension_slot(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::FutureExtensionSlot)
    }

    fn new(id: impl Into<String>, role: PhysicalScenarioActorRole) -> Self {
        Self {
            id: id.into(),
            role,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub const fn role(&self) -> PhysicalScenarioActorRole {
        self.role
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioActorRole {
    ForegroundReader,
    ForegroundWriter,
    CheckpointDriver,
    CompactionDriver,
    MaintenanceReclaimer,
    RecoveryDriver,
    ScrubDriver,
    OfflineVerifier,
    ShortcutRejectionProbe,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalScenarioSchedule {
    production_boundary_yieldpoint: String,
}

impl PhysicalScenarioSchedule {
    pub fn named_boundary_yieldpoint(production_boundary_yieldpoint: impl Into<String>) -> Self {
        Self {
            production_boundary_yieldpoint: production_boundary_yieldpoint.into(),
        }
    }

    pub fn production_boundary_yieldpoint(&self) -> &str {
        &self.production_boundary_yieldpoint
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalScenarioFault {
    kind: PhysicalScenarioFaultKind,
}

impl PhysicalScenarioFault {
    pub const fn no_fault() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::NoFault,
        }
    }

    pub const fn future_extension_slot() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::FutureExtensionSlot,
        }
    }

    pub const fn stale_generation() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::StaleGeneration,
        }
    }

    pub const fn missing_read_plan_release() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::MissingReadPlanRelease,
        }
    }

    pub const fn execution_time_reference_discovery() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::ExecutionTimeReferenceDiscovery,
        }
    }

    pub const fn unbounded_read_plan_footprint() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::UnboundedReadPlanFootprint,
        }
    }

    pub const fn early_reclaim() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::EarlyReclaim,
        }
    }

    pub const fn stale_epoch_reuse() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::StaleEpochReuse,
        }
    }

    pub const fn in_place_compaction_overwrite() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::InPlaceCompactionOverwrite,
        }
    }

    pub const fn mixed_root_read() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::MixedRootRead,
        }
    }

    pub const fn s6_backend_latency_injection() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::S6BackendLatencyInjection,
        }
    }

    pub const fn s6_queue_depth_saturation() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::S6QueueDepthSaturation,
        }
    }

    pub const fn s6_bandwidth_throttle() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::S6BandwidthThrottle,
        }
    }

    pub const fn s6_delayed_sync() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::S6DelayedSync,
        }
    }

    pub const fn s6_page_cache_pressure() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::S6PageCachePressure,
        }
    }

    pub const fn s6_background_pacing_late_yield() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::S6BackgroundPacingLateYield,
        }
    }

    pub const fn kind(&self) -> PhysicalScenarioFaultKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioFaultKind {
    NoFault,
    StaleGeneration,
    MissingReadPlanRelease,
    ExecutionTimeReferenceDiscovery,
    UnboundedReadPlanFootprint,
    EarlyReclaim,
    StaleEpochReuse,
    InPlaceCompactionOverwrite,
    MixedRootRead,
    S6BackendLatencyInjection,
    S6QueueDepthSaturation,
    S6BandwidthThrottle,
    S6DelayedSync,
    S6PageCachePressure,
    S6BackgroundPacingLateYield,
    FutureExtensionSlot,
}
