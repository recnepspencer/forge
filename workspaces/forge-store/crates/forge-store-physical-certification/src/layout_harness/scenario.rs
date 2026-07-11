use super::actors::S8LayoutActorLane;
use super::closeout::S8LayoutCloseoutEvidenceLane;
use super::coverage::S8LayoutCoverageRowKind;
use super::faults::S8LayoutFaultLane;
use super::observers::S8LayoutObserverLane;
use super::oracles::S8LayoutOracleLane;
use super::shortcut_denials::S8LayoutShortcutDenialKind;
use super::transcripts::S8LayoutTranscriptKind;
mod queries;

pub use queries::{
    all_layout_index_layout_scenarios, canonical_layout_index_layout_production_apis,
    canonical_layout_index_layout_required_transitions, canonical_layout_index_layout_shortcut_denials,
    canonical_layout_index_layout_supported_scenarios,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutProductionApi {
    LayoutFamilies,
    LayoutStrategyAdmission,
    AccessPlanning,
    AccessLowering,
    AccessExecution,
    LayoutRebuild,
    LayoutMigration,
    LayoutReadmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutTransitionState {
    Declared,
    Admitted,
    Planned,
    Lowered,
    ExecutionReady,
    Executed,
    Rebuilt,
    Rebound,
    Readmitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutScenarioKind {
    LayoutDeclarationInventory,
    AccessShapeDenial,
    BroadScanRejection,
    ExactCounter,
    CorruptionRebuildParity,
    MigrationRollbackInterruption,
    TrustBoundaryReadmission,
    MultiArtifactIntegration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8LayoutScenarioDefinition {
    kind: S8LayoutScenarioKind,
    production_apis: &'static [S8LayoutProductionApi],
    actors: &'static [S8LayoutActorLane],
    faults: &'static [S8LayoutFaultLane],
    observers: &'static [S8LayoutObserverLane],
    oracles: &'static [S8LayoutOracleLane],
    coverage: &'static [S8LayoutCoverageRowKind],
    shortcut_denials: &'static [S8LayoutShortcutDenialKind],
    transitions: &'static [S8LayoutTransitionState],
    transcript: S8LayoutTranscriptKind,
    closeout: S8LayoutCloseoutEvidenceLane,
}

const DECLARATION_APIS: &[S8LayoutProductionApi] = &[
    S8LayoutProductionApi::LayoutFamilies,
    S8LayoutProductionApi::LayoutStrategyAdmission,
];
const ACCESS_APIS: &[S8LayoutProductionApi] = &[
    S8LayoutProductionApi::AccessPlanning,
    S8LayoutProductionApi::AccessLowering,
];
const EXECUTION_APIS: &[S8LayoutProductionApi] = &[
    S8LayoutProductionApi::AccessPlanning,
    S8LayoutProductionApi::AccessLowering,
    S8LayoutProductionApi::AccessExecution,
];
const REBUILD_APIS: &[S8LayoutProductionApi] = &[
    S8LayoutProductionApi::LayoutRebuild,
    S8LayoutProductionApi::LayoutReadmission,
];
const MIGRATION_APIS: &[S8LayoutProductionApi] = &[
    S8LayoutProductionApi::LayoutMigration,
    S8LayoutProductionApi::LayoutReadmission,
];
const READMISSION_APIS: &[S8LayoutProductionApi] = &[S8LayoutProductionApi::LayoutReadmission];
const INTEGRATION_APIS: &[S8LayoutProductionApi] = &[
    S8LayoutProductionApi::LayoutFamilies,
    S8LayoutProductionApi::LayoutStrategyAdmission,
    S8LayoutProductionApi::AccessPlanning,
    S8LayoutProductionApi::AccessLowering,
    S8LayoutProductionApi::AccessExecution,
    S8LayoutProductionApi::LayoutRebuild,
    S8LayoutProductionApi::LayoutMigration,
    S8LayoutProductionApi::LayoutReadmission,
];

const DECLARATION_ACTORS: &[S8LayoutActorLane] = &[
    S8LayoutActorLane::DeclarationCatalog,
    S8LayoutActorLane::OfflineVerifier,
];
const ACCESS_ACTORS: &[S8LayoutActorLane] = &[S8LayoutActorLane::ForegroundAccess];
const RECOVERY_ACTORS: &[S8LayoutActorLane] =
    &[S8LayoutActorLane::Recovery, S8LayoutActorLane::Maintenance];
const MIGRATION_ACTORS: &[S8LayoutActorLane] =
    &[S8LayoutActorLane::Migration, S8LayoutActorLane::Recovery];
const READMISSION_ACTORS: &[S8LayoutActorLane] = &[
    S8LayoutActorLane::OfflineVerifier,
    S8LayoutActorLane::Recovery,
];
const INTEGRATION_ACTORS: &[S8LayoutActorLane] = &[
    S8LayoutActorLane::DeclarationCatalog,
    S8LayoutActorLane::ForegroundAccess,
    S8LayoutActorLane::Recovery,
    S8LayoutActorLane::Migration,
    S8LayoutActorLane::Maintenance,
    S8LayoutActorLane::OfflineVerifier,
];

const NO_FAULTS: &[S8LayoutFaultLane] = &[S8LayoutFaultLane::NoFaultControl];
const REBUILD_FAULTS: &[S8LayoutFaultLane] = &[
    S8LayoutFaultLane::ByteCorruption,
    S8LayoutFaultLane::StaleGeneration,
];
const MIGRATION_FAULTS: &[S8LayoutFaultLane] = &[
    S8LayoutFaultLane::CrashInterruption,
    S8LayoutFaultLane::ReorderedPersistence,
];
const READMISSION_FAULTS: &[S8LayoutFaultLane] = &[
    S8LayoutFaultLane::TerminalProjectionShortcut,
    S8LayoutFaultLane::StaleGeneration,
];
const INTEGRATION_FAULTS: &[S8LayoutFaultLane] = &[
    S8LayoutFaultLane::CrashInterruption,
    S8LayoutFaultLane::ByteCorruption,
    S8LayoutFaultLane::ReorderedPersistence,
];

const DECLARATION_OBSERVERS: &[S8LayoutObserverLane] = &[
    S8LayoutObserverLane::DeclarationInventoryObserver,
    S8LayoutObserverLane::OfflineVerifierObserver,
];
const EXECUTION_OBSERVERS: &[S8LayoutObserverLane] = &[
    S8LayoutObserverLane::CounterReceiptObserver,
    S8LayoutObserverLane::MultiArtifactTraceObserver,
];
const RECOVERY_OBSERVERS: &[S8LayoutObserverLane] = &[
    S8LayoutObserverLane::RecoveryOutcomeObserver,
    S8LayoutObserverLane::OfflineVerifierObserver,
];
const READMISSION_OBSERVERS: &[S8LayoutObserverLane] = &[
    S8LayoutObserverLane::ReadmissionObserver,
    S8LayoutObserverLane::OfflineVerifierObserver,
];
const INTEGRATION_OBSERVERS: &[S8LayoutObserverLane] = &[
    S8LayoutObserverLane::CounterReceiptObserver,
    S8LayoutObserverLane::RecoveryOutcomeObserver,
    S8LayoutObserverLane::MultiArtifactTraceObserver,
];

const DECLARATION_ORACLES: &[S8LayoutOracleLane] =
    &[S8LayoutOracleLane::DeclarationInventoryOracle];
const ACCESS_ORACLES: &[S8LayoutOracleLane] = &[
    S8LayoutOracleLane::AccessShapeDenialOracle,
    S8LayoutOracleLane::BroadScanRejectionOracle,
];
const COUNTER_ORACLES: &[S8LayoutOracleLane] = &[S8LayoutOracleLane::ExactCounterOracle];
const REBUILD_ORACLES: &[S8LayoutOracleLane] = &[S8LayoutOracleLane::RebuildParityOracle];
const MIGRATION_ORACLES: &[S8LayoutOracleLane] = &[S8LayoutOracleLane::MigrationRollbackOracle];
const READMISSION_ORACLES: &[S8LayoutOracleLane] = &[S8LayoutOracleLane::ReadmissionBoundaryOracle];
const INTEGRATION_ORACLES: &[S8LayoutOracleLane] =
    &[S8LayoutOracleLane::MultiArtifactIntegrationOracle];

const DECLARATION_COVERAGE: &[S8LayoutCoverageRowKind] =
    &[S8LayoutCoverageRowKind::DeclarationInventory];
const ACCESS_SHAPE_COVERAGE: &[S8LayoutCoverageRowKind] =
    &[S8LayoutCoverageRowKind::AccessShapeDenial];
const BROAD_SCAN_COVERAGE: &[S8LayoutCoverageRowKind] =
    &[S8LayoutCoverageRowKind::BroadScanRejection];
const COUNTER_COVERAGE: &[S8LayoutCoverageRowKind] = &[S8LayoutCoverageRowKind::ExactCounter];
const REBUILD_COVERAGE: &[S8LayoutCoverageRowKind] = &[S8LayoutCoverageRowKind::RebuildParity];
const MIGRATION_COVERAGE: &[S8LayoutCoverageRowKind] =
    &[S8LayoutCoverageRowKind::MigrationRollback];
const READMISSION_COVERAGE: &[S8LayoutCoverageRowKind] =
    &[S8LayoutCoverageRowKind::ReadmissionBoundary];
const INTEGRATION_COVERAGE: &[S8LayoutCoverageRowKind] =
    &[S8LayoutCoverageRowKind::MultiArtifactIntegration];

const ACCESS_SHORTCUTS: &[S8LayoutShortcutDenialKind] = &[
    S8LayoutShortcutDenialKind::BroadScanMasqueradingAsPointLookup,
    S8LayoutShortcutDenialKind::CopiedCounterRows,
];
const READMISSION_SHORTCUTS: &[S8LayoutShortcutDenialKind] = &[
    S8LayoutShortcutDenialKind::TerminalProjectionAuthority,
    S8LayoutShortcutDenialKind::FoundationalMaterializationAuthority,
];
const FIXTURE_SHORTCUTS: &[S8LayoutShortcutDenialKind] =
    &[S8LayoutShortcutDenialKind::SyntheticFixtureAuthority];
const INTEGRATION_SHORTCUTS: &[S8LayoutShortcutDenialKind] = &[
    S8LayoutShortcutDenialKind::CopiedCounterRows,
    S8LayoutShortcutDenialKind::LooseLogEvidence,
    S8LayoutShortcutDenialKind::SyntheticFixtureAuthority,
];

const DECLARATION_TRANSITIONS: &[S8LayoutTransitionState] = &[
    S8LayoutTransitionState::Declared,
    S8LayoutTransitionState::Admitted,
];
const ACCESS_TRANSITIONS: &[S8LayoutTransitionState] = &[
    S8LayoutTransitionState::Admitted,
    S8LayoutTransitionState::Planned,
    S8LayoutTransitionState::Lowered,
];
const EXECUTION_TRANSITIONS: &[S8LayoutTransitionState] = &[
    S8LayoutTransitionState::Admitted,
    S8LayoutTransitionState::Planned,
    S8LayoutTransitionState::Lowered,
    S8LayoutTransitionState::ExecutionReady,
    S8LayoutTransitionState::Executed,
];
const REBUILD_TRANSITIONS: &[S8LayoutTransitionState] = &[
    S8LayoutTransitionState::Executed,
    S8LayoutTransitionState::Rebuilt,
    S8LayoutTransitionState::Readmitted,
];
const MIGRATION_TRANSITIONS: &[S8LayoutTransitionState] = &[
    S8LayoutTransitionState::Declared,
    S8LayoutTransitionState::Admitted,
    S8LayoutTransitionState::Rebound,
    S8LayoutTransitionState::Readmitted,
];
const READMISSION_TRANSITIONS: &[S8LayoutTransitionState] = &[
    S8LayoutTransitionState::Executed,
    S8LayoutTransitionState::Readmitted,
];
const INTEGRATION_TRANSITIONS: &[S8LayoutTransitionState] = &[
    S8LayoutTransitionState::Declared,
    S8LayoutTransitionState::Admitted,
    S8LayoutTransitionState::Planned,
    S8LayoutTransitionState::Lowered,
    S8LayoutTransitionState::ExecutionReady,
    S8LayoutTransitionState::Executed,
    S8LayoutTransitionState::Rebuilt,
    S8LayoutTransitionState::Rebound,
    S8LayoutTransitionState::Readmitted,
];

pub fn layout_scenario(kind: S8LayoutScenarioKind) -> S8LayoutScenarioDefinition {
    match kind {
        S8LayoutScenarioKind::LayoutDeclarationInventory => S8LayoutScenarioDefinition {
            kind,
            production_apis: DECLARATION_APIS,
            actors: DECLARATION_ACTORS,
            faults: NO_FAULTS,
            observers: DECLARATION_OBSERVERS,
            oracles: DECLARATION_ORACLES,
            coverage: DECLARATION_COVERAGE,
            shortcut_denials: FIXTURE_SHORTCUTS,
            transitions: DECLARATION_TRANSITIONS,
            transcript: S8LayoutTranscriptKind::ScenarioTranscript,
            closeout: S8LayoutCloseoutEvidenceLane::ScenarioDefinition,
        },
        S8LayoutScenarioKind::AccessShapeDenial => S8LayoutScenarioDefinition {
            kind,
            production_apis: ACCESS_APIS,
            actors: ACCESS_ACTORS,
            faults: NO_FAULTS,
            observers: EXECUTION_OBSERVERS,
            oracles: ACCESS_ORACLES,
            coverage: ACCESS_SHAPE_COVERAGE,
            shortcut_denials: ACCESS_SHORTCUTS,
            transitions: ACCESS_TRANSITIONS,
            transcript: S8LayoutTranscriptKind::ShortcutDenialTrace,
            closeout: S8LayoutCloseoutEvidenceLane::ScenarioDefinition,
        },
        S8LayoutScenarioKind::BroadScanRejection => S8LayoutScenarioDefinition {
            kind,
            production_apis: ACCESS_APIS,
            actors: ACCESS_ACTORS,
            faults: NO_FAULTS,
            observers: EXECUTION_OBSERVERS,
            oracles: ACCESS_ORACLES,
            coverage: BROAD_SCAN_COVERAGE,
            shortcut_denials: ACCESS_SHORTCUTS,
            transitions: ACCESS_TRANSITIONS,
            transcript: S8LayoutTranscriptKind::ShortcutDenialTrace,
            closeout: S8LayoutCloseoutEvidenceLane::PerformanceEvidence,
        },
        S8LayoutScenarioKind::ExactCounter => S8LayoutScenarioDefinition {
            kind,
            production_apis: EXECUTION_APIS,
            actors: ACCESS_ACTORS,
            faults: NO_FAULTS,
            observers: EXECUTION_OBSERVERS,
            oracles: COUNTER_ORACLES,
            coverage: COUNTER_COVERAGE,
            shortcut_denials: ACCESS_SHORTCUTS,
            transitions: EXECUTION_TRANSITIONS,
            transcript: S8LayoutTranscriptKind::ScenarioTranscript,
            closeout: S8LayoutCloseoutEvidenceLane::PerformanceEvidence,
        },
        S8LayoutScenarioKind::CorruptionRebuildParity => S8LayoutScenarioDefinition {
            kind,
            production_apis: REBUILD_APIS,
            actors: RECOVERY_ACTORS,
            faults: REBUILD_FAULTS,
            observers: RECOVERY_OBSERVERS,
            oracles: REBUILD_ORACLES,
            coverage: REBUILD_COVERAGE,
            shortcut_denials: READMISSION_SHORTCUTS,
            transitions: REBUILD_TRANSITIONS,
            transcript: S8LayoutTranscriptKind::ReplayBundle,
            closeout: S8LayoutCloseoutEvidenceLane::CertificationCloseout,
        },
        S8LayoutScenarioKind::MigrationRollbackInterruption => S8LayoutScenarioDefinition {
            kind,
            production_apis: MIGRATION_APIS,
            actors: MIGRATION_ACTORS,
            faults: MIGRATION_FAULTS,
            observers: RECOVERY_OBSERVERS,
            oracles: MIGRATION_ORACLES,
            coverage: MIGRATION_COVERAGE,
            shortcut_denials: FIXTURE_SHORTCUTS,
            transitions: MIGRATION_TRANSITIONS,
            transcript: S8LayoutTranscriptKind::ReplayBundle,
            closeout: S8LayoutCloseoutEvidenceLane::CertificationCloseout,
        },
        S8LayoutScenarioKind::TrustBoundaryReadmission => S8LayoutScenarioDefinition {
            kind,
            production_apis: READMISSION_APIS,
            actors: READMISSION_ACTORS,
            faults: READMISSION_FAULTS,
            observers: READMISSION_OBSERVERS,
            oracles: READMISSION_ORACLES,
            coverage: READMISSION_COVERAGE,
            shortcut_denials: READMISSION_SHORTCUTS,
            transitions: READMISSION_TRANSITIONS,
            transcript: S8LayoutTranscriptKind::ShortcutDenialTrace,
            closeout: S8LayoutCloseoutEvidenceLane::CertificationCloseout,
        },
        S8LayoutScenarioKind::MultiArtifactIntegration => S8LayoutScenarioDefinition {
            kind,
            production_apis: INTEGRATION_APIS,
            actors: INTEGRATION_ACTORS,
            faults: INTEGRATION_FAULTS,
            observers: INTEGRATION_OBSERVERS,
            oracles: INTEGRATION_ORACLES,
            coverage: INTEGRATION_COVERAGE,
            shortcut_denials: INTEGRATION_SHORTCUTS,
            transitions: INTEGRATION_TRANSITIONS,
            transcript: S8LayoutTranscriptKind::ReplayBundle,
            closeout: S8LayoutCloseoutEvidenceLane::CertificationCloseout,
        },
    }
}

impl S8LayoutScenarioDefinition {
    pub const fn kind(&self) -> S8LayoutScenarioKind {
        self.kind
    }
    pub const fn production_apis(&self) -> &'static [S8LayoutProductionApi] {
        self.production_apis
    }
    pub const fn actors(&self) -> &'static [S8LayoutActorLane] {
        self.actors
    }
    pub const fn faults(&self) -> &'static [S8LayoutFaultLane] {
        self.faults
    }
    pub const fn observers(&self) -> &'static [S8LayoutObserverLane] {
        self.observers
    }
    pub const fn oracles(&self) -> &'static [S8LayoutOracleLane] {
        self.oracles
    }
    pub const fn coverage(&self) -> &'static [S8LayoutCoverageRowKind] {
        self.coverage
    }
    pub const fn shortcut_denials(&self) -> &'static [S8LayoutShortcutDenialKind] {
        self.shortcut_denials
    }
    pub const fn transitions(&self) -> &'static [S8LayoutTransitionState] {
        self.transitions
    }
    pub const fn transcript(&self) -> S8LayoutTranscriptKind {
        self.transcript
    }
    pub const fn closeout(&self) -> S8LayoutCloseoutEvidenceLane {
        self.closeout
    }
}
