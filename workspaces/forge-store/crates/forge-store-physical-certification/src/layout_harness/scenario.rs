use super::actors::LayoutActorLane;
use super::closeout::LayoutCloseoutEvidenceLane;
use super::coverage::LayoutCoverageRowKind;
use super::faults::LayoutFaultLane;
use super::observers::LayoutObserverLane;
use super::oracles::LayoutOracleLane;
use super::shortcut_denials::LayoutShortcutDenialKind;
use super::transcripts::LayoutTranscriptKind;
mod queries;

pub use queries::{
    all_layout_index_layout_scenarios, canonical_layout_index_layout_production_apis,
    canonical_layout_index_layout_required_transitions,
    canonical_layout_index_layout_shortcut_denials,
    canonical_layout_index_layout_supported_scenarios,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutProductionApi {
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
pub enum LayoutTransitionState {
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
pub enum LayoutScenarioKind {
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
pub struct LayoutScenarioDefinition {
    kind: LayoutScenarioKind,
    production_apis: &'static [LayoutProductionApi],
    actors: &'static [LayoutActorLane],
    faults: &'static [LayoutFaultLane],
    observers: &'static [LayoutObserverLane],
    oracles: &'static [LayoutOracleLane],
    coverage: &'static [LayoutCoverageRowKind],
    shortcut_denials: &'static [LayoutShortcutDenialKind],
    transitions: &'static [LayoutTransitionState],
    transcript: LayoutTranscriptKind,
    closeout: LayoutCloseoutEvidenceLane,
}

const DECLARATION_APIS: &[LayoutProductionApi] = &[
    LayoutProductionApi::LayoutFamilies,
    LayoutProductionApi::LayoutStrategyAdmission,
];
const ACCESS_APIS: &[LayoutProductionApi] = &[
    LayoutProductionApi::AccessPlanning,
    LayoutProductionApi::AccessLowering,
];
const EXECUTION_APIS: &[LayoutProductionApi] = &[
    LayoutProductionApi::AccessPlanning,
    LayoutProductionApi::AccessLowering,
    LayoutProductionApi::AccessExecution,
];
const REBUILD_APIS: &[LayoutProductionApi] = &[
    LayoutProductionApi::LayoutRebuild,
    LayoutProductionApi::LayoutReadmission,
];
const MIGRATION_APIS: &[LayoutProductionApi] = &[
    LayoutProductionApi::LayoutMigration,
    LayoutProductionApi::LayoutReadmission,
];
const READMISSION_APIS: &[LayoutProductionApi] = &[LayoutProductionApi::LayoutReadmission];
const INTEGRATION_APIS: &[LayoutProductionApi] = &[
    LayoutProductionApi::LayoutFamilies,
    LayoutProductionApi::LayoutStrategyAdmission,
    LayoutProductionApi::AccessPlanning,
    LayoutProductionApi::AccessLowering,
    LayoutProductionApi::AccessExecution,
    LayoutProductionApi::LayoutRebuild,
    LayoutProductionApi::LayoutMigration,
    LayoutProductionApi::LayoutReadmission,
];

const DECLARATION_ACTORS: &[LayoutActorLane] = &[
    LayoutActorLane::DeclarationCatalog,
    LayoutActorLane::OfflineVerifier,
];
const ACCESS_ACTORS: &[LayoutActorLane] = &[LayoutActorLane::ForegroundAccess];
const RECOVERY_ACTORS: &[LayoutActorLane] =
    &[LayoutActorLane::Recovery, LayoutActorLane::Maintenance];
const MIGRATION_ACTORS: &[LayoutActorLane] =
    &[LayoutActorLane::Migration, LayoutActorLane::Recovery];
const READMISSION_ACTORS: &[LayoutActorLane] =
    &[LayoutActorLane::OfflineVerifier, LayoutActorLane::Recovery];
const INTEGRATION_ACTORS: &[LayoutActorLane] = &[
    LayoutActorLane::DeclarationCatalog,
    LayoutActorLane::ForegroundAccess,
    LayoutActorLane::Recovery,
    LayoutActorLane::Migration,
    LayoutActorLane::Maintenance,
    LayoutActorLane::OfflineVerifier,
];

const NO_FAULTS: &[LayoutFaultLane] = &[LayoutFaultLane::NoFaultControl];
const REBUILD_FAULTS: &[LayoutFaultLane] = &[
    LayoutFaultLane::ByteCorruption,
    LayoutFaultLane::StaleGeneration,
];
const MIGRATION_FAULTS: &[LayoutFaultLane] = &[
    LayoutFaultLane::CrashInterruption,
    LayoutFaultLane::ReorderedPersistence,
];
const READMISSION_FAULTS: &[LayoutFaultLane] = &[
    LayoutFaultLane::TerminalProjectionShortcut,
    LayoutFaultLane::StaleGeneration,
];
const INTEGRATION_FAULTS: &[LayoutFaultLane] = &[
    LayoutFaultLane::CrashInterruption,
    LayoutFaultLane::ByteCorruption,
    LayoutFaultLane::ReorderedPersistence,
];

const DECLARATION_OBSERVERS: &[LayoutObserverLane] = &[
    LayoutObserverLane::DeclarationInventoryObserver,
    LayoutObserverLane::OfflineVerifierObserver,
];
const EXECUTION_OBSERVERS: &[LayoutObserverLane] = &[
    LayoutObserverLane::CounterReceiptObserver,
    LayoutObserverLane::MultiArtifactTraceObserver,
];
const RECOVERY_OBSERVERS: &[LayoutObserverLane] = &[
    LayoutObserverLane::RecoveryOutcomeObserver,
    LayoutObserverLane::OfflineVerifierObserver,
];
const READMISSION_OBSERVERS: &[LayoutObserverLane] = &[
    LayoutObserverLane::ReadmissionObserver,
    LayoutObserverLane::OfflineVerifierObserver,
];
const INTEGRATION_OBSERVERS: &[LayoutObserverLane] = &[
    LayoutObserverLane::CounterReceiptObserver,
    LayoutObserverLane::RecoveryOutcomeObserver,
    LayoutObserverLane::MultiArtifactTraceObserver,
];

const DECLARATION_ORACLES: &[LayoutOracleLane] = &[LayoutOracleLane::DeclarationInventoryOracle];
const ACCESS_ORACLES: &[LayoutOracleLane] = &[
    LayoutOracleLane::AccessShapeDenialOracle,
    LayoutOracleLane::BroadScanRejectionOracle,
];
const COUNTER_ORACLES: &[LayoutOracleLane] = &[LayoutOracleLane::ExactCounterOracle];
const REBUILD_ORACLES: &[LayoutOracleLane] = &[LayoutOracleLane::RebuildParityOracle];
const MIGRATION_ORACLES: &[LayoutOracleLane] = &[LayoutOracleLane::MigrationRollbackOracle];
const READMISSION_ORACLES: &[LayoutOracleLane] = &[LayoutOracleLane::ReadmissionBoundaryOracle];
const INTEGRATION_ORACLES: &[LayoutOracleLane] =
    &[LayoutOracleLane::MultiArtifactIntegrationOracle];

const DECLARATION_COVERAGE: &[LayoutCoverageRowKind] =
    &[LayoutCoverageRowKind::DeclarationInventory];
const ACCESS_SHAPE_COVERAGE: &[LayoutCoverageRowKind] = &[LayoutCoverageRowKind::AccessShapeDenial];
const BROAD_SCAN_COVERAGE: &[LayoutCoverageRowKind] = &[LayoutCoverageRowKind::BroadScanRejection];
const COUNTER_COVERAGE: &[LayoutCoverageRowKind] = &[LayoutCoverageRowKind::ExactCounter];
const REBUILD_COVERAGE: &[LayoutCoverageRowKind] = &[LayoutCoverageRowKind::RebuildParity];
const MIGRATION_COVERAGE: &[LayoutCoverageRowKind] = &[LayoutCoverageRowKind::MigrationRollback];
const READMISSION_COVERAGE: &[LayoutCoverageRowKind] =
    &[LayoutCoverageRowKind::ReadmissionBoundary];
const INTEGRATION_COVERAGE: &[LayoutCoverageRowKind] =
    &[LayoutCoverageRowKind::MultiArtifactIntegration];

const ACCESS_SHORTCUTS: &[LayoutShortcutDenialKind] = &[
    LayoutShortcutDenialKind::BroadScanMasqueradingAsPointLookup,
    LayoutShortcutDenialKind::CopiedCounterRows,
];
const READMISSION_SHORTCUTS: &[LayoutShortcutDenialKind] = &[
    LayoutShortcutDenialKind::TerminalProjectionAuthority,
    LayoutShortcutDenialKind::FoundationalMaterializationAuthority,
];
const FIXTURE_SHORTCUTS: &[LayoutShortcutDenialKind] =
    &[LayoutShortcutDenialKind::SyntheticFixtureAuthority];
const INTEGRATION_SHORTCUTS: &[LayoutShortcutDenialKind] = &[
    LayoutShortcutDenialKind::CopiedCounterRows,
    LayoutShortcutDenialKind::LooseLogEvidence,
    LayoutShortcutDenialKind::SyntheticFixtureAuthority,
];

const DECLARATION_TRANSITIONS: &[LayoutTransitionState] = &[
    LayoutTransitionState::Declared,
    LayoutTransitionState::Admitted,
];
const ACCESS_TRANSITIONS: &[LayoutTransitionState] = &[
    LayoutTransitionState::Admitted,
    LayoutTransitionState::Planned,
    LayoutTransitionState::Lowered,
];
const EXECUTION_TRANSITIONS: &[LayoutTransitionState] = &[
    LayoutTransitionState::Admitted,
    LayoutTransitionState::Planned,
    LayoutTransitionState::Lowered,
    LayoutTransitionState::ExecutionReady,
    LayoutTransitionState::Executed,
];
const REBUILD_TRANSITIONS: &[LayoutTransitionState] = &[
    LayoutTransitionState::Executed,
    LayoutTransitionState::Rebuilt,
    LayoutTransitionState::Readmitted,
];
const MIGRATION_TRANSITIONS: &[LayoutTransitionState] = &[
    LayoutTransitionState::Declared,
    LayoutTransitionState::Admitted,
    LayoutTransitionState::Rebound,
    LayoutTransitionState::Readmitted,
];
const READMISSION_TRANSITIONS: &[LayoutTransitionState] = &[
    LayoutTransitionState::Executed,
    LayoutTransitionState::Readmitted,
];
const INTEGRATION_TRANSITIONS: &[LayoutTransitionState] = &[
    LayoutTransitionState::Declared,
    LayoutTransitionState::Admitted,
    LayoutTransitionState::Planned,
    LayoutTransitionState::Lowered,
    LayoutTransitionState::ExecutionReady,
    LayoutTransitionState::Executed,
    LayoutTransitionState::Rebuilt,
    LayoutTransitionState::Rebound,
    LayoutTransitionState::Readmitted,
];

pub fn layout_scenario(kind: LayoutScenarioKind) -> LayoutScenarioDefinition {
    match kind {
        LayoutScenarioKind::LayoutDeclarationInventory => LayoutScenarioDefinition {
            kind,
            production_apis: DECLARATION_APIS,
            actors: DECLARATION_ACTORS,
            faults: NO_FAULTS,
            observers: DECLARATION_OBSERVERS,
            oracles: DECLARATION_ORACLES,
            coverage: DECLARATION_COVERAGE,
            shortcut_denials: FIXTURE_SHORTCUTS,
            transitions: DECLARATION_TRANSITIONS,
            transcript: LayoutTranscriptKind::ScenarioTranscript,
            closeout: LayoutCloseoutEvidenceLane::ScenarioDefinition,
        },
        LayoutScenarioKind::AccessShapeDenial => LayoutScenarioDefinition {
            kind,
            production_apis: ACCESS_APIS,
            actors: ACCESS_ACTORS,
            faults: NO_FAULTS,
            observers: EXECUTION_OBSERVERS,
            oracles: ACCESS_ORACLES,
            coverage: ACCESS_SHAPE_COVERAGE,
            shortcut_denials: ACCESS_SHORTCUTS,
            transitions: ACCESS_TRANSITIONS,
            transcript: LayoutTranscriptKind::ShortcutDenialTrace,
            closeout: LayoutCloseoutEvidenceLane::ScenarioDefinition,
        },
        LayoutScenarioKind::BroadScanRejection => LayoutScenarioDefinition {
            kind,
            production_apis: ACCESS_APIS,
            actors: ACCESS_ACTORS,
            faults: NO_FAULTS,
            observers: EXECUTION_OBSERVERS,
            oracles: ACCESS_ORACLES,
            coverage: BROAD_SCAN_COVERAGE,
            shortcut_denials: ACCESS_SHORTCUTS,
            transitions: ACCESS_TRANSITIONS,
            transcript: LayoutTranscriptKind::ShortcutDenialTrace,
            closeout: LayoutCloseoutEvidenceLane::PerformanceEvidence,
        },
        LayoutScenarioKind::ExactCounter => LayoutScenarioDefinition {
            kind,
            production_apis: EXECUTION_APIS,
            actors: ACCESS_ACTORS,
            faults: NO_FAULTS,
            observers: EXECUTION_OBSERVERS,
            oracles: COUNTER_ORACLES,
            coverage: COUNTER_COVERAGE,
            shortcut_denials: ACCESS_SHORTCUTS,
            transitions: EXECUTION_TRANSITIONS,
            transcript: LayoutTranscriptKind::ScenarioTranscript,
            closeout: LayoutCloseoutEvidenceLane::PerformanceEvidence,
        },
        LayoutScenarioKind::CorruptionRebuildParity => LayoutScenarioDefinition {
            kind,
            production_apis: REBUILD_APIS,
            actors: RECOVERY_ACTORS,
            faults: REBUILD_FAULTS,
            observers: RECOVERY_OBSERVERS,
            oracles: REBUILD_ORACLES,
            coverage: REBUILD_COVERAGE,
            shortcut_denials: READMISSION_SHORTCUTS,
            transitions: REBUILD_TRANSITIONS,
            transcript: LayoutTranscriptKind::ReplayBundle,
            closeout: LayoutCloseoutEvidenceLane::CertificationCloseout,
        },
        LayoutScenarioKind::MigrationRollbackInterruption => LayoutScenarioDefinition {
            kind,
            production_apis: MIGRATION_APIS,
            actors: MIGRATION_ACTORS,
            faults: MIGRATION_FAULTS,
            observers: RECOVERY_OBSERVERS,
            oracles: MIGRATION_ORACLES,
            coverage: MIGRATION_COVERAGE,
            shortcut_denials: FIXTURE_SHORTCUTS,
            transitions: MIGRATION_TRANSITIONS,
            transcript: LayoutTranscriptKind::ReplayBundle,
            closeout: LayoutCloseoutEvidenceLane::CertificationCloseout,
        },
        LayoutScenarioKind::TrustBoundaryReadmission => LayoutScenarioDefinition {
            kind,
            production_apis: READMISSION_APIS,
            actors: READMISSION_ACTORS,
            faults: READMISSION_FAULTS,
            observers: READMISSION_OBSERVERS,
            oracles: READMISSION_ORACLES,
            coverage: READMISSION_COVERAGE,
            shortcut_denials: READMISSION_SHORTCUTS,
            transitions: READMISSION_TRANSITIONS,
            transcript: LayoutTranscriptKind::ShortcutDenialTrace,
            closeout: LayoutCloseoutEvidenceLane::CertificationCloseout,
        },
        LayoutScenarioKind::MultiArtifactIntegration => LayoutScenarioDefinition {
            kind,
            production_apis: INTEGRATION_APIS,
            actors: INTEGRATION_ACTORS,
            faults: INTEGRATION_FAULTS,
            observers: INTEGRATION_OBSERVERS,
            oracles: INTEGRATION_ORACLES,
            coverage: INTEGRATION_COVERAGE,
            shortcut_denials: INTEGRATION_SHORTCUTS,
            transitions: INTEGRATION_TRANSITIONS,
            transcript: LayoutTranscriptKind::ReplayBundle,
            closeout: LayoutCloseoutEvidenceLane::CertificationCloseout,
        },
    }
}

impl LayoutScenarioDefinition {
    pub const fn kind(&self) -> LayoutScenarioKind {
        self.kind
    }
    pub const fn production_apis(&self) -> &'static [LayoutProductionApi] {
        self.production_apis
    }
    pub const fn actors(&self) -> &'static [LayoutActorLane] {
        self.actors
    }
    pub const fn faults(&self) -> &'static [LayoutFaultLane] {
        self.faults
    }
    pub const fn observers(&self) -> &'static [LayoutObserverLane] {
        self.observers
    }
    pub const fn oracles(&self) -> &'static [LayoutOracleLane] {
        self.oracles
    }
    pub const fn coverage(&self) -> &'static [LayoutCoverageRowKind] {
        self.coverage
    }
    pub const fn shortcut_denials(&self) -> &'static [LayoutShortcutDenialKind] {
        self.shortcut_denials
    }
    pub const fn transitions(&self) -> &'static [LayoutTransitionState] {
        self.transitions
    }
    pub const fn transcript(&self) -> LayoutTranscriptKind {
        self.transcript
    }
    pub const fn closeout(&self) -> LayoutCloseoutEvidenceLane {
        self.closeout
    }
}
