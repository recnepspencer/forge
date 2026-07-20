use worth_proof::TransitionOutcome;
use worth_server::{
    WorthServer, WorthServerProductApplicationAdapterRegistration, WorthServerWorthNativeSession,
    WorthServerWorthNativeSessionInput,
};

use super::{registration, TestDurableProductExecutor};

#[path = "../product_adapter_phase_nine/fixture.rs"]
mod product_adapter_fixture;

pub fn build_server(executor: &TestDurableProductExecutor) -> WorthServer {
    product_adapter_fixture::build_server(vec![registration(executor.clone())])
}

pub fn build_server_with_registration(
    registration: WorthServerProductApplicationAdapterRegistration,
) -> WorthServer {
    product_adapter_fixture::build_server(vec![registration])
}

pub fn session(
    server: &WorthServer,
    tenant_id: &str,
    workspace_id: &str,
) -> WorthServerWorthNativeSession {
    match server.worth_native().session(
        WorthServerWorthNativeSessionInput::builder()
            .with_authenticated_principal_id("principal-7")
            .with_tenant_id(tenant_id)
            .with_workspace_id(workspace_id)
            .with_branch_id("branch-9")
            .build()
            .expect("test session input should validate"),
    ) {
        TransitionOutcome::Success(session) => session,
        other => panic!("expected WORTH-native session, got {other:?}"),
    }
}
