//! R8.29 / Q8.9 — registry lifecycle authority belongs only to execution.
//!
//! A supported feature must not widen this boundary. The host facade exports
//! the move-only recovery handle, but neither its registry type nor a route
//! from the handle to the registry's slot-addressed lifecycle controls.

use worth_query_execution::facade::primary_graph::{
    WorthQueryRecoveryHandle, WorthQueryRecoveryHandleRegistry,
};

fn holder_cannot_reach_registry_authority(handle: &WorthQueryRecoveryHandle) {
    let _ = handle.registry_arc();
    let _ = handle.registry_slot();
}

fn main() {}
