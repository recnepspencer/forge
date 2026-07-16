mod execution;
mod intent;
mod lowering;

pub use execution::{
    BackupRestoreExecutionDenial, BackupRestoreReadinessDenial, ExecutedBackupRestore,
    ExecutionReadyBackupRestore, RestoreExecutionReceipt,
};
pub use intent::{BackupRestoreIntent, EvidenceBoundBackupRestorePlan};
pub use lowering::{
    AuthorizedBackupRestorePlan, BackupRestoreLoweringDenial, LoweredBackupRestorePlan,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct BackupRestoreOperation;
