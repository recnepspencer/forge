use worth_store_physical_backend::ControlMediaFault;

use super::{
    ActiveBackupRecoveryHandle, IndeterminateRecoveryStagingHandle,
    IndeterminateRepairRecoveryHandle, OperationalControlReplayResource, OperationalOperationId,
    OperationalWorkflowKind, PendingRecoveryPublicationHandle, PreparedRecoveryPublicationHandle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalControlHistoryViolation {
    record_index: u64,
    operation_id: OperationalOperationId,
    kind: OperationalControlHistoryViolationKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationalControlHistoryViolationKind {
    DuplicateWorkflowOpen,
    RecordBeforeWorkflowOpen,
    BackupRecordForDifferentWorkflow { workflow: OperationalWorkflowKind },
    SourceLeaseAuthorityMismatch,
    DuplicateSourceLease,
    MaterializationBeforeSourceLease,
    MaterializationCutMismatch,
    DuplicateMaterializationPlan,
    MaterializationReceiptBeforePlan,
    DuplicateMaterializationReceipt,
    VerificationBeforeMaterialization,
    TerminalBeforeSourceLease,
    TerminalReleaseCutMismatch,
    RecordAfterTerminal,
    WorkflowOpenWithoutDurableSourceLease,
    AuthorizationConsumptionConflict,
    DuplicateRepairJournalOpen,
    RepairJournalAuthorizationMismatch,
    RepairRecordBeforeJournalOpen,
    RepairPlanFingerprintMismatch,
    DuplicateRepairOwnerReceipt,
    DuplicateRepairOwnerStart,
    RepairReceiptBeforeOwnerStart,
    RepairCompletedBeforeAllOwnerReceipts,
    RepairRecordAfterDisposition,
    DuplicateRecoveryPublication,
    RecoveryDispositionBeforePublication,
    RecoveryPublicationIdentityMismatch,
    DuplicateRecoveryPublicationDisposition,
    DuplicateRecoveryStaging,
    RecoveryStagingCompletionBeforeAuthorization,
    RecoveryStagingBindingMismatch,
    RecoveryPublicationBeforeStagingCompletion,
    RecoveryPublicationBeforePreparation,
    RecoveryPublicationBindingMismatch,
    FenceReleaseBeforeDisposition,
    FenceReleaseBindingMismatch,
}

impl OperationalControlHistoryViolation {
    pub(crate) const fn new(
        record_index: u64,
        operation_id: OperationalOperationId,
        kind: OperationalControlHistoryViolationKind,
    ) -> Self {
        Self {
            record_index,
            operation_id,
            kind,
        }
    }
    pub const fn record_index(&self) -> u64 {
        self.record_index
    }

    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }

    pub const fn kind(&self) -> &OperationalControlHistoryViolationKind {
        &self.kind
    }
}

pub(crate) enum SelectedControlReplayDenial {
    AllocationFailed,
    CounterOverflow,
    DerivedIndex(ControlMediaFault),
    BudgetExceeded {
        resource: OperationalControlReplayResource,
        required: u64,
        limit: u64,
    },
    Invalid(OperationalControlHistoryViolation),
}

pub(crate) struct ReplayedSelectedControlHistory {
    pub(crate) active_backups: Vec<ActiveBackupRecoveryHandle>,
    pub(crate) completed_backups: u64,
    pub(crate) abandoned_backups: u64,
    pub(crate) indeterminate_repairs: Vec<IndeterminateRepairRecoveryHandle>,
    pub(crate) indeterminate_recovery_staging: Vec<IndeterminateRecoveryStagingHandle>,
    pub(crate) pending_recovery_publications: Vec<PendingRecoveryPublicationHandle>,
    pub(crate) prepared_recovery_publications: Vec<PreparedRecoveryPublicationHandle>,
    pub(crate) terminal_recovery_fence_releases: Vec<super::TerminalRecoveryFenceReleaseHandle>,
}

pub(super) enum StateLookupDenial {
    Semantic(OperationalControlHistoryViolationKind),
    DerivedIndex(ControlMediaFault),
}

pub(super) fn wrong_workflow(
    workflow: OperationalWorkflowKind,
) -> OperationalControlHistoryViolationKind {
    OperationalControlHistoryViolationKind::BackupRecordForDifferentWorkflow { workflow }
}

pub(super) const fn after_terminal() -> OperationalControlHistoryViolationKind {
    OperationalControlHistoryViolationKind::RecordAfterTerminal
}

pub(super) fn invalid<T>(
    record_index: u64,
    operation_id: OperationalOperationId,
    kind: OperationalControlHistoryViolationKind,
) -> Result<T, SelectedControlReplayDenial> {
    Err(SelectedControlReplayDenial::Invalid(
        OperationalControlHistoryViolation {
            record_index,
            operation_id,
            kind,
        },
    ))
}

pub(super) fn state_denial<T>(
    record_index: u64,
    operation_id: OperationalOperationId,
    denial: StateLookupDenial,
) -> Result<T, SelectedControlReplayDenial> {
    match denial {
        StateLookupDenial::Semantic(kind) => invalid(record_index, operation_id, kind),
        StateLookupDenial::DerivedIndex(fault) => {
            Err(SelectedControlReplayDenial::DerivedIndex(fault))
        }
    }
}
