use worth_query_host::facade::domain::ApplicationSchema;
use worth_query_host::facade::primary_graph::{
    WorthQueryOpaqueRecoveryWireIdentity, WorthQueryPrimaryGraphApplicationRuntime,
};

fn cannot_substitute_wire_identity_for_committed_receipt<Schema: ApplicationSchema>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    wire_identity: &WorthQueryOpaqueRecoveryWireIdentity,
) {
    let _ = runtime.mint_recovery_handle(wire_identity);
}

fn main() {}
