use super::expectation::{PhysicalScenarioExpectationKind, PhysicalScenarioNonClaim};
use super::vocabulary::{
    PhysicalScenarioActorRole, PhysicalScenarioFaultKind, PhysicalScenarioIntent,
    PhysicalSimulationScenarioFamily,
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
