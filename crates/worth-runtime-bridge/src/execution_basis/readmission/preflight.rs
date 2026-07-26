use std::sync::Arc;

use crate::execution_basis::{
    BridgeExecutionBasisReadmissionCounters, BridgeExecutionBasisReadmissionDenialKind,
    BridgeExecutionBasisReadmissionDenied, BridgeExecutionBasisReadmissionYielded,
    BridgeExecutionBasisSignalTerminal, BridgeExecutionBasisTerminalDisposition,
    BridgeYieldedExecutionBasis,
};
use crate::facade::RuntimeBridge;

#[must_use = "validated yielded-basis authority must be readmitted or recovered"]
pub struct BridgeYieldedExecutionBasisPreflight {
    yielded: BridgeYieldedExecutionBasis,
    operation_binding_identity: Arc<str>,
    counters: BridgeExecutionBasisReadmissionCounters,
}

pub(crate) fn preflight_yielded_execution_basis(
    runtime: &RuntimeBridge,
    yielded: BridgeYieldedExecutionBasis,
    operation_binding_identity: &str,
) -> Result<BridgeYieldedExecutionBasisPreflight, BridgeExecutionBasisReadmissionDenied> {
    let mut counters = BridgeExecutionBasisReadmissionCounters::default();
    counters.checked_preflight();
    let denial = if yielded.basis.bridge_runtime_key != runtime.signal_runtime_key {
        Some((
            BridgeExecutionBasisReadmissionDenialKind::ForeignRuntime,
            "yielded Bridge basis belongs to a different runtime",
        ))
    } else if yielded.basis.authoritative_source_profile != runtime.authoritative_source_profile {
        Some((
            BridgeExecutionBasisReadmissionDenialKind::SourceProfileMismatch,
            "yielded Bridge source profile no longer matches the runtime",
        ))
    } else if yielded.managed_intent().operation_binding_identity() != operation_binding_identity {
        Some((
            BridgeExecutionBasisReadmissionDenialKind::OperationBindingMismatch,
            "yielded Bridge basis belongs to a different operation binding",
        ))
    } else if !yield_posture_is_readmittable(&yielded) {
        Some((
            BridgeExecutionBasisReadmissionDenialKind::YieldPostureMismatch,
            "Bridge basis was not cleanly yielded with its Signal request and reservation released",
        ))
    } else {
        None
    };
    if let Some((kind, detail)) = denial {
        return Err(BridgeExecutionBasisReadmissionDenied::new(
            kind, detail, yielded, counters,
        ));
    }
    Ok(BridgeYieldedExecutionBasisPreflight {
        yielded,
        operation_binding_identity: Arc::from(operation_binding_identity),
        counters,
    })
}

fn yield_posture_is_readmittable(yielded: &BridgeYieldedExecutionBasis) -> bool {
    let receipt = yielded.receipt();
    receipt.disposition() == BridgeExecutionBasisTerminalDisposition::Yielded
        && receipt.signal_terminal() == BridgeExecutionBasisSignalTerminal::Cancelled
        && receipt.signal_transition_performed()
        && receipt.reservation_released()
        && receipt.basis_identity() == yielded.basis_identity()
}

impl BridgeYieldedExecutionBasisPreflight {
    pub fn step_contract(&self) -> &crate::execution_basis::BridgeManagedExecutionStepContract {
        self.yielded.step_contract()
    }

    pub fn yielded_receipt(
        &self,
    ) -> &crate::execution_basis::BridgeExecutionBasisFinalizationReceipt {
        self.yielded.receipt()
    }

    pub fn into_returned_yielded(self) -> BridgeExecutionBasisReadmissionYielded {
        BridgeExecutionBasisReadmissionYielded::new(self.yielded, self.counters)
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        BridgeYieldedExecutionBasis,
        Arc<str>,
        BridgeExecutionBasisReadmissionCounters,
    ) {
        (self.yielded, self.operation_binding_identity, self.counters)
    }
}
