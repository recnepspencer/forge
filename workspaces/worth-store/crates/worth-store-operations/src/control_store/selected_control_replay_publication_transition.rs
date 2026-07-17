use worth_store_authority::StoreCurrentAuthorityIdentity;

use super::{
    recovery_publication_control_replay::{
        observe_disposition, observe_fence_released, observe_prepared, observe_published,
    },
    recovery_staging_control_replay::consume_completed_for_publication,
    selected_control_replay_contract::SelectedControlReplayDenial,
    OperationalControlHistoryViolation, OperationalControlRecordKind, OperationalOperationId,
    SelectedControlReplay,
};

impl SelectedControlReplay {
    pub(super) fn observe_recovery_publication_transition(
        &mut self,
        record_index: u64,
        operation: &OperationalOperationId,
        authority: StoreCurrentAuthorityIdentity,
        kind: &OperationalControlRecordKind,
    ) -> Result<bool, SelectedControlReplayDenial> {
        match kind {
            OperationalControlRecordKind::RecoveryPublicationPrepared { binding } => {
                consume_completed_for_publication(
                    &mut self.recovery_staging,
                    operation,
                    binding.operation_tag(),
                )
                .map_err(|kind| replay_denial(record_index, operation, kind))?;
                observe_prepared(
                    &mut self.recovery_publications,
                    operation,
                    authority,
                    binding.clone(),
                )
                .map_err(|kind| replay_denial(record_index, operation, kind))?;
            }
            OperationalControlRecordKind::RecoveryPublicationPending { binding } => {
                observe_published(
                    &mut self.recovery_publications,
                    operation,
                    authority,
                    binding.clone(),
                )
                .map_err(|kind| replay_denial(record_index, operation, kind))?;
            }
            OperationalControlRecordKind::RecoveryPublicationDisposition {
                publication_identity,
                disposition_tag,
                disposition_basis,
                observed_authority,
            } => observe_disposition(
                &mut self.recovery_publications,
                operation,
                authority,
                *publication_identity,
                *disposition_tag,
                *disposition_basis,
                *observed_authority,
            )
            .map_err(|kind| replay_denial(record_index, operation, kind))?,
            OperationalControlRecordKind::RecoveryPublicationFenceReleased {
                publication_identity,
                fence_identity,
                fence_plan_fingerprint,
                disposition_tag,
            } => observe_fence_released(
                &mut self.recovery_publications,
                operation,
                authority,
                *publication_identity,
                *fence_identity,
                *fence_plan_fingerprint,
                *disposition_tag,
            )
            .map_err(|kind| replay_denial(record_index, operation, kind))?,
            _ => return Ok(false),
        }
        Ok(true)
    }
}

fn replay_denial(
    record_index: u64,
    operation: &OperationalOperationId,
    kind: super::OperationalControlHistoryViolationKind,
) -> SelectedControlReplayDenial {
    SelectedControlReplayDenial::Invalid(OperationalControlHistoryViolation::new(
        record_index,
        operation.clone(),
        kind,
    ))
}
