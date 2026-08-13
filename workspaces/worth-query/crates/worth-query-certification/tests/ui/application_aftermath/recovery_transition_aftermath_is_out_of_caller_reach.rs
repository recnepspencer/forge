//! R8.30 — a transition dispatches on the aftermath its handle carries, and a
//! caller can neither supply that binding nor read the contract inside it.
//!
//! Deleting the `aftermath` parameter is only half the property. What makes the
//! remaining lane safe is that `WorthQueryRecoveryHandleBinding` is exported for
//! naming but not for building — its fields are private and `from_receipt` is
//! confined to the mint path — and the contract the six transitions match on is
//! reachable only from inside the crate.

use worth_query_execution::facade::primary_graph::{
    WorthQueryRecoveryHandle, WorthQueryRecoveryHandleBinding,
};

fn caller_cannot_build_a_binding_carrying_its_own_aftermath() {
    let _ = WorthQueryRecoveryHandleBinding {
        installed_operation: [0u8; 32],
    };
}

fn caller_cannot_read_the_contract_transitions_dispatch_on(handle: &WorthQueryRecoveryHandle) {
    let _ = handle.binding().installed_aftermath();
}

fn main() {}
