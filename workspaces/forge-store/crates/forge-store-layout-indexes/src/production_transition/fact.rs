use super::S8OwnerOutcomeCase;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutStateMachine {
    ArtifactDeclaration,
    KeyDomainAdmission,
    StrategyInvariantAdmission,
    LayoutAdmission,
    AccessSelectionAndBudgetAdmission,
    AccessLowering,
    ExecutionReadiness,
    ExecutedEvidence,
    DerivedRebuildParity,
    LiveMaintenanceAdmissionAndLowering,
    MigrationRollbackPlanning,
    StaleRebindReadmission,
    CorruptionQuarantine,
    BootstrapCatalogDiscovery,
    FullDeclaredScanAdmission,
    DegradedExactScan,
    MaterializationCoverageAbsence,
    BTreeSearchPathInvariant,
    CompactionCutover,
    LegacyDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutMachineState {
    Unclassified,
    Declared,
    SelectionRequested,
    Admitted,
    Budgeted,
    Lowered,
    Ready,
    Executed,
    ExactCountersObserved,
    Rebuilt,
    Stale,
    Readmitted,
    Quarantined,
    QuarantineReadmissionRequired,
    OfflineEvidenceReadmissionRequired,
    TerminalImportReadmissionRequired,
    CoveragePartial,
    AbsenceProven,
    CanonicalKeysAdmitted,
    SeparatorValidated,
    ParityVerified,
    RebindRequired,
    Deferred,
    MaintenanceReady,
    MaintenanceAdmittedLagged,
    MaintenanceAdmittedDeferred,
    MaintenanceLagged,
    MaintenanceDeferred,
    MaintenanceRebuildOnly,
    MaintenanceAdvisoryOnly,
    MaintenanceVerifierOnly,
    MaintenanceMigrationOnly,
    Clean,
    NotFound,
    RebuildRequired,
    MigrationRequired,
    Unsupported,
    CatalogDiscovered,
    CurrentRootAdmitted,
    CompactionPlanAdmitted,
    CompactionRewriteLowered,
    CompactionTombstoneRetentionAdmitted,
    CompactionPublicationCommitted,
    CompactionRecoveryVisibilityAdmitted,
    CompactionReclaimDeferred,
    CompactionReclaimed,
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutMachineTransition {
    Declare,
    Admit,
    Deny,
    Lower,
    Ready,
    Execute,
    Rebuild,
    Rebind,
    Readmit,
    Quarantine,
    Budget,
    AdmitExactCounters,
    SelectAndAdmitBudget,
    Resolve,
    VerifyParity,
    ProveAbsence,
    ValidateSeparator,
    Classify,
    Defer,
    RequireRebind,
    ValidateCurrentRoot,
    LowerReady,
    LowerLagged,
    LowerDeferred,
    Publish,
    AdmitRecoveryVisibility,
    AdmitTombstoneRetention,
    DeferReclaim,
    Reclaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8LayoutProductionOperation {
    DeclareArtifactFamily,
    AdmitKeyDomain,
    AdmitStrategyInvariantSuite,
    AdmitLayoutStrategy,
    SelectAccessPlanWithBudget,
    LowerSelectedAccess,
    AdmitExecutionReadiness,
    AdmitCountersAndExecute,
    RebindAndReadmitStaleAccess,
    RebuildDerivedIndex,
    VerifyDerivedParity,
    AdmitLiveMaintenance,
    LowerLiveMaintenance,
    PlanMigration,
    PlanRollback,
    ClassifyCorruption,
    ReadmitCorruptionEvidence,
    ReadDiscoveredBootstrapCatalog,
    AdmitFullDeclaredScan,
    ExecuteBudgetedDegradedExactScan,
    ProveExactIndexAbsence,
    VerifyBTreeSearchPath,
    LowerCompactionRewrite,
    AdmitCompactionTombstoneRetention,
    PublishCompactionRewrite,
    AdmitCompactionRecoveryVisibility,
    DeferCompactionReclaim,
    DrainCompactionReclaim,
    DenyCompactionMutation,
    ClassifyLegacyDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutMachineEdge {
    from: S8LayoutMachineState,
    transition: S8LayoutMachineTransition,
    to: S8LayoutMachineState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutProductionTransition {
    case: S8OwnerOutcomeCase,
    edge: S8LayoutMachineEdge,
}

impl S8LayoutProductionTransition {
    pub(crate) const fn new(
        machine: S8LayoutStateMachine,
        production_operation: S8LayoutProductionOperation,
        case_name: &'static str,
        from: S8LayoutMachineState,
        transition: S8LayoutMachineTransition,
        to: S8LayoutMachineState,
    ) -> Self {
        Self {
            case: S8OwnerOutcomeCase::new(machine, production_operation, case_name),
            edge: S8LayoutMachineEdge::new(from, transition, to),
        }
    }

    pub const fn machine(self) -> S8LayoutStateMachine {
        self.case.machine()
    }

    pub const fn production_operation(self) -> S8LayoutProductionOperation {
        self.case.production_operation()
    }

    pub const fn outcome_case(self) -> S8OwnerOutcomeCase {
        self.case
    }

    pub const fn edge(self) -> S8LayoutMachineEdge {
        self.edge
    }
}

pub(crate) const fn owner_transition(
    machine: S8LayoutStateMachine,
    production_operation: S8LayoutProductionOperation,
    case_name: &'static str,
    from: S8LayoutMachineState,
    transition: S8LayoutMachineTransition,
    to: S8LayoutMachineState,
) -> S8LayoutProductionTransition {
    S8LayoutProductionTransition::new(
        machine,
        production_operation,
        case_name,
        from,
        transition,
        to,
    )
}

impl S8LayoutMachineEdge {
    const fn new(
        from: S8LayoutMachineState,
        transition: S8LayoutMachineTransition,
        to: S8LayoutMachineState,
    ) -> Self {
        Self {
            from,
            transition,
            to,
        }
    }

    pub const fn from(self) -> S8LayoutMachineState {
        self.from
    }
    pub const fn transition(self) -> S8LayoutMachineTransition {
        self.transition
    }
    pub const fn to(self) -> S8LayoutMachineState {
        self.to
    }
}
