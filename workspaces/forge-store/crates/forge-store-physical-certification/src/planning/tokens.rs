use crate::PhysicalScenarioActorRole;

use super::requirements::{FixtureClassKind, ObserverKind, OracleFamilyKind, PhysicalDriverKind};

pub(crate) fn physical_driver_token(driver: PhysicalDriverKind) -> &'static str {
    match driver {
        PhysicalDriverKind::ProductionBoundaryYieldpoint => "production-boundary-yieldpoint",
        PhysicalDriverKind::FreshRuntimeRecovery => "fresh-runtime-recovery",
        PhysicalDriverKind::MemoryPressureBoundary => "memory-pressure-boundary",
        PhysicalDriverKind::IoPressureBoundary => "io-pressure-boundary",
        PhysicalDriverKind::OfflineVerifierBoundary => "offline-verifier-boundary",
        PhysicalDriverKind::ShortcutRejectionBoundary => "shortcut-rejection-boundary",
        PhysicalDriverKind::FutureExtensionSlot => "future-extension-slot",
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

pub(crate) fn observer_token(observer: ObserverKind) -> &'static str {
    match observer {
        ObserverKind::IndependentPhysicalTrace => "independent-physical-trace",
        ObserverKind::RecoveryOutcomeObserver => "recovery-outcome-observer",
        ObserverKind::ShortcutRejectionObserver => "shortcut-rejection-observer",
        ObserverKind::FutureExtensionSlot => "future-extension-slot",
    }
}

pub(crate) fn oracle_family_token(oracle_family: OracleFamilyKind) -> &'static str {
    match oracle_family {
        OracleFamilyKind::TranscriptReplayEvidence => "transcript-replay-evidence",
        OracleFamilyKind::PhysicalIsolationReadinessShape => "s5-readiness-shape",
        OracleFamilyKind::PhysicalIsolationInterleaving => "s5-physical-isolation-interleaving",
        OracleFamilyKind::IoPressureSimulation => "s6-io-pressure-simulation",
        OracleFamilyKind::S4RecoveryDogfood => "s4-recovery-dogfood",
        OracleFamilyKind::BlobHarnessEvidence => "s7-blob-harness-evidence",
        OracleFamilyKind::BlobHeavyQualification => "s7-blob-heavy-qualification",
        OracleFamilyKind::ForbiddenShortcutRejection => "forbidden-shortcut-rejection",
        OracleFamilyKind::FutureExtensionNonClaim => "future-extension-non-claim",
    }
}

pub(crate) fn fixture_class_token(fixture_class: FixtureClassKind) -> &'static str {
    match fixture_class {
        FixtureClassKind::AspectNativeBoundaryFact => "aspect-native-boundary-fact",
        FixtureClassKind::S4RecoveryArtifacts => "s4-recovery-artifacts",
        FixtureClassKind::FutureExtensionSlot => "future-extension-slot",
    }
}
