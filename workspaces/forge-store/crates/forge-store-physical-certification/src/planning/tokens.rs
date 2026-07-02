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
        OracleFamilyKind::S5ReadinessShape => "s5-readiness-shape",
        OracleFamilyKind::S4RecoveryDogfood => "s4-recovery-dogfood",
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
