use forge_query::facade::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimeSupportProfile,
};
use forge_server::{
    ForgeServerDirectDeliveryClass, ForgeServerDirectFreshnessMode, ForgeServerMiddlewareConfig,
    ForgeServerQueryHandoffDenialCode, ForgeServerQueryHandoffInput,
    ForgeServerQueryHandoffOperation, ForgeServerQueryRequestedResume, ForgeServerSurfaceFamily,
    ForgeServerTransportClass,
};

#[path = "support/query_handoff/fixture.rs"]
mod query_handoff_fixture;
#[path = "support/query_handoff/runtime.rs"]
mod query_handoff_runtime;

use query_handoff_fixture::{
    admit_mutation_posture, admit_read_posture, denied, request_input, resolve_request_context,
    test_server, test_server_with_middleware,
};
use query_handoff_runtime::{ProfiledTestWorkspaceProvider, TestWorkspaceProvider};

#[test]
fn prepare_denies_durable_resume_but_admits_runtime_backed_resume() {
    let server = test_server(TestWorkspaceProvider::default(), false);
    let admission = admit_read_posture(
        &server,
        resolve_request_context(
            &server,
            request_input(
                ForgeServerSurfaceFamily::ForgeNative,
                ForgeServerTransportClass::ForgeNativeInProcess,
            ),
        ),
    );

    let durable_denial = denied(
        server
            .query_handoff()
            .prepare(ForgeServerQueryHandoffInput::new(
                admission.clone(),
                ForgeServerQueryHandoffOperation::downstream_delivery(
                    "users.profile",
                    ForgeServerDirectFreshnessMode::LiveStrict,
                    ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
                    ForgeServerQueryRequestedResume::durable(),
                ),
            )),
    );
    let runtime_backed = query_handoff_fixture::success(server.query_handoff().prepare(
        ForgeServerQueryHandoffInput::new(
            admission,
            ForgeServerQueryHandoffOperation::downstream_delivery(
                "users.profile",
                ForgeServerDirectFreshnessMode::LiveStrict,
                ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
                ForgeServerQueryRequestedResume::runtime_backed(None::<String>),
            ),
        ),
    ));

    assert_eq!(
        durable_denial.code(),
        ForgeServerQueryHandoffDenialCode::DurableResumeDeferred
    );
    assert!(durable_denial
        .detail()
        .contains("durable resume remains deferred"));
    assert!(matches!(
        runtime_backed.support_posture(),
        forge_server::ForgeServerQuerySupportPosture::RuntimeBackedResumeSupported { .. }
    ));
}

#[test]
fn prepare_denies_mismatched_read_intent_before_query_binding() {
    let server = test_server(TestWorkspaceProvider::default(), false);
    let denial = denied(
        server
            .query_handoff()
            .prepare(ForgeServerQueryHandoffInput::new(
                admit_read_posture(
                    &server,
                    resolve_request_context(
                        &server,
                        request_input(
                            ForgeServerSurfaceFamily::ForgeNative,
                            ForgeServerTransportClass::ForgeNativeInProcess,
                        ),
                    ),
                ),
                ForgeServerQueryHandoffOperation::query_mutation("users.rename"),
            )),
    );

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::PreparedIntentMismatch
    );
}

#[test]
fn prepare_denies_read_handoff_when_query_workspace_does_not_admit_read_family() {
    let server = test_server(
        ProfiledTestWorkspaceProvider::new(
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
                ForgeQueryRuntimeFamilySupport::unsupported(
                    ForgeQueryRuntimeFacadeFamily::Read,
                    "read is intentionally denied in this hostile test profile",
                ),
            ),
        ),
        false,
    );

    let denial = denied(
        server
            .query_handoff()
            .prepare(ForgeServerQueryHandoffInput::new(
                admit_read_posture(
                    &server,
                    resolve_request_context(
                        &server,
                        request_input(
                            ForgeServerSurfaceFamily::ForgeNative,
                            ForgeServerTransportClass::ForgeNativeInProcess,
                        ),
                    ),
                ),
                ForgeServerQueryHandoffOperation::query_read("users.profile"),
            )),
    );

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
    );
    assert!(denial
        .detail()
        .contains("query workspace does not admit `read` facade family"));
}

#[test]
fn prepare_denies_downstream_delivery_when_query_workspace_does_not_admit_live_family() {
    let server = test_server(
        ProfiledTestWorkspaceProvider::new(
            ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_family_support(
                ForgeQueryRuntimeFamilySupport::unsupported(
                    ForgeQueryRuntimeFacadeFamily::Live,
                    "live delivery is intentionally denied in this hostile test profile",
                ),
            ),
        ),
        false,
    );

    let denial = denied(
        server
            .query_handoff()
            .prepare(ForgeServerQueryHandoffInput::new(
                admit_read_posture(
                    &server,
                    resolve_request_context(
                        &server,
                        request_input(
                            ForgeServerSurfaceFamily::ForgeNative,
                            ForgeServerTransportClass::ForgeNativeInProcess,
                        ),
                    ),
                ),
                ForgeServerQueryHandoffOperation::downstream_delivery(
                    "users.profile",
                    ForgeServerDirectFreshnessMode::LiveStrict,
                    ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
                    ForgeServerQueryRequestedResume::none(),
                ),
            )),
    );

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
    );
    assert!(denial
        .detail()
        .contains("query workspace does not admit `live` facade family"));
}

#[test]
fn prepare_denies_downstream_delivery_when_middleware_only_admitted_mutation_intent() {
    let server = test_server_with_middleware(
        TestWorkspaceProvider::default(),
        false,
        ForgeServerMiddlewareConfig::builder()
            .with_query_mutation_enabled(true)
            .build()
            .expect("middleware config should validate"),
    );
    let mutation_admission = admit_mutation_posture(
        &server,
        resolve_request_context(
            &server,
            request_input(
                ForgeServerSurfaceFamily::ForgeNative,
                ForgeServerTransportClass::ForgeNativeInProcess,
            ),
        ),
    );

    let denial = denied(
        server
            .query_handoff()
            .prepare(ForgeServerQueryHandoffInput::new(
                mutation_admission,
                ForgeServerQueryHandoffOperation::downstream_delivery(
                    "users.rename",
                    ForgeServerDirectFreshnessMode::LiveStrict,
                    ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
                    ForgeServerQueryRequestedResume::none(),
                ),
            )),
    );

    assert_eq!(
        denial.code(),
        ForgeServerQueryHandoffDenialCode::DownstreamDeliveryRequiresReadIntent
    );
}
