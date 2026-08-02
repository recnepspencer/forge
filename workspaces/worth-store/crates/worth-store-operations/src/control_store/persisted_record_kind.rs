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
    ReplicaBootstrapTransferRecorded {
        authorization_plan_fingerprint: [u8; 32],
        execution_plan_fingerprint: [u8; 32],
        receipt_identity: [u8; 32],
        durable_target_identity: [u8; 32],
        source_lease_identity: [u8; 32],
        source_bytes_read: u64,
        output_bytes_written: u64,
        backend_requests: u64,
        maximum_resident_buffer_bytes: u64,
    },
    ReplicaBootstrapCompleted {
        receipt_identity: [u8; 32],
        verification_identity: [u8; 32],
        source_lease_identity: [u8; 32],
    },
    ReplicaBootstrapAbandoned {
        receipt_identity: [u8; 32],
        reason: String,
        source_lease_identity: [u8; 32],
    },
    ReplicaPromotionFenceRecorded {
        authorization_plan_fingerprint: [u8; 32],
        execution_plan_fingerprint: [u8; 32],
        fence_identity: [u8; 32],
        promoted_epoch: u64,
    },
    ReplicaPromotionRecorded {
        authorization_plan_fingerprint: [u8; 32],
        execution_plan_fingerprint: [u8; 32],
        receipt_identity: [u8; 32],
        fence_identity: [u8; 32],
        promoted_epoch: u64,
    },
    ReplicaPromotionPublished {
        receipt_identity: [u8; 32],
        verification_identity: [u8; 32],
        publication_identity: [u8; 32],
        target_identity: [u8; 32],
        promoted_epoch: u64,
    },
    ReplicaPromotionReadmitted {
        publication_identity: [u8; 32],
        serve_lease_identity: [u8; 32],
        serving_epoch: u64,
    },
    OldPrimaryRejoinPlanned {
        promotion_receipt_identity: [u8; 32],
        rejoin_plan_fingerprint: [u8; 32],
        disposition_tag: u8,
    },
    OldPrimaryRejoinCompleted {
        rejoin_plan_fingerprint: [u8; 32],
        rejoin_receipt_identity: [u8; 32],
        forensic_retention_identity: [u8; 32],
        rebootstrap_target_identity: [u8; 32],
        disposition_tag: u8,
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
