#[derive(Debug)]
pub(crate) struct SelectedRecoveryHandles {
    pub(super) active_backups: Vec<super::ActiveBackupRecoveryHandle>,
    pub(super) indeterminate_repairs: Vec<super::IndeterminateRepairRecoveryHandle>,
    pub(super) indeterminate_recovery_staging: Vec<super::IndeterminateRecoveryStagingHandle>,
    pub(super) replica_bootstraps: Vec<super::ReplicaBootstrapRecoveryHandle>,
    pub(super) replica_promotions: Vec<super::ReplicaPromotionRecoveryHandle>,
}
