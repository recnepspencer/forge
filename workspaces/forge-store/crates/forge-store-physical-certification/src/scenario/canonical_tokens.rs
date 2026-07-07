use super::expectation::{PhysicalScenarioExpectationKind, PhysicalScenarioNonClaim};
use super::vocabulary::{
    PhysicalScenarioActorRole, PhysicalScenarioFaultKind, PhysicalScenarioIntent,
    PhysicalSimulationScenarioFamily,
};
use forge_store_blob_chunks::{
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass, BlobHarnessFailurePoint,
    BlobHarnessPlacementClass, BlobHarnessSecurityScopeClass, BlobHarnessSizeClass,
};

pub(crate) fn family_token(family: PhysicalSimulationScenarioFamily) -> &'static str {
    match family {
        PhysicalSimulationScenarioFamily::S4RecoveryDogfood => "s4-recovery-dogfood",
        PhysicalSimulationScenarioFamily::S5ReadinessShapeProbe => "s5-readiness-shape-probe",
        PhysicalSimulationScenarioFamily::S5StableReadPlanAdmission => {
            "s5-stable-read-plan-admission"
        }
        PhysicalSimulationScenarioFamily::S5CompactionInterlock => "s5-compaction-interlock",
        PhysicalSimulationScenarioFamily::S5CheckpointPublicationInterlock => {
            "s5-checkpoint-publication-interlock"
        }
        PhysicalSimulationScenarioFamily::S5ReclaimReachability => "s5-reclaim-reachability",
        PhysicalSimulationScenarioFamily::S5TierMovementStability => "s5-tier-movement-stability",
        PhysicalSimulationScenarioFamily::S5FutureChunkStability => "s5-future-chunk-stability",
        PhysicalSimulationScenarioFamily::S5RestartDuringCutover => "s5-restart-during-cutover",
        PhysicalSimulationScenarioFamily::S6IoPressureHarness => "s6-io-pressure-harness",
        PhysicalSimulationScenarioFamily::S7BlobHarnessSeed => "s7-blob-harness-seed",
        PhysicalSimulationScenarioFamily::ShortcutRejectionDogfood => "shortcut-rejection-dogfood",
        PhysicalSimulationScenarioFamily::FutureExtensionSlot => "future-extension-slot",
    }
}

pub(crate) fn intent_token(intent: PhysicalScenarioIntent) -> &'static str {
    match intent {
        PhysicalScenarioIntent::RecoveryReplayDogfood => "recovery-replay-dogfood",
        PhysicalScenarioIntent::ProtectBeforeObserveShape => "protect-before-observe-shape",
        PhysicalScenarioIntent::StableReadPlanCounterContracts => {
            "stable-read-plan-counter-contracts"
        }
        PhysicalScenarioIntent::StableReadPlanTranscriptReplay => {
            "stable-read-plan-transcript-replay"
        }
        PhysicalScenarioIntent::StableReadPlanStaleGenerationMutant => {
            "stable-read-plan-stale-generation-mutant"
        }
        PhysicalScenarioIntent::StableReadPlanMissingReleaseMutant => {
            "stable-read-plan-missing-release-mutant"
        }
        PhysicalScenarioIntent::StableReadPlanExecutionTimeDiscoveryMutant => {
            "stable-read-plan-execution-time-discovery-mutant"
        }
        PhysicalScenarioIntent::StableReadPlanUnboundedFootprintMutant => {
            "stable-read-plan-unbounded-footprint-mutant"
        }
        PhysicalScenarioIntent::S5CompactionEarlyReclaimMutant => {
            "s5-compaction-early-reclaim-mutant"
        }
        PhysicalScenarioIntent::S5CompactionStaleEpochReuseMutant => {
            "s5-compaction-stale-epoch-reuse-mutant"
        }
        PhysicalScenarioIntent::S5CompactionInPlaceOverwriteMutant => {
            "s5-compaction-in-place-overwrite-mutant"
        }
        PhysicalScenarioIntent::S5MixedRootReadMutant => "s5-mixed-root-read-mutant",
        PhysicalScenarioIntent::S5CheckpointPublicationInterlock => {
            "s5-checkpoint-publication-interlock"
        }
        PhysicalScenarioIntent::S5ReclaimReachabilityBarrier => "s5-reclaim-reachability-barrier",
        PhysicalScenarioIntent::S5TierMovementStabilityOnly => "s5-tier-movement-stability-only",
        PhysicalScenarioIntent::S5FutureChunkStabilityOnly => "s5-future-chunk-stability-only",
        PhysicalScenarioIntent::S5RestartDuringCutover => "s5-restart-during-cutover",
        PhysicalScenarioIntent::S6IoPressureSimulation => "s6-io-pressure-simulation",
        PhysicalScenarioIntent::S7BlobHarnessSeed => "s7-blob-harness-seed",
        PhysicalScenarioIntent::ForbiddenShortcutRejectionShape => {
            "forbidden-shortcut-rejection-shape"
        }
        PhysicalScenarioIntent::FutureExtensionSlot => "future-extension-slot",
    }
}

pub(crate) fn actor_role_token(role: PhysicalScenarioActorRole) -> &'static str {
    match role {
        PhysicalScenarioActorRole::ForegroundReader => "foreground-reader",
        PhysicalScenarioActorRole::ForegroundWriter => "foreground-writer",
        PhysicalScenarioActorRole::CheckpointDriver => "checkpoint-driver",
        PhysicalScenarioActorRole::CompactionDriver => "compaction-driver",
        PhysicalScenarioActorRole::MaintenanceReclaimer => "maintenance-reclaimer",
        PhysicalScenarioActorRole::RecoveryDriver => "recovery-driver",
        PhysicalScenarioActorRole::ScrubDriver => "scrub-driver",
        PhysicalScenarioActorRole::OfflineVerifier => "offline-verifier",
        PhysicalScenarioActorRole::ShortcutRejectionProbe => "shortcut-rejection-probe",
        PhysicalScenarioActorRole::BlobIngestActor => "blob-ingest-actor",
        PhysicalScenarioActorRole::BlobReadActor => "blob-read-actor",
        PhysicalScenarioActorRole::BlobVerifyActor => "blob-verify-actor",
        PhysicalScenarioActorRole::BlobResumeActor => "blob-resume-actor",
        PhysicalScenarioActorRole::BlobDedupeActor => "blob-dedupe-actor",
        PhysicalScenarioActorRole::BlobExportActor => "blob-export-actor",
        PhysicalScenarioActorRole::BlobImportActor => "blob-import-actor",
        PhysicalScenarioActorRole::BlobPlacementMoveActor => "blob-placement-move-actor",
        PhysicalScenarioActorRole::BlobPartialReplicationActor => "blob-partial-replication-actor",
        PhysicalScenarioActorRole::BlobReclaimActor => "blob-reclaim-actor",
        PhysicalScenarioActorRole::FutureExtensionSlot => "future-extension-slot",
    }
}

pub(crate) fn fault_token(fault: PhysicalScenarioFaultKind) -> &'static str {
    match fault {
        PhysicalScenarioFaultKind::NoFault => "no-fault",
        PhysicalScenarioFaultKind::StaleGeneration => "stale-generation",
        PhysicalScenarioFaultKind::MissingReadPlanRelease => "missing-read-plan-release",
        PhysicalScenarioFaultKind::ExecutionTimeReferenceDiscovery => {
            "execution-time-reference-discovery"
        }
        PhysicalScenarioFaultKind::UnboundedReadPlanFootprint => "unbounded-read-plan-footprint",
        PhysicalScenarioFaultKind::EarlyReclaim => "early-reclaim",
        PhysicalScenarioFaultKind::StaleEpochReuse => "stale-epoch-reuse",
        PhysicalScenarioFaultKind::InPlaceCompactionOverwrite => "in-place-compaction-overwrite",
        PhysicalScenarioFaultKind::MixedRootRead => "mixed-root-read",
        PhysicalScenarioFaultKind::S6BackendLatencyInjection => "s6-backend-latency-injection",
        PhysicalScenarioFaultKind::S6QueueDepthSaturation => "s6-queue-depth-saturation",
        PhysicalScenarioFaultKind::S6BandwidthThrottle => "s6-bandwidth-throttle",
        PhysicalScenarioFaultKind::S6DelayedSync => "s6-delayed-sync",
        PhysicalScenarioFaultKind::S6PageCachePressure => "s6-page-cache-pressure",
        PhysicalScenarioFaultKind::S6BackgroundPacingLateYield => "s6-background-pacing-late-yield",
        PhysicalScenarioFaultKind::BlobCrashAfterChunkWrite => "blob-crash-after-chunk-write",
        PhysicalScenarioFaultKind::BlobCrashAfterSessionCheckpoint => {
            "blob-crash-after-session-checkpoint"
        }
        PhysicalScenarioFaultKind::BlobCrashAfterRootPublication => {
            "blob-crash-after-root-publication"
        }
        PhysicalScenarioFaultKind::BlobTierMoveInterruption => "blob-tier-move-interruption",
        PhysicalScenarioFaultKind::BlobExportInterruption => "blob-export-interruption",
        PhysicalScenarioFaultKind::BlobReclaimInterruption => "blob-reclaim-interruption",
        PhysicalScenarioFaultKind::FutureExtensionSlot => "future-extension-slot",
    }
}

pub(crate) fn expectation_token(expectation: PhysicalScenarioExpectationKind) -> &'static str {
    match expectation {
        PhysicalScenarioExpectationKind::S4RecoveryDogfood => "s4-recovery-dogfood",
        PhysicalScenarioExpectationKind::S5ReadinessShapeProbe => "s5-readiness-shape-probe",
        PhysicalScenarioExpectationKind::S5ReadinessWithShortcutRejectionProbe => {
            "s5-readiness-with-shortcut-rejection-probe"
        }
        PhysicalScenarioExpectationKind::S5CheckpointPublicationCrashReplay => {
            "s5-checkpoint-publication-crash-replay"
        }
        PhysicalScenarioExpectationKind::StableReadPlanCounterContracts => {
            "stable-read-plan-counter-contracts"
        }
        PhysicalScenarioExpectationKind::StableReadPlanTranscriptReplay => {
            "stable-read-plan-transcript-replay"
        }
        PhysicalScenarioExpectationKind::StableReadPlanDenial => "stable-read-plan-denial",
        PhysicalScenarioExpectationKind::S5PhysicalIsolationInterleaving => {
            "s5-physical-isolation-interleaving"
        }
        PhysicalScenarioExpectationKind::S5PhysicalIsolationDenial => {
            "s5-physical-isolation-denial"
        }
        PhysicalScenarioExpectationKind::S6IoPressureSimulation => "s6-io-pressure-simulation",
        PhysicalScenarioExpectationKind::S7BlobHarnessSeed => "s7-blob-harness-seed",
        PhysicalScenarioExpectationKind::ShortcutRejectionDogfood => "shortcut-rejection-dogfood",
        PhysicalScenarioExpectationKind::FutureExtensionSlot => "future-extension-slot",
    }
}

pub(crate) fn non_claim_token(non_claim: PhysicalScenarioNonClaim) -> &'static str {
    match non_claim {
        PhysicalScenarioNonClaim::NoS5PhysicalIsolationCorrectnessClaim => {
            "no-s5-physical-isolation-correctness-claim"
        }
        PhysicalScenarioNonClaim::NoRealBackendSafetyQualification => {
            "no-real-backend-safety-qualification"
        }
        PhysicalScenarioNonClaim::NoS7BlobOperationCorrectnessClaim => {
            "no-s7-blob-operation-correctness-claim"
        }
        PhysicalScenarioNonClaim::FutureExtensionSlotDoesNotImplementFutureBehavior => {
            "future-extension-slot-does-not-implement-future-behavior"
        }
    }
}

pub(crate) fn blob_harness_size_class_token(size_class: BlobHarnessSizeClass) -> &'static str {
    match size_class {
        BlobHarnessSizeClass::TinyShortcut => "tiny-shortcut",
        BlobHarnessSizeClass::LocalDeterministic => "local-deterministic",
        BlobHarnessSizeClass::MemoryEnvelopeExceeding => "memory-envelope-exceeding",
        BlobHarnessSizeClass::HeavyMultiGbDeclared => "heavy-multi-gb-declared",
    }
}

pub(crate) fn blob_harness_chunk_size_class_token(
    chunk_size_class: BlobHarnessChunkSizeClass,
) -> &'static str {
    match chunk_size_class {
        BlobHarnessChunkSizeClass::Fixed64KiB => "fixed-64-kib",
        BlobHarnessChunkSizeClass::Fixed1MiB => "fixed-1-mib",
        BlobHarnessChunkSizeClass::Fixed8MiB => "fixed-8-mib",
    }
}

pub(crate) fn blob_harness_placement_class_token(
    placement_class: BlobHarnessPlacementClass,
) -> &'static str {
    match placement_class {
        BlobHarnessPlacementClass::StoreLocal => "store-local",
        BlobHarnessPlacementClass::ExternalPlacementObserved => "external-placement-observed",
        BlobHarnessPlacementClass::ColdTierObserved => "cold-tier-observed",
    }
}

pub(crate) fn blob_harness_security_scope_class_token(
    security_scope_class: BlobHarnessSecurityScopeClass,
) -> &'static str {
    match security_scope_class {
        BlobHarnessSecurityScopeClass::ScopePreserving => "scope-preserving",
        BlobHarnessSecurityScopeClass::CrossScopeDenied => "cross-scope-denied",
    }
}

pub(crate) fn blob_harness_access_mode_token(access_mode: BlobHarnessAccessMode) -> &'static str {
    match access_mode {
        BlobHarnessAccessMode::ReadOnlyReplay => "read-only-replay",
        BlobHarnessAccessMode::ResumableIngestSeed => "resumable-ingest-seed",
        BlobHarnessAccessMode::ExportBoundary => "export-boundary",
        BlobHarnessAccessMode::ImportReadmission => "import-readmission",
        BlobHarnessAccessMode::PartialReplication => "partial-replication",
    }
}

pub(crate) fn blob_harness_failure_point_token(
    failure_point: BlobHarnessFailurePoint,
) -> &'static str {
    match failure_point {
        BlobHarnessFailurePoint::NoFaultSeed => "no-fault-seed",
        BlobHarnessFailurePoint::AfterChunkWrite => "after-chunk-write",
        BlobHarnessFailurePoint::AfterSessionCheckpoint => "after-session-checkpoint",
        BlobHarnessFailurePoint::AfterRootPublication => "after-root-publication",
        BlobHarnessFailurePoint::DuringTierMove => "during-tier-move",
        BlobHarnessFailurePoint::DuringExport => "during-export",
        BlobHarnessFailurePoint::DuringReclaim => "during-reclaim",
    }
}

pub(crate) fn blob_harness_actor_mix_token(actor_mix: BlobHarnessActorMix) -> &'static str {
    match actor_mix {
        BlobHarnessActorMix::SeedReplayOnly => "seed-replay-only",
        BlobHarnessActorMix::IngestReadVerify => "ingest-read-verify",
        BlobHarnessActorMix::ResumeRecovery => "resume-recovery",
        BlobHarnessActorMix::DedupeReclaim => "dedupe-reclaim",
        BlobHarnessActorMix::ExportImport => "export-import",
        BlobHarnessActorMix::PlacementMovePartialReplication => {
            "placement-move-partial-replication"
        }
    }
}
