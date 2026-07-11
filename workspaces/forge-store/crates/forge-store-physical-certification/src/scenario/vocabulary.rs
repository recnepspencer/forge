#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalSimulationScenarioFamily {
    S4RecoveryDogfood,
    PhysicalIsolationReadinessShapeProbe,
    PhysicalIsolationStableReadPlanAdmission,
    PhysicalIsolationCompactionInterlock,
    PhysicalIsolationCheckpointPublicationInterlock,
    PhysicalIsolationReclaimReachability,
    PhysicalIsolationTierMovementStability,
    PhysicalIsolationFutureChunkStability,
    PhysicalIsolationRestartDuringCutover,
    IoPressureHarness,
    BlobHarnessSeed,
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
    PhysicalIsolationCompactionEarlyReclaimMutant,
    PhysicalIsolationCompactionStaleEpochReuseMutant,
    PhysicalIsolationCompactionInPlaceOverwriteMutant,
    MixedRootReadMutant,
    PhysicalIsolationCheckpointPublicationInterlock,
    PhysicalIsolationReclaimReachabilityBarrier,
    PhysicalIsolationTierMovementStabilityOnly,
    PhysicalIsolationFutureChunkStabilityOnly,
    PhysicalIsolationRestartDuringCutover,
    IoPressureSimulation,
    BlobHarnessSeed,
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

    pub fn blob_ingest_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobIngestActor)
    }

    pub fn blob_read_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobReadActor)
    }

    pub fn blob_verify_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobVerifyActor)
    }

    pub fn blob_resume_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobResumeActor)
    }

    pub fn blob_dedupe_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobDedupeActor)
    }

    pub fn blob_export_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobExportActor)
    }

    pub fn blob_import_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobImportActor)
    }

    pub fn blob_placement_move_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobPlacementMoveActor)
    }

    pub fn blob_partial_replication_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobPartialReplicationActor)
    }

    pub fn blob_reclaim_actor(id: impl Into<String>) -> Self {
        Self::new(id, PhysicalScenarioActorRole::BlobReclaimActor)
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
    BlobIngestActor,
    BlobReadActor,
    BlobVerifyActor,
    BlobResumeActor,
    BlobDedupeActor,
    BlobExportActor,
    BlobImportActor,
    BlobPlacementMoveActor,
    BlobPartialReplicationActor,
    BlobReclaimActor,
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

    pub const fn io_pressure_backend_latency_injection() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::IoPressureBackendLatencyInjection,
        }
    }

    pub const fn io_pressure_queue_depth_saturation() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::IoPressureQueueDepthSaturation,
        }
    }

    pub const fn io_pressure_bandwidth_throttle() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::IoPressureBandwidthThrottle,
        }
    }

    pub const fn io_pressure_delayed_sync() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::IoPressureDelayedSync,
        }
    }

    pub const fn io_pressure_page_cache_pressure() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::IoPressurePageCachePressure,
        }
    }

    pub const fn io_pressure_background_pacing_late_yield() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::IoPressureBackgroundPacingLateYield,
        }
    }

    pub const fn blob_crash_after_chunk_write() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::BlobCrashAfterChunkWrite,
        }
    }

    pub const fn blob_crash_after_session_checkpoint() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::BlobCrashAfterSessionCheckpoint,
        }
    }

    pub const fn blob_crash_after_root_publication() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::BlobCrashAfterRootPublication,
        }
    }

    pub const fn blob_tier_move_interruption() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::BlobTierMoveInterruption,
        }
    }

    pub const fn blob_export_interruption() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::BlobExportInterruption,
        }
    }

    pub const fn blob_reclaim_interruption() -> Self {
        Self {
            kind: PhysicalScenarioFaultKind::BlobReclaimInterruption,
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
    IoPressureBackendLatencyInjection,
    IoPressureQueueDepthSaturation,
    IoPressureBandwidthThrottle,
    IoPressureDelayedSync,
    IoPressurePageCachePressure,
    IoPressureBackgroundPacingLateYield,
    BlobCrashAfterChunkWrite,
    BlobCrashAfterSessionCheckpoint,
    BlobCrashAfterRootPublication,
    BlobTierMoveInterruption,
    BlobExportInterruption,
    BlobReclaimInterruption,
    FutureExtensionSlot,
}
