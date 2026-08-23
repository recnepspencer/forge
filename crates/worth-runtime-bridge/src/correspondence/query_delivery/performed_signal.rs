use super::super::BridgeCorrespondenceDeliveryReceipt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgePerformedSignalInvalidationDenial {
    NoPerformedSignalExecution,
    NoPreparedSignalInvalidation,
    SignalTargetBindingMismatch,
    PerformedTargetMismatch,
    TriggeringCorrespondenceMismatch,
}

pub struct BridgePerformedSignalInvalidation {
    truth: super::super::BridgeDeliveredCorrespondenceChangeSet,
    query_binding_identity: std::sync::Arc<str>,
    query_capability_identity: u64,
    performed: worth_signal::facade::adapters::SignalInvalidationExecutionReceipt,
}

impl BridgePerformedSignalInvalidation {
    pub fn performed(&self) -> &worth_signal::facade::adapters::SignalInvalidationExecutionReceipt {
        &self.performed
    }

    pub fn query_binding_identity(&self) -> &str {
        &self.query_binding_identity
    }

    pub const fn query_capability_identity(&self) -> u64 {
        self.query_capability_identity
    }

    pub(crate) fn retains_truth(
        &self,
        candidate: &super::super::BridgeDeliveredCorrespondenceChangeSet,
    ) -> bool {
        self.truth.basis() == candidate.basis()
            && self.truth.dependency() == candidate.dependency()
            && self.truth.commit_identity() == candidate.commit_identity()
            && self.truth.patch_identity() == candidate.patch_identity()
            && self.truth.snapshot_identity() == candidate.snapshot_identity()
            && self.truth.branch_identity() == candidate.branch_identity()
            && self.truth.changes() == candidate.changes()
    }
}

pub fn bind_performed_signal_invalidation(
    truth: &BridgeCorrespondenceDeliveryReceipt,
    decision: &mut crate::conditional_execution::BridgeConditionalDecisionEvidence,
) -> Result<BridgePerformedSignalInvalidation, BridgePerformedSignalInvalidationDenial> {
    if !decision.retains_triggering_correspondence(truth.change_set()) {
        return Err(BridgePerformedSignalInvalidationDenial::TriggeringCorrespondenceMismatch);
    }
    let prepared = truth
        .prepared_signal_invalidation()
        .ok_or(BridgePerformedSignalInvalidationDenial::NoPreparedSignalInvalidation)?;
    let output_aspect = decision.signal().output_aspect();
    if !prepared.has_unique_target_bindings()
        || !prepared.retains_target(
            decision.signal_graph_instance_id(),
            decision.signal_node(),
            output_aspect,
        )
    {
        return Err(BridgePerformedSignalInvalidationDenial::SignalTargetBindingMismatch);
    }
    let performed = decision
        .take_performed_signal_invalidation()
        .ok_or(BridgePerformedSignalInvalidationDenial::NoPerformedSignalExecution)?;
    if !performed
        .retains_executed_target(decision.signal_graph_instance_id(), decision.signal_node())
    {
        return Err(BridgePerformedSignalInvalidationDenial::PerformedTargetMismatch);
    }
    Ok(BridgePerformedSignalInvalidation {
        truth: truth.change_set().clone(),
        query_binding_identity: std::sync::Arc::from(decision.query_binding_identity()),
        query_capability_identity: decision.query_capability_identity(),
        performed,
    })
}
