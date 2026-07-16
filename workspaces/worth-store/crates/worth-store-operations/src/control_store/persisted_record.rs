use super::{
    BackupMaterializationRecoveryPlan, OperationalControlRecord, OperationalControlRecordKind,
    OperationalOperationId, OperationalTransitionId, OperationalWorkflowKind,
};
use worth_store_physical_backend::ControlMediaFault;
use worth_store_physical_backend::ControlRecoveryObjectHandle;
use worth_store_physical_isolation::{
    BackupCutRecoveryRecord, BackupReachabilityLeaseReleaseRecord,
};

#[derive(Debug)]
pub(crate) enum PersistedControlRecordDecodeDenial {
    InvalidEncoding,
    AllocationFailed,
    Media(ControlMediaFault),
    ReplayBudgetExceeded { required: u64, limit: u64 },
}

pub(crate) struct PersistedOperationalControlRecord {
    pub(crate) authority_identity_fingerprint: [u8; 32],
    pub(crate) operation_id: String,
    pub(crate) transition_id: String,
    pub(crate) kind: PersistedOperationalControlRecordKind,
}

pub(crate) enum PersistedOperationalControlRecordKind {
    WorkflowOpened {
        workflow: PersistedWorkflowKind,
    },
    SourceLeasePersisted {
        cut_identity: [u8; 32],
        object_digest: [u8; 32],
        object_bytes: u64,
    },
    BackupMaterializationOpened {
        cut_identity: [u8; 32],
        target_platform: u8,
        target_bytes: Vec<u8>,
        buffer_bytes: u64,
    },
    BackupMaterializationRecorded {
        manifest_digest: [u8; 32],
    },
    IndependentBackupVerificationRecordedAndSourceLeaseReleased {
        verification_identity: [u8; 32],
        release_recovery_bytes: Vec<u8>,
    },
    BackupAbandoned {
        reason: String,
        released_source_lease: Vec<u8>,
    },
    AuthorizationConsumed {
        authorization_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        operation_tag: u8,
        execution_plan_fingerprint: Option<[u8; 32]>,
        assertion_identity: [u8; 32],
        expires_at: u64,
        replay_same_operation_identity: bool,
    },
    RepairExecutionOpened {
        authorization_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        owner_node_count: u64,
        topology_tag: u8,
    },
    RepairOwnerReceiptPersisted {
        plan_fingerprint: [u8; 32],
        node_fingerprint: [u8; 32],
        receipt_fingerprint: [u8; 32],
        owner_tag: u8,
    },
    RepairOwnerEffectStarted {
        plan_fingerprint: [u8; 32],
        node_fingerprint: [u8; 32],
        owner_tag: u8,
    },
    OperationalOwnerReceiptPersisted {
        workflow: PersistedWorkflowKind,
        plan_fingerprint: [u8; 32],
        receipt_fingerprint: [u8; 32],
        owner_tag: u8,
    },
    RepairDispositionRecorded {
        plan_fingerprint: [u8; 32],
        disposition_tag: u8,
        disposition_basis: [u8; 32],
    },
    RecoveryStagingCompleted {
        authorization_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        execution_plan_fingerprint: [u8; 32],
        staged_media_identity: [u8; 32],
    },
    RecoveryPublicationPrepared {
        operation_tag: u8,
        cutover_plan_fingerprint: [u8; 32],
        publication_plan_fingerprint: [u8; 32],
        publication_identity: [u8; 32],
        candidate_media_identity: [u8; 32],
        fence_identity: [u8; 32],
        fence_plan_fingerprint: [u8; 32],
        authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
        admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
    },
    RecoveryPublicationPending {
        operation_tag: u8,
        cutover_plan_fingerprint: [u8; 32],
        publication_plan_fingerprint: [u8; 32],
        publication_identity: [u8; 32],
        candidate_media_identity: [u8; 32],
        fence_identity: [u8; 32],
        fence_plan_fingerprint: [u8; 32],
        authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
        admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
    },
    RecoveryPublicationDisposition {
        publication_identity: [u8; 32],
        disposition_tag: u8,
        disposition_basis: [u8; 32],
        observed_authority: [u8; 32],
    },
    RecoveryPublicationFenceReleased {
        publication_identity: [u8; 32],
        fence_identity: [u8; 32],
        fence_plan_fingerprint: [u8; 32],
        disposition_tag: u8,
    },
}

#[derive(Clone, Copy)]
pub(crate) enum PersistedWorkflowKind {
    OfflineInspection,
    Backup,
    Restore,
    PointInTimeRecovery,
    Rollback,
    Repair,
    ReplicaBootstrap,
    ReplicaPromotion,
    ForensicAcquisition,
}

impl PersistedOperationalControlRecord {
    pub(crate) fn into_domain(
        self,
        load_recovery_object: impl FnOnce(
            ControlRecoveryObjectHandle,
        )
            -> Result<Vec<u8>, PersistedControlRecordDecodeDenial>,
    ) -> Result<OperationalControlRecord, PersistedControlRecordDecodeDenial> {
        let operation_id = OperationalOperationId::new(self.operation_id)
            .map_err(|_| PersistedControlRecordDecodeDenial::InvalidEncoding)?;
        let transition_id = OperationalTransitionId::new(self.transition_id)
            .map_err(|_| PersistedControlRecordDecodeDenial::InvalidEncoding)?;
        Ok(OperationalControlRecord::from_persisted_parts(
            worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint(
                self.authority_identity_fingerprint,
            ),
            operation_id,
            transition_id,
            self.kind.into_domain(load_recovery_object)?,
        ))
    }
}

impl PersistedOperationalControlRecordKind {
    fn into_domain(
        self,
        load_recovery_object: impl FnOnce(
            ControlRecoveryObjectHandle,
        )
            -> Result<Vec<u8>, PersistedControlRecordDecodeDenial>,
    ) -> Result<OperationalControlRecordKind, PersistedControlRecordDecodeDenial> {
        Ok(match self {
            Self::WorkflowOpened { workflow } => OperationalControlRecordKind::WorkflowOpened {
                workflow: workflow.into(),
            },
            Self::SourceLeasePersisted {
                cut_identity,
                object_digest,
                object_bytes,
            } => {
                let recovery_object =
                    ControlRecoveryObjectHandle::from_persisted(object_digest, object_bytes)
                        .ok_or(PersistedControlRecordDecodeDenial::InvalidEncoding)?;
                let recovery_bytes = load_recovery_object(recovery_object)?;
                let recovery = BackupCutRecoveryRecord::recover(&recovery_bytes)
                    .map_err(|_| PersistedControlRecordDecodeDenial::InvalidEncoding)?;
                if recovery.cut_identity() != cut_identity {
                    return Err(PersistedControlRecordDecodeDenial::InvalidEncoding);
                }
                OperationalControlRecordKind::SourceLeasePersisted {
                    recovery: Box::new(recovery),
                    recovery_object,
                }
            }
            Self::BackupMaterializationOpened {
                cut_identity,
                target_platform,
                target_bytes,
                buffer_bytes,
            } => {
                let target_parent = super::operational_media_path::decode_operational_media_path(
                    target_platform,
                    &target_bytes,
                )
                .map_err(|_| PersistedControlRecordDecodeDenial::InvalidEncoding)?;
                let plan = BackupMaterializationRecoveryPlan::from_persisted(
                    cut_identity,
                    target_parent,
                    buffer_bytes,
                )
                .map_err(|denial| match denial {
                    super::BackupMaterializationRecoveryPlanDenial::AllocationFailed => {
                        PersistedControlRecordDecodeDenial::AllocationFailed
                    }
                    _ => PersistedControlRecordDecodeDenial::InvalidEncoding,
                })?;
                OperationalControlRecordKind::BackupMaterializationOpened { plan }
            }
            Self::BackupMaterializationRecorded { manifest_digest } => {
                OperationalControlRecordKind::BackupMaterializationRecorded { manifest_digest }
            },
            Self::IndependentBackupVerificationRecordedAndSourceLeaseReleased {
                verification_identity,
                release_recovery_bytes,
            } => OperationalControlRecordKind::IndependentBackupVerificationRecordedAndSourceLeaseReleased {
                verification_identity,
                release: BackupReachabilityLeaseReleaseRecord::recover(&release_recovery_bytes)
                    .map_err(|_| PersistedControlRecordDecodeDenial::InvalidEncoding)?,
            },
            Self::BackupAbandoned {
                reason,
                released_source_lease,
            } => OperationalControlRecordKind::BackupAbandoned {
                reason,
                released_source_lease: BackupReachabilityLeaseReleaseRecord::recover(
                    &released_source_lease,
                )
                    .map_err(|_| PersistedControlRecordDecodeDenial::InvalidEncoding)?,
            },
            Self::AuthorizationConsumed {
                authorization_identity,
                plan_fingerprint,
                operation_tag,
                execution_plan_fingerprint,
                assertion_identity,
                expires_at,
                replay_same_operation_identity,
            } => OperationalControlRecordKind::AuthorizationConsumed {
                authorization_identity,
                plan_fingerprint,
                operation_tag,
                execution_plan_fingerprint,
                assertion_identity,
                expires_at,
                replay_same_operation_identity,
            },
            Self::RepairExecutionOpened { authorization_identity, plan_fingerprint, owner_node_count,
                topology_tag } =>
                OperationalControlRecordKind::RepairExecutionOpened {
                    authorization_identity, plan_fingerprint, owner_node_count, topology_tag },
            Self::RepairOwnerReceiptPersisted { plan_fingerprint, node_fingerprint,
                receipt_fingerprint, owner_tag } =>
                OperationalControlRecordKind::RepairOwnerReceiptPersisted {
                    plan_fingerprint, node_fingerprint, receipt_fingerprint, owner_tag },
            Self::RepairOwnerEffectStarted { plan_fingerprint, node_fingerprint, owner_tag } =>
                OperationalControlRecordKind::RepairOwnerEffectStarted {
                    plan_fingerprint, node_fingerprint, owner_tag },
            Self::OperationalOwnerReceiptPersisted { workflow, plan_fingerprint,
                receipt_fingerprint, owner_tag } =>
                OperationalControlRecordKind::OperationalOwnerReceiptPersisted {
                    workflow: workflow.into(), plan_fingerprint,
                    receipt_fingerprint, owner_tag },
            Self::RepairDispositionRecorded { plan_fingerprint, disposition_tag,
                disposition_basis } =>
                OperationalControlRecordKind::RepairDispositionRecorded {
                    plan_fingerprint, disposition_tag, disposition_basis },
            Self::RecoveryStagingCompleted { authorization_identity, plan_fingerprint,
                execution_plan_fingerprint, staged_media_identity } =>
                OperationalControlRecordKind::RecoveryStagingCompleted {
                    authorization_identity, plan_fingerprint, execution_plan_fingerprint,
                    staged_media_identity },
            Self::RecoveryPublicationPending { operation_tag, cutover_plan_fingerprint,
                publication_plan_fingerprint, publication_identity, candidate_media_identity,
                fence_identity, fence_plan_fingerprint, authority_posture, admission_policy } =>
                OperationalControlRecordKind::RecoveryPublicationPending {
                    binding: super::control_record::RecoveryPublicationControlBinding::from_persisted(
                        operation_tag, cutover_plan_fingerprint, publication_plan_fingerprint,
                        publication_identity, candidate_media_identity, fence_identity,
                        fence_plan_fingerprint, authority_posture, admission_policy,
                    )
                },
            Self::RecoveryPublicationPrepared { operation_tag, cutover_plan_fingerprint,
                publication_plan_fingerprint, publication_identity, candidate_media_identity,
                fence_identity, fence_plan_fingerprint, authority_posture, admission_policy } =>
                OperationalControlRecordKind::RecoveryPublicationPrepared {
                    binding: super::control_record::RecoveryPublicationControlBinding::from_persisted(
                        operation_tag, cutover_plan_fingerprint, publication_plan_fingerprint,
                        publication_identity, candidate_media_identity, fence_identity,
                        fence_plan_fingerprint, authority_posture, admission_policy,
                    )
                },
            Self::RecoveryPublicationDisposition { publication_identity, disposition_tag,
                disposition_basis, observed_authority } =>
                OperationalControlRecordKind::RecoveryPublicationDisposition {
                    publication_identity, disposition_tag, disposition_basis,
                    observed_authority: worth_store_authority::StoreCurrentAuthorityIdentity::from_persisted_fingerprint(
                        observed_authority), },
            Self::RecoveryPublicationFenceReleased { publication_identity, fence_identity,
                fence_plan_fingerprint, disposition_tag } =>
                OperationalControlRecordKind::RecoveryPublicationFenceReleased {
                    publication_identity, fence_identity, fence_plan_fingerprint, disposition_tag,
                },
        })
    }
}

impl From<OperationalWorkflowKind> for PersistedWorkflowKind {
    fn from(value: OperationalWorkflowKind) -> Self {
        match value {
            OperationalWorkflowKind::OfflineInspection => Self::OfflineInspection,
            OperationalWorkflowKind::Backup => Self::Backup,
            OperationalWorkflowKind::Restore => Self::Restore,
            OperationalWorkflowKind::PointInTimeRecovery => Self::PointInTimeRecovery,
            OperationalWorkflowKind::Rollback => Self::Rollback,
            OperationalWorkflowKind::Repair => Self::Repair,
            OperationalWorkflowKind::ReplicaBootstrap => Self::ReplicaBootstrap,
            OperationalWorkflowKind::ReplicaPromotion => Self::ReplicaPromotion,
            OperationalWorkflowKind::ForensicAcquisition => Self::ForensicAcquisition,
        }
    }
}

impl From<PersistedWorkflowKind> for OperationalWorkflowKind {
    fn from(value: PersistedWorkflowKind) -> Self {
        match value {
            PersistedWorkflowKind::OfflineInspection => Self::OfflineInspection,
            PersistedWorkflowKind::Backup => Self::Backup,
            PersistedWorkflowKind::Restore => Self::Restore,
            PersistedWorkflowKind::PointInTimeRecovery => Self::PointInTimeRecovery,
            PersistedWorkflowKind::Rollback => Self::Rollback,
            PersistedWorkflowKind::Repair => Self::Repair,
            PersistedWorkflowKind::ReplicaBootstrap => Self::ReplicaBootstrap,
            PersistedWorkflowKind::ReplicaPromotion => Self::ReplicaPromotion,
            PersistedWorkflowKind::ForensicAcquisition => Self::ForensicAcquisition,
        }
    }
}
