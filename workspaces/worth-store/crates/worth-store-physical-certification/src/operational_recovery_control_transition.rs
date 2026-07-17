use worth_store_operations::{OperationalControlRecordKind, OperationalControlRecordKind as Kind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationalRecoveryControlTransitionKind {
    BackupSourceLease,
    BackupMaterializationOpen,
    BackupMaterializationCompletion,
    IndependentBackupVerification,
    AuthorizationConsumption,
    RecoveryOwnerReceipt,
    RecoveryStagingCompletion,
    RepairExecutionOpen,
    RepairOwnerEffect,
    RepairOwnerReceipt,
    RepairDisposition,
    RecoveryPublicationPreparation,
    RecoveryPublicationPending,
    RecoveryPublicationDisposition,
    RecoveryPublicationFenceRelease,
    WorkflowAbandonment,
    ReplicaBootstrapTransfer,
    ReplicaBootstrapCompletion,
    ReplicaPromotionFence,
    ReplicaPromotionRecord,
    ReplicaPromotionPublication,
    ReplicaPromotionReadmission,
    OldPrimaryRejoinPlan,
    OldPrimaryRejoinCompletion,
}

impl OperationalRecoveryControlTransitionKind {
    pub const ALL: [Self; 24] = [
        Self::BackupSourceLease,
        Self::BackupMaterializationOpen,
        Self::BackupMaterializationCompletion,
        Self::IndependentBackupVerification,
        Self::AuthorizationConsumption,
        Self::RecoveryOwnerReceipt,
        Self::RecoveryStagingCompletion,
        Self::RepairExecutionOpen,
        Self::RepairOwnerEffect,
        Self::RepairOwnerReceipt,
        Self::RepairDisposition,
        Self::RecoveryPublicationPreparation,
        Self::RecoveryPublicationPending,
        Self::RecoveryPublicationDisposition,
        Self::RecoveryPublicationFenceRelease,
        Self::WorkflowAbandonment,
        Self::ReplicaBootstrapTransfer,
        Self::ReplicaBootstrapCompletion,
        Self::ReplicaPromotionFence,
        Self::ReplicaPromotionRecord,
        Self::ReplicaPromotionPublication,
        Self::ReplicaPromotionReadmission,
        Self::OldPrimaryRejoinPlan,
        Self::OldPrimaryRejoinCompletion,
    ];

    pub fn from_record(record: &OperationalControlRecordKind) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.matches(record))
    }

    pub const fn matches(self, record: &OperationalControlRecordKind) -> bool {
        match self {
            Self::BackupSourceLease => matches!(record, Kind::SourceLeasePersisted { .. }),
            Self::BackupMaterializationOpen => {
                matches!(record, Kind::BackupMaterializationOpened { .. })
            }
            Self::BackupMaterializationCompletion => {
                matches!(record, Kind::BackupMaterializationRecorded { .. })
            }
            Self::IndependentBackupVerification => matches!(
                record,
                Kind::IndependentBackupVerificationRecordedAndSourceLeaseReleased { .. }
            ),
            Self::AuthorizationConsumption => matches!(record, Kind::AuthorizationConsumed { .. }),
            Self::RecoveryOwnerReceipt => {
                matches!(record, Kind::OperationalOwnerReceiptPersisted { .. })
            }
            Self::RecoveryStagingCompletion => {
                matches!(record, Kind::RecoveryStagingCompleted { .. })
            }
            Self::RepairExecutionOpen => matches!(record, Kind::RepairExecutionOpened { .. }),
            Self::RepairOwnerEffect => matches!(record, Kind::RepairOwnerEffectStarted { .. }),
            Self::RepairOwnerReceipt => matches!(record, Kind::RepairOwnerReceiptPersisted { .. }),
            Self::RepairDisposition => matches!(record, Kind::RepairDispositionRecorded { .. }),
            Self::RecoveryPublicationPreparation => {
                matches!(record, Kind::RecoveryPublicationPrepared { .. })
            }
            Self::RecoveryPublicationPending => {
                matches!(record, Kind::RecoveryPublicationPending { .. })
            }
            Self::RecoveryPublicationDisposition => {
                matches!(record, Kind::RecoveryPublicationDisposition { .. })
            }
            Self::RecoveryPublicationFenceRelease => {
                matches!(record, Kind::RecoveryPublicationFenceReleased { .. })
            }
            Self::WorkflowAbandonment => matches!(
                record,
                Kind::BackupAbandoned { .. } | Kind::ReplicaBootstrapAbandoned { .. }
            ),
            Self::ReplicaBootstrapTransfer => {
                matches!(record, Kind::ReplicaBootstrapTransferRecorded { .. })
            }
            Self::ReplicaBootstrapCompletion => {
                matches!(record, Kind::ReplicaBootstrapCompleted { .. })
            }
            Self::ReplicaPromotionFence => {
                matches!(record, Kind::ReplicaPromotionFenceRecorded { .. })
            }
            Self::ReplicaPromotionRecord => matches!(record, Kind::ReplicaPromotionRecorded { .. }),
            Self::ReplicaPromotionPublication => {
                matches!(record, Kind::ReplicaPromotionPublished { .. })
            }
            Self::ReplicaPromotionReadmission => {
                matches!(record, Kind::ReplicaPromotionReadmitted { .. })
            }
            Self::OldPrimaryRejoinPlan => matches!(record, Kind::OldPrimaryRejoinPlanned { .. }),
            Self::OldPrimaryRejoinCompletion => {
                matches!(record, Kind::OldPrimaryRejoinCompleted { .. })
            }
        }
    }

    pub(super) fn before_token(self) -> &'static str {
        match self {
            Self::BackupSourceLease => "s10-control-before-backup-source-lease",
            Self::BackupMaterializationOpen => "s10-control-before-materialization-open",
            Self::BackupMaterializationCompletion => {
                "s10-control-before-materialization-completion"
            }
            Self::IndependentBackupVerification => {
                "s10-control-before-independent-backup-verification"
            }
            Self::AuthorizationConsumption => "s10-control-before-authorization-consumption",
            Self::RecoveryOwnerReceipt => "s10-control-before-recovery-owner-receipt",
            Self::RecoveryStagingCompletion => "s10-control-before-recovery-staging-completion",
            Self::RepairExecutionOpen => "s10-control-before-repair-execution-open",
            Self::RepairOwnerEffect => "s10-control-before-repair-owner-effect",
            Self::RepairOwnerReceipt => "s10-control-before-repair-owner-receipt",
            Self::RepairDisposition => "s10-control-before-repair-disposition",
            Self::RecoveryPublicationPreparation => "s10-control-before-publication-preparation",
            Self::RecoveryPublicationPending => "s10-control-before-publication-pending",
            Self::RecoveryPublicationDisposition => "s10-control-before-publication-disposition",
            Self::RecoveryPublicationFenceRelease => "s10-control-before-publication-fence-release",
            Self::WorkflowAbandonment => "s10-control-before-workflow-abandonment",
            Self::ReplicaBootstrapTransfer => "s10-control-before-replica-bootstrap-transfer",
            Self::ReplicaBootstrapCompletion => "s10-control-before-replica-bootstrap-completion",
            Self::ReplicaPromotionFence => "s10-control-before-replica-promotion-fence",
            Self::ReplicaPromotionRecord => "s10-control-before-replica-promotion-record",
            Self::ReplicaPromotionPublication => "s10-control-before-replica-promotion-publication",
            Self::ReplicaPromotionReadmission => "s10-control-before-replica-promotion-readmission",
            Self::OldPrimaryRejoinPlan => "s10-control-before-old-primary-rejoin-plan",
            Self::OldPrimaryRejoinCompletion => "s10-control-before-old-primary-rejoin-completion",
        }
    }
    pub(super) fn after_token(self) -> &'static str {
        match self {
            Self::BackupSourceLease => "s10-control-after-backup-source-lease",
            Self::BackupMaterializationOpen => "s10-control-after-materialization-open",
            Self::BackupMaterializationCompletion => "s10-control-after-materialization-completion",
            Self::IndependentBackupVerification => {
                "s10-control-after-independent-backup-verification"
            }
            Self::AuthorizationConsumption => "s10-control-after-authorization-consumption",
            Self::RecoveryOwnerReceipt => "s10-control-after-recovery-owner-receipt",
            Self::RecoveryStagingCompletion => "s10-control-after-recovery-staging-completion",
            Self::RepairExecutionOpen => "s10-control-after-repair-execution-open",
            Self::RepairOwnerEffect => "s10-control-after-repair-owner-effect",
            Self::RepairOwnerReceipt => "s10-control-after-repair-owner-receipt",
            Self::RepairDisposition => "s10-control-after-repair-disposition",
            Self::RecoveryPublicationPreparation => "s10-control-after-publication-preparation",
            Self::RecoveryPublicationPending => "s10-control-after-publication-pending",
            Self::RecoveryPublicationDisposition => "s10-control-after-publication-disposition",
            Self::RecoveryPublicationFenceRelease => "s10-control-after-publication-fence-release",
            Self::WorkflowAbandonment => "s10-control-after-workflow-abandonment",
            Self::ReplicaBootstrapTransfer => "s10-control-after-replica-bootstrap-transfer",
            Self::ReplicaBootstrapCompletion => "s10-control-after-replica-bootstrap-completion",
            Self::ReplicaPromotionFence => "s10-control-after-replica-promotion-fence",
            Self::ReplicaPromotionRecord => "s10-control-after-replica-promotion-record",
            Self::ReplicaPromotionPublication => "s10-control-after-replica-promotion-publication",
            Self::ReplicaPromotionReadmission => "s10-control-after-replica-promotion-readmission",
            Self::OldPrimaryRejoinPlan => "s10-control-after-old-primary-rejoin-plan",
            Self::OldPrimaryRejoinCompletion => "s10-control-after-old-primary-rejoin-completion",
        }
    }
}
