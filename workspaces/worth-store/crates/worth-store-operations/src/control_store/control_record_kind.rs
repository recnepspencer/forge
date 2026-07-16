use super::BackupMaterializationRecoveryPlan;
use worth_store_physical_backend::ControlRecoveryObjectHandle;
use worth_store_physical_isolation::{
    BackupCutRecoveryRecord, BackupReachabilityLeaseReleaseRecord,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalWorkflowKind {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationalControlRecordKind {
    WorkflowOpened {
        workflow: OperationalWorkflowKind,
    },
    SourceLeasePersisted {
        recovery: Box<BackupCutRecoveryRecord>,
        recovery_object: ControlRecoveryObjectHandle,
    },
    BackupMaterializationOpened {
        plan: BackupMaterializationRecoveryPlan,
    },
    BackupMaterializationRecorded {
        manifest_digest: [u8; 32],
    },
    IndependentBackupVerificationRecordedAndSourceLeaseReleased {
        verification_identity: [u8; 32],
        release: BackupReachabilityLeaseReleaseRecord,
    },
    BackupAbandoned {
        reason: String,
        released_source_lease: BackupReachabilityLeaseReleaseRecord,
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
        binding: RecoveryPublicationControlBinding,
    },
    RecoveryPublicationPending {
        binding: RecoveryPublicationControlBinding,
    },
    RecoveryPublicationDisposition {
        publication_identity: [u8; 32],
        disposition_tag: u8,
        disposition_basis: [u8; 32],
        observed_authority: worth_store_authority::StoreCurrentAuthorityIdentity,
    },
    RecoveryPublicationFenceReleased {
        publication_identity: [u8; 32],
        fence_identity: [u8; 32],
        fence_plan_fingerprint: [u8; 32],
        disposition_tag: u8,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPublicationControlBinding {
    operation_tag: u8,
    cutover_plan_fingerprint: [u8; 32],
    publication_plan_fingerprint: [u8; 32],
    publication_identity: [u8; 32],
    candidate_media_identity: [u8; 32],
    fence_identity: [u8; 32],
    fence_plan_fingerprint: [u8; 32],
    authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
    admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
}

impl RecoveryPublicationControlBinding {
    pub(crate) fn from_prepared_cutover(
        operation_tag: u8,
        fence: worth_store_authority::RecoveryWriteFenceReceipt,
        publication: &worth_store_physical_isolation::RecoveryPublicationLoweredPlan,
        authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
        admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
    ) -> Self {
        Self {
            operation_tag,
            cutover_plan_fingerprint: fence.cutover_plan_fingerprint(),
            publication_plan_fingerprint: publication.fingerprint(),
            publication_identity: publication.publication_identity(),
            candidate_media_identity: publication.candidate_media_identity(),
            fence_identity: fence.fence_identity(),
            fence_plan_fingerprint: fence.plan_fingerprint(),
            authority_posture,
            admission_policy,
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn from_persisted(
        operation_tag: u8,
        cutover_plan_fingerprint: [u8; 32],
        publication_plan_fingerprint: [u8; 32],
        publication_identity: [u8; 32],
        candidate_media_identity: [u8; 32],
        fence_identity: [u8; 32],
        fence_plan_fingerprint: [u8; 32],
        authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
        admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
    ) -> Self {
        Self {
            operation_tag,
            cutover_plan_fingerprint,
            publication_plan_fingerprint,
            publication_identity,
            candidate_media_identity,
            fence_identity,
            fence_plan_fingerprint,
            authority_posture,
            admission_policy,
        }
    }

    pub const fn operation_tag(&self) -> u8 {
        self.operation_tag
    }
    pub const fn cutover_plan_fingerprint(&self) -> [u8; 32] {
        self.cutover_plan_fingerprint
    }
    pub const fn publication_plan_fingerprint(&self) -> [u8; 32] {
        self.publication_plan_fingerprint
    }
    pub const fn publication_identity(&self) -> [u8; 32] {
        self.publication_identity
    }
    pub const fn candidate_media_identity(&self) -> [u8; 32] {
        self.candidate_media_identity
    }
    pub const fn fence_identity(&self) -> [u8; 32] {
        self.fence_identity
    }
    pub const fn fence_plan_fingerprint(&self) -> [u8; 32] {
        self.fence_plan_fingerprint
    }
    pub const fn authority_posture(
        &self,
    ) -> worth_store_authority::RecoveryAuthorityAdmissionPosture {
        self.authority_posture
    }
    pub const fn admission_policy(
        &self,
    ) -> worth_store_authority::RecoveryAuthorityAdmissionPolicy {
        self.admission_policy
    }
}
