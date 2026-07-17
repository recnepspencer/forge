use crate::OperationalRecoveryControlTransitionKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationalRecoveryYieldpoint {
    BeforeDurableControlTransition(OperationalRecoveryControlTransitionKind),
    AfterDurableControlTransition(OperationalRecoveryControlTransitionKind),
    BeforeForensicSourceAcquisition,
    AfterForensicSourceRecord,
    BeforeForensicFinalization,
    AfterForensicFinalization,
    BeforeBootstrapTransfer,
    AfterBootstrapTransfer,
    BeforeBootstrapControlRecord,
    AfterBootstrapControlRecord,
    BeforeBootstrapPostVerification,
    AfterBootstrapPostVerification,
    BeforeBootstrapCompletion,
    AfterBootstrapCompletion,
    BeforePromotionExternalFence,
    AfterPromotionExternalFence,
    BeforePromotionFenceRecord,
    AfterPromotionFenceRecord,
    BeforePromotionRecord,
    AfterPromotionRecord,
    BeforePromotionPostVerification,
    AfterPromotionPostVerification,
    BeforePromotionPublication,
    AfterPromotionPublication,
    BeforePromotionReadmission,
    AfterPromotionReadmission,
    BeforeOldPrimaryRejoinPlan,
    AfterOldPrimaryRejoinPlan,
    BeforeOldPrimaryRejoinExecution,
    AfterOldPrimaryRejoinExecution,
    BeforeOldPrimaryRejoinCompletion,
    AfterOldPrimaryRejoinCompletion,
    BeforeAuditDerivation,
    AfterAuditDerivation,
    BeforeAuditExport,
    AfterAuditExport,
}

impl OperationalRecoveryYieldpoint {
    pub const ALL: [Self; 82] = [
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::BackupSourceLease,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::BackupSourceLease,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::BackupMaterializationOpen,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::BackupMaterializationOpen,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::BackupMaterializationCompletion,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::BackupMaterializationCompletion,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::IndependentBackupVerification,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::IndependentBackupVerification,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::AuthorizationConsumption,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::AuthorizationConsumption,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryOwnerReceipt,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryOwnerReceipt,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryStagingCompletion,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryStagingCompletion,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RepairExecutionOpen,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RepairExecutionOpen,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RepairOwnerEffect,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RepairOwnerEffect,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RepairOwnerReceipt,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RepairOwnerReceipt,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RepairDisposition,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RepairDisposition,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryPublicationPreparation,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryPublicationPreparation,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryPublicationPending,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryPublicationPending,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryPublicationDisposition,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryPublicationDisposition,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryPublicationFenceRelease,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::RecoveryPublicationFenceRelease,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::WorkflowAbandonment,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::WorkflowAbandonment,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaBootstrapTransfer,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaBootstrapTransfer,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaBootstrapCompletion,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaBootstrapCompletion,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaPromotionFence,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaPromotionFence,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaPromotionRecord,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaPromotionRecord,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaPromotionPublication,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaPromotionPublication,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaPromotionReadmission,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::ReplicaPromotionReadmission,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::OldPrimaryRejoinPlan,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::OldPrimaryRejoinPlan,
        ),
        Self::BeforeDurableControlTransition(
            OperationalRecoveryControlTransitionKind::OldPrimaryRejoinCompletion,
        ),
        Self::AfterDurableControlTransition(
            OperationalRecoveryControlTransitionKind::OldPrimaryRejoinCompletion,
        ),
        Self::BeforeForensicSourceAcquisition,
        Self::AfterForensicSourceRecord,
        Self::BeforeForensicFinalization,
        Self::AfterForensicFinalization,
        Self::BeforeBootstrapTransfer,
        Self::AfterBootstrapTransfer,
        Self::BeforeBootstrapControlRecord,
        Self::AfterBootstrapControlRecord,
        Self::BeforeBootstrapPostVerification,
        Self::AfterBootstrapPostVerification,
        Self::BeforeBootstrapCompletion,
        Self::AfterBootstrapCompletion,
        Self::BeforePromotionExternalFence,
        Self::AfterPromotionExternalFence,
        Self::BeforePromotionFenceRecord,
        Self::AfterPromotionFenceRecord,
        Self::BeforePromotionRecord,
        Self::AfterPromotionRecord,
        Self::BeforePromotionPostVerification,
        Self::AfterPromotionPostVerification,
        Self::BeforePromotionPublication,
        Self::AfterPromotionPublication,
        Self::BeforePromotionReadmission,
        Self::AfterPromotionReadmission,
        Self::BeforeOldPrimaryRejoinPlan,
        Self::AfterOldPrimaryRejoinPlan,
        Self::BeforeOldPrimaryRejoinExecution,
        Self::AfterOldPrimaryRejoinExecution,
        Self::BeforeOldPrimaryRejoinCompletion,
        Self::AfterOldPrimaryRejoinCompletion,
        Self::BeforeAuditDerivation,
        Self::AfterAuditDerivation,
        Self::BeforeAuditExport,
        Self::AfterAuditExport,
    ];

    pub fn token(self) -> &'static str {
        match self {
            Self::BeforeDurableControlTransition(kind) => kind.before_token(),
            Self::AfterDurableControlTransition(kind) => kind.after_token(),
            Self::BeforeForensicSourceAcquisition => "s10-forensic-before-source-acquisition",
            Self::AfterForensicSourceRecord => "s10-forensic-after-source-record",
            Self::BeforeForensicFinalization => "s10-forensic-before-finalization",
            Self::AfterForensicFinalization => "s10-forensic-after-finalization",
            Self::BeforeBootstrapTransfer => "s10-bootstrap-before-transfer",
            Self::AfterBootstrapTransfer => "s10-bootstrap-after-transfer",
            Self::BeforeBootstrapControlRecord => "s10-bootstrap-before-control-record",
            Self::AfterBootstrapControlRecord => "s10-bootstrap-after-control-record",
            Self::BeforeBootstrapPostVerification => "s10-bootstrap-before-post-verification",
            Self::AfterBootstrapPostVerification => "s10-bootstrap-after-post-verification",
            Self::BeforeBootstrapCompletion => "s10-bootstrap-before-completion",
            Self::AfterBootstrapCompletion => "s10-bootstrap-after-completion",
            Self::BeforePromotionExternalFence => "s10-promotion-before-external-fence",
            Self::AfterPromotionExternalFence => "s10-promotion-after-external-fence",
            Self::BeforePromotionFenceRecord => "s10-promotion-before-fence-record",
            Self::AfterPromotionFenceRecord => "s10-promotion-after-fence-record",
            Self::BeforePromotionRecord => "s10-promotion-before-record",
            Self::AfterPromotionRecord => "s10-promotion-after-record",
            Self::BeforePromotionPostVerification => "s10-promotion-before-post-verification",
            Self::AfterPromotionPostVerification => "s10-promotion-after-post-verification",
            Self::BeforePromotionPublication => "s10-promotion-before-publication",
            Self::AfterPromotionPublication => "s10-promotion-after-publication",
            Self::BeforePromotionReadmission => "s10-promotion-before-readmission",
            Self::AfterPromotionReadmission => "s10-promotion-after-readmission",
            Self::BeforeOldPrimaryRejoinPlan => "s10-promotion-before-old-primary-rejoin-plan",
            Self::AfterOldPrimaryRejoinPlan => "s10-promotion-after-old-primary-rejoin-plan",
            Self::BeforeOldPrimaryRejoinExecution => "s10-rejoin-before-owner-execution",
            Self::AfterOldPrimaryRejoinExecution => "s10-rejoin-after-owner-execution",
            Self::BeforeOldPrimaryRejoinCompletion => "s10-rejoin-before-completion-record",
            Self::AfterOldPrimaryRejoinCompletion => "s10-rejoin-after-completion-record",
            Self::BeforeAuditDerivation => "s10-audit-before-derivation",
            Self::AfterAuditDerivation => "s10-audit-after-derivation",
            Self::BeforeAuditExport => "s10-audit-before-export",
            Self::AfterAuditExport => "s10-audit-after-export",
        }
    }
}
