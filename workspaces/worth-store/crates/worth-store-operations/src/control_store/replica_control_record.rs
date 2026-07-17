use worth_store_authority::StoreCurrentAuthorityIdentity;

use super::{
    OperationalControlRecord, OperationalControlRecordKind, OperationalOperationId,
    OperationalTransitionId,
};

impl OperationalControlRecord {
    pub(crate) fn replica_bootstrap_transfer_recorded(
        authority_identity: StoreCurrentAuthorityIdentity,
        operation_id: OperationalOperationId,
        transition_id: OperationalTransitionId,
        authorization_plan_fingerprint: [u8; 32],
        receipt: &worth_store_replication::ReplicaBootstrapReceipt,
    ) -> Self {
        let counters = receipt.execution_counters();
        Self {
            authority_identity,
            operation_id,
            transition_id,
            kind: OperationalControlRecordKind::ReplicaBootstrapTransferRecorded {
                authorization_plan_fingerprint,
                execution_plan_fingerprint: receipt.plan_fingerprint(),
                receipt_identity: receipt.receipt_identity(),
                durable_target_identity: receipt.durable_target_identity(),
                source_lease_identity: receipt.retained_source_lease_identity(),
                source_bytes_read: counters.source_bytes_read(),
                output_bytes_written: counters.output_bytes_written(),
                backend_requests: counters.backend_requests(),
                maximum_resident_buffer_bytes: counters.maximum_resident_buffer_bytes(),
            },
        }
    }

    pub(crate) fn replica_bootstrap_completed(
        authority_identity: StoreCurrentAuthorityIdentity,
        operation_id: OperationalOperationId,
        transition_id: OperationalTransitionId,
        receipt: &worth_store_replication::ReplicaBootstrapReceipt,
        verification: &worth_store_offline_verifier::IndependentlyVerifiedReplicaTarget,
    ) -> Self {
        Self {
            authority_identity,
            operation_id,
            transition_id,
            kind: OperationalControlRecordKind::ReplicaBootstrapCompleted {
                receipt_identity: receipt.receipt_identity(),
                verification_identity: verification.verification_identity(),
                source_lease_identity: receipt.retained_source_lease_identity(),
            },
        }
    }

    pub(crate) fn replica_bootstrap_abandoned(
        authority_identity: StoreCurrentAuthorityIdentity,
        operation_id: OperationalOperationId,
        transition_id: OperationalTransitionId,
        receipt: &worth_store_replication::ReplicaBootstrapReceipt,
        reason: String,
    ) -> Self {
        Self {
            authority_identity,
            operation_id,
            transition_id,
            kind: OperationalControlRecordKind::ReplicaBootstrapAbandoned {
                receipt_identity: receipt.receipt_identity(),
                reason,
                source_lease_identity: receipt.retained_source_lease_identity(),
            },
        }
    }

    pub(crate) fn replica_promotion_fence_recorded(
        authority_identity: StoreCurrentAuthorityIdentity,
        operation_id: OperationalOperationId,
        transition_id: OperationalTransitionId,
        authorization_plan_fingerprint: [u8; 32],
        execution_plan_fingerprint: [u8; 32],
        fence: worth_store_authority::FenceProof,
    ) -> Self {
        Self {
            authority_identity,
            operation_id,
            transition_id,
            kind: OperationalControlRecordKind::ReplicaPromotionFenceRecorded {
                authorization_plan_fingerprint,
                execution_plan_fingerprint,
                fence_identity: fence.fence_identity(),
                promoted_epoch: fence.promoted_epoch().get(),
            },
        }
    }

    pub(crate) fn replica_promotion_recorded(
        authority_identity: StoreCurrentAuthorityIdentity,
        operation_id: OperationalOperationId,
        transition_id: OperationalTransitionId,
        authorization_plan_fingerprint: [u8; 32],
        receipt: &worth_store_replication::ReplicaPromotionReceipt,
    ) -> Self {
        Self {
            authority_identity,
            operation_id,
            transition_id,
            kind: OperationalControlRecordKind::ReplicaPromotionRecorded {
                authorization_plan_fingerprint,
                execution_plan_fingerprint: receipt.plan_fingerprint(),
                receipt_identity: receipt.receipt_identity(),
                fence_identity: receipt.fence_identity(),
                promoted_epoch: receipt.promoted_epoch().get(),
            },
        }
    }

    pub(crate) fn replica_promotion_published(
        authority_identity: StoreCurrentAuthorityIdentity,
        operation_id: OperationalOperationId,
        transition_id: OperationalTransitionId,
        receipt: &worth_store_replication::ReplicaPromotionReceipt,
        verification_identity: [u8; 32],
        publication_identity: [u8; 32],
    ) -> Self {
        Self {
            authority_identity,
            operation_id,
            transition_id,
            kind: OperationalControlRecordKind::ReplicaPromotionPublished {
                receipt_identity: receipt.receipt_identity(),
                verification_identity,
                publication_identity,
                target_identity: receipt.durable_target_identity(),
                promoted_epoch: receipt.promoted_epoch().get(),
            },
        }
    }

    pub(crate) fn replica_promotion_readmitted(
        authority_identity: StoreCurrentAuthorityIdentity,
        operation_id: OperationalOperationId,
        transition_id: OperationalTransitionId,
        publication_identity: [u8; 32],
        serve_lease: worth_store_authority::PrimaryServeLease,
    ) -> Self {
        Self {
            authority_identity,
            operation_id,
            transition_id,
            kind: OperationalControlRecordKind::ReplicaPromotionReadmitted {
                publication_identity,
                serve_lease_identity: serve_lease.lease_identity(),
                serving_epoch: serve_lease.epoch(),
            },
        }
    }

    pub(crate) fn old_primary_rejoin_planned(
        authority_identity: StoreCurrentAuthorityIdentity,
        operation_id: OperationalOperationId,
        transition_id: OperationalTransitionId,
        promotion_receipt_identity: [u8; 32],
        plan: &worth_store_replication::OldPrimaryRejoinPlan,
    ) -> Self {
        Self {
            authority_identity,
            operation_id,
            transition_id,
            kind: OperationalControlRecordKind::OldPrimaryRejoinPlanned {
                promotion_receipt_identity,
                rejoin_plan_fingerprint: plan.fingerprint(),
                disposition_tag: plan.disposition() as u8,
            },
        }
    }

    pub(crate) fn old_primary_rejoin_completed(
        authority_identity: StoreCurrentAuthorityIdentity,
        operation_id: OperationalOperationId,
        transition_id: OperationalTransitionId,
        receipt: &worth_store_replication::OldPrimaryRejoinReceipt,
    ) -> Self {
        Self {
            authority_identity,
            operation_id,
            transition_id,
            kind: OperationalControlRecordKind::OldPrimaryRejoinCompleted {
                rejoin_plan_fingerprint: receipt.plan_fingerprint(),
                rejoin_receipt_identity: receipt.receipt_identity(),
                forensic_retention_identity: receipt
                    .forensic_retention_identity()
                    .unwrap_or([0; 32]),
                rebootstrap_target_identity: receipt
                    .rebootstrap_target_identity()
                    .unwrap_or([0; 32]),
                disposition_tag: receipt.disposition() as u8,
            },
        }
    }
}
