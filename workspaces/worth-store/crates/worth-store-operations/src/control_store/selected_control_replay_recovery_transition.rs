use super::{
    recovery_staging_control_replay::observe_owner_receipt,
    selected_control_replay_contract::SelectedControlReplayDenial,
    OperationalControlHistoryViolation, OperationalOperationId, OperationalWorkflowKind,
    SelectedControlReplay,
};

impl SelectedControlReplay {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn observe_recovery_owner_receipt(
        &mut self,
        record_index: u64,
        operation: &OperationalOperationId,
        workflow: OperationalWorkflowKind,
        plan_fingerprint: [u8; 32],
        receipt_fingerprint: [u8; 32],
        owner_tag: u8,
    ) -> Result<(), SelectedControlReplayDenial> {
        observe_owner_receipt(
            &mut self.recovery_staging,
            operation,
            workflow,
            plan_fingerprint,
            receipt_fingerprint,
            owner_tag,
        )
        .map_err(|kind| {
            SelectedControlReplayDenial::Invalid(OperationalControlHistoryViolation::new(
                record_index,
                operation.clone(),
                kind,
            ))
        })
    }
}
