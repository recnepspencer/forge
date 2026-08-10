use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MaintenanceWorkClass {
    RetentionAudit,
    CompactionMaintenance,
    DerivedArtifactReclaim,
    AuthoritativeReclaim,
    RetainedRangeRebuild,
    SnapshotRefresh,
    DerivedFamilyRebuild,
    ReplicationPreparation,
    MaintenanceAudit,
    TierPlacementProposal,
    TierMoveExecution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceExecutionPosture {
    ForegroundBlocking,
    ForegroundAware,
    FullyDeferrable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MaintenanceDebtFamily {
    CompactionDebt,
    RebuildDebt,
    SnapshotDebt,
    ReplicationPreparationDebt,
    TierPlacementDebt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ForegroundReservationFamily {
    Write,
    Read,
    Continuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BackgroundReservationFamily {
    Maintenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MaintenanceReservationFamily {
    Foreground(ForegroundReservationFamily),
    Background(BackgroundReservationFamily),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceEscalationDecision {
    StayBackground,
    PaceUpWithinBackgroundBudget,
    EscalateWithForegroundImpact,
    DeferWithOperatorSignal,
    RejectNewDerivedWork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TierWorkContainerClass {
    TierPlacementProposal,
    TierMoveExecution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceFailureKind {
    ReservationViolation,
    FreshnessFailure,
    EquivalenceConflict,
    RestartAdmissionFailure,
    Deferred,
    Cancelled,
    ExecutionFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenancePlanFamily {
    ForegroundReserved,
    BackgroundPaced,
    Escalated,
    Deferred,
    Cancelled,
}
