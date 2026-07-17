use super::replica_operation_control_replay::{
    observe_bootstrap_terminal, observe_bootstrap_transfer, observe_promotion_fence,
    observe_promotion_publication, observe_promotion_readmission, observe_promotion_receipt,
};
use super::replica_operation_rejoin_replay::{
    observe_old_primary_rejoin, observe_old_primary_rejoin_completion,
};
use super::{
    OperationalControlHistoryViolation, OperationalControlRecordKind, OperationalOperationId,
    RecoveredReplicaBootstrapDisposition, SelectedControlReplay, SelectedControlReplayDenial,
};

impl SelectedControlReplay {
    pub(super) fn observe_replica_transition(
        &mut self,
        record_index: u64,
        operation: &OperationalOperationId,
        kind: &OperationalControlRecordKind,
    ) -> Result<bool, SelectedControlReplayDenial> {
        let result = match kind {
            OperationalControlRecordKind::ReplicaBootstrapTransferRecorded {
                authorization_plan_fingerprint,
                execution_plan_fingerprint,
                receipt_identity,
                durable_target_identity,
                source_lease_identity,
                source_bytes_read,
                output_bytes_written,
                backend_requests,
                maximum_resident_buffer_bytes,
            } => {
                let Some(counters) =
                    worth_store_replication::ReplicaBootstrapExecutionCounters::measured(
                        *source_bytes_read,
                        *output_bytes_written,
                        *backend_requests,
                        *maximum_resident_buffer_bytes,
                    )
                else {
                    return Err(SelectedControlReplayDenial::Invalid(
                        OperationalControlHistoryViolation::new(
                            record_index,
                            operation.clone(),
                            super::OperationalControlHistoryViolationKind::ReplicaOperationBindingMismatch,
                        ),
                    ));
                };
                observe_bootstrap_transfer(
                    &mut self.replica_bootstraps,
                    operation,
                    *authorization_plan_fingerprint,
                    *execution_plan_fingerprint,
                    *receipt_identity,
                    *durable_target_identity,
                    *source_lease_identity,
                    counters,
                )
            }
            OperationalControlRecordKind::ReplicaBootstrapCompleted {
                receipt_identity,
                verification_identity,
                source_lease_identity,
            } => observe_bootstrap_terminal(
                &mut self.replica_bootstraps,
                operation,
                *receipt_identity,
                *source_lease_identity,
                RecoveredReplicaBootstrapDisposition::Completed {
                    verification_identity: *verification_identity,
                },
            ),
            OperationalControlRecordKind::ReplicaBootstrapAbandoned {
                receipt_identity,
                source_lease_identity,
                ..
            } => observe_bootstrap_terminal(
                &mut self.replica_bootstraps,
                operation,
                *receipt_identity,
                *source_lease_identity,
                RecoveredReplicaBootstrapDisposition::Abandoned,
            ),
            OperationalControlRecordKind::ReplicaPromotionFenceRecorded {
                authorization_plan_fingerprint,
                execution_plan_fingerprint,
                fence_identity,
                promoted_epoch,
            } => observe_promotion_fence(
                &mut self.replica_promotions,
                operation,
                *authorization_plan_fingerprint,
                *execution_plan_fingerprint,
                *fence_identity,
                *promoted_epoch,
            ),
            OperationalControlRecordKind::ReplicaPromotionRecorded {
                authorization_plan_fingerprint,
                execution_plan_fingerprint,
                receipt_identity,
                fence_identity,
                promoted_epoch,
            } => observe_promotion_receipt(
                &mut self.replica_promotions,
                operation,
                *authorization_plan_fingerprint,
                *execution_plan_fingerprint,
                *receipt_identity,
                *fence_identity,
                *promoted_epoch,
            ),
            OperationalControlRecordKind::ReplicaPromotionPublished {
                receipt_identity,
                verification_identity,
                publication_identity,
                target_identity,
                promoted_epoch,
            } => observe_promotion_publication(
                &mut self.replica_promotions,
                operation,
                *receipt_identity,
                *verification_identity,
                *publication_identity,
                *target_identity,
                *promoted_epoch,
            ),
            OperationalControlRecordKind::ReplicaPromotionReadmitted {
                publication_identity,
                serve_lease_identity,
                serving_epoch,
            } => observe_promotion_readmission(
                &mut self.replica_promotions,
                operation,
                *publication_identity,
                *serve_lease_identity,
                *serving_epoch,
            ),
            OperationalControlRecordKind::OldPrimaryRejoinPlanned {
                promotion_receipt_identity,
                rejoin_plan_fingerprint,
                disposition_tag,
            } => observe_old_primary_rejoin(
                &mut self.replica_promotions,
                operation,
                *promotion_receipt_identity,
                *rejoin_plan_fingerprint,
                *disposition_tag,
            ),
            OperationalControlRecordKind::OldPrimaryRejoinCompleted {
                rejoin_plan_fingerprint,
                rejoin_receipt_identity,
                forensic_retention_identity,
                rebootstrap_target_identity,
                disposition_tag,
            } => observe_old_primary_rejoin_completion(
                &mut self.replica_promotions,
                operation,
                *rejoin_plan_fingerprint,
                *rejoin_receipt_identity,
                *forensic_retention_identity,
                *rebootstrap_target_identity,
                *disposition_tag,
            ),
            _ => return Ok(false),
        };
        result.map_err(|kind| {
            SelectedControlReplayDenial::Invalid(OperationalControlHistoryViolation::new(
                record_index,
                operation.clone(),
                kind,
            ))
        })?;
        Ok(true)
    }
}
