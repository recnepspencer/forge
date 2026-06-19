use forge_proof::TransitionOutcome;
use forge_server::{
    ForgeServerOperationAdmissionDenialCode, ForgeServerOperationAuthorityDeclaration,
    ForgeServerOperationAuthorityKind, ForgeServerOperationAuthorizationPolicy,
    ForgeServerOperationConcurrencyClass, ForgeServerOperationConcurrencyDenialCode,
    ForgeServerOperationConcurrencyFacade, ForgeServerOperationFamily,
    ForgeServerOperationInputEnvelope, ForgeServerOperationRegistration,
    ForgeServerOperationRequestInput, ForgeServerPipelineInput, ForgeServerPipelineIntent,
    ForgeServerProductSessionCoordinationTarget, ForgeServerSharedReadBasisKind,
    ForgeServerSurfaceFamily,
};
use serde_json::json;

#[path = "support/operation_request/runtime.rs"]
mod operation_request_runtime;

use operation_request_runtime::{
    forge_native_resolved_context, forge_native_resolved_context_for_principal,
    operation_request_test_server, operation_request_test_server_with_operations,
};

#[test]
fn declared_shared_read_footprints_admit_concurrent_planning() {
    let server = operation_request_test_server();
    let resolved = forge_native_resolved_context(&server, None);
    let admission = match server.middleware().admit(ForgeServerPipelineInput::new(
        resolved,
        ForgeServerPipelineIntent::query_read("users.profile"),
    )) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected read admission, got {other:?}"),
    };

    let first = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::QueryDirectRead)
                .with_operation_name("users.profile")
                .with_basis_digest("basis-users-profile")
                .build(),
        )
        .expect("first operation request should admit");
    let second = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::QueryDirectRead)
                .with_operation_name("users.profile")
                .with_basis_digest("basis-users-profile")
                .build(),
        )
        .expect("second operation request should admit");

    let first_posture = server
        .operation_admissions()
        .admit_declared(&admission, &first)
        .expect("first posture should admit");
    let second_posture = server
        .operation_admissions()
        .admit_declared(&admission, &second)
        .expect("second posture should admit");
    let concurrency = ForgeServerOperationConcurrencyFacade
        .classify_pair(&first_posture, &second_posture)
        .expect("equivalent shared reads should admit concurrent classification");

    assert_eq!(
        first_posture.authority_footprint().authority_kind(),
        ForgeServerOperationAuthorityKind::SharedReadOnly
    );
    assert_eq!(
        concurrency,
        ForgeServerOperationConcurrencyClass::ConcurrentSharedRead
    );
    assert_eq!(
        first_posture.authority_footprint().canonical_digest(),
        second_posture.authority_footprint().canonical_digest()
    );
    assert_eq!(
        first_posture.footprint_receipt().canonical_digest(),
        second_posture.footprint_receipt().canonical_digest()
    );
}

#[test]
fn authorization_proof_is_required_after_footprint_classification() {
    let server =
        operation_request_test_server_with_operations([ForgeServerOperationRegistration::enabled(
            ForgeServerOperationFamily::QueryDirectRead,
        )
        .exposed_on([
            ForgeServerSurfaceFamily::ForgeNative,
            ForgeServerSurfaceFamily::CompatHttp,
        ])
        .admit_operation_names(["users.profile"])
        .with_authority_declaration(ForgeServerOperationAuthorityDeclaration::query_shared_read())
        .with_authorization_policy(
            ForgeServerOperationAuthorizationPolicy::require_principal("principal-9"),
        )]);
    let resolved = forge_native_resolved_context_for_principal(&server, None, "principal-7");
    let read_admission = match server.middleware().admit(ForgeServerPipelineInput::new(
        resolved,
        ForgeServerPipelineIntent::query_read("users.profile"),
    )) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected read admission, got {other:?}"),
    };
    let operation_request = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &read_admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::QueryDirectRead)
                .with_operation_name("users.profile")
                .with_basis_digest("basis-users-profile")
                .build(),
        )
        .expect("read request should admit");

    let denial = server
        .operation_admissions()
        .admit_declared(&read_admission, &operation_request)
        .expect_err("unauthorized principal should deny after valid footprint classification");

    assert_eq!(
        denial.code(),
        ForgeServerOperationAdmissionDenialCode::AuthorizationDenied
    );
    assert!(denial.detail().contains("principal `principal-7`"));
}

#[test]
fn ambiguous_operation_authority_cannot_fall_back_to_global_lock() {
    let server = operation_request_test_server();
    let resolved = forge_native_resolved_context(&server, None);
    let admission = match server.middleware().admit(ForgeServerPipelineInput::new(
        resolved,
        ForgeServerPipelineIntent::query_read("users.profile"),
    )) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected read admission, got {other:?}"),
    };
    let operation_request = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::QueryDirectRead)
                .with_operation_name("users.profile")
                .with_basis_digest("basis-users-profile")
                .build(),
        )
        .expect("read operation request should admit");

    let denial = server
        .operation_admissions()
        .admit(
            &admission,
            &operation_request,
            forge_server::ForgeServerOperationAuthorityMetadata::deterministic_submission(
                "query-write",
                "query-write-review",
                "caller-basis-bound",
                "best-effort",
            ),
        )
        .expect_err("mismatched authority metadata should deny");

    assert_eq!(
        denial.code(),
        ForgeServerOperationAdmissionDenialCode::AuthorityDenied
    );
    assert!(denial
        .detail()
        .contains("does not admit the declared authority metadata"));
}

#[test]
fn fixture_only_product_read_basis_is_admitted_as_declared_authority_fact() {
    let server = forge_server::ForgeServer::builder()
        .with_config(
            forge_server::ForgeServerConfig::builder()
                .with_bind_address(([127, 0, 0, 1], 8080).into())
                .with_request_context_config(
                    forge_server::ForgeServerRequestContextConfig::builder()
                        .with_default_diagnostics_profile(
                            forge_server::request_context::DiagnosticRichnessProfile::Standard,
                        )
                        .with_preview_targeting_enabled(true)
                        .build()
                        .expect("request context config should validate"),
                )
                .with_middleware_config(
                    forge_server::ForgeServerMiddlewareConfig::builder()
                        .with_query_mutation_enabled(true)
                        .build()
                        .expect("middleware config should validate"),
                )
                .build()
                .expect("server config should validate"),
        )
        .register_operation(
            ForgeServerOperationRegistration::enabled(
                ForgeServerOperationFamily::ProductApplicationRead,
            )
            .exposed_on([ForgeServerSurfaceFamily::ForgeNative])
            .admit_operation_names(["editor.render"])
            .with_authority_declaration(
                ForgeServerOperationAuthorityDeclaration::product_shared_read(
                    ForgeServerSharedReadBasisKind::FixtureOnly,
                ),
            ),
        )
        .register_surface(forge_server::surfaces::ForgeNativeSurface::enabled())
        .register_surface(forge_server::surfaces::CompatHttpSurface::phase_one_enabled())
        .build()
        .expect("fixture-only product read registration should build for readiness-owned denial");

    assert_eq!(server.surface_inventory().registered_families.len(), 2);
}

#[test]
fn shared_read_metadata_must_match_admitted_operation_basis() {
    let server = operation_request_test_server();
    let resolved = forge_native_resolved_context(&server, None);
    let admission = match server.middleware().admit(ForgeServerPipelineInput::new(
        resolved,
        ForgeServerPipelineIntent::query_read("users.profile"),
    )) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected read admission, got {other:?}"),
    };
    let operation_request = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::QueryDirectRead)
                .with_operation_name("users.profile")
                .with_basis_digest("basis-users-profile")
                .build(),
        )
        .expect("read operation request should admit");

    let denial = server
        .operation_admissions()
        .admit(
            &admission,
            &operation_request,
            forge_server::ForgeServerOperationAuthorityMetadata::shared_read(
                "query-shared-read-basis",
                "basis-users-profile-drifted",
                "users.profile",
            ),
        )
        .expect_err("drifted metadata basis should deny");

    assert_eq!(
        denial.code(),
        ForgeServerOperationAdmissionDenialCode::AuthorityDenied
    );
    assert!(denial
        .detail()
        .contains("does not match the admitted operation basis"));
}

#[test]
fn authorization_proof_distinguishes_payload_identity() {
    let server = operation_request_test_server();
    let resolved = forge_native_resolved_context(&server, None);
    let admission = match server.middleware().admit(ForgeServerPipelineInput::new(
        resolved,
        ForgeServerPipelineIntent::query_mutation("tasks.insert"),
    )) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected mutation admission, got {other:?}"),
    };
    let first = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::QueryDirectSubmission)
                .with_operation_name("tasks.insert")
                .with_payload_envelope(ForgeServerOperationInputEnvelope::json(
                    "task-command.v1",
                    &json!({"command": {"family": "insert", "id": "task-1"}}),
                ))
                .build(),
        )
        .expect("first mutation request should admit");
    let second = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::QueryDirectSubmission)
                .with_operation_name("tasks.insert")
                .with_payload_envelope(ForgeServerOperationInputEnvelope::json(
                    "task-command.v1",
                    &json!({"command": {"family": "insert", "id": "task-2"}}),
                ))
                .build(),
        )
        .expect("second mutation request should admit");

    let first_posture = server
        .operation_admissions()
        .admit_declared(&admission, &first)
        .expect("first mutation posture should admit");
    let second_posture = server
        .operation_admissions()
        .admit_declared(&admission, &second)
        .expect("second mutation posture should admit");

    assert_ne!(
        first_posture.authorization_proof().canonical_digest(),
        second_posture.authorization_proof().canonical_digest()
    );
}

#[test]
fn conflicting_product_draft_footprints_fail_before_execution() {
    let server = operation_request_test_server();
    let resolved = forge_native_resolved_context(&server, None);
    let admission = match server.middleware().admit(ForgeServerPipelineInput::new(
        resolved,
        ForgeServerPipelineIntent::query_mutation("editor.apply"),
    )) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected mutation admission, got {other:?}"),
    };
    let first = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::ProductApplicationMutation)
                .with_operation_name("editor.apply")
                .with_basis_digest("basis-editor-1")
                .with_idempotency_key("idem-editor-1")
                .with_product_session_identity("session-1")
                .build(),
        )
        .expect("first product mutation should admit");
    let second = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::ProductApplicationMutation)
                .with_operation_name("editor.apply")
                .with_basis_digest("basis-editor-2")
                .with_idempotency_key("idem-editor-2")
                .with_product_session_identity("session-1")
                .build(),
        )
        .expect("second product mutation should admit");

    let first_posture = server
        .operation_admissions()
        .admit_declared(&admission, &first)
        .expect("first product posture should admit");
    let second_posture = server
        .operation_admissions()
        .admit_declared(&admission, &second)
        .expect("second product posture should admit");

    let denial = ForgeServerOperationConcurrencyFacade
        .classify_pair(&first_posture, &second_posture)
        .expect_err("same product draft scope should not admit concurrent mutation");

    assert_eq!(
        denial.code(),
        ForgeServerOperationConcurrencyDenialCode::ConflictingMutableAuthority
    );
    assert!(denial.detail().contains("product draft scope"));
}

#[test]
fn session_creation_coordination_admits_without_preexisting_session_identity() {
    let server = operation_request_test_server();
    let resolved = forge_native_resolved_context(&server, None);
    let admission = match server.middleware().admit(ForgeServerPipelineInput::new(
        resolved,
        ForgeServerPipelineIntent::forge_native_session("product_session.open_mutation"),
    )) {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected forge-native session admission, got {other:?}"),
    };
    let operation_request = server
        .operation_requests()
        .admit_from_forge_native_admission(
            &admission,
            ForgeServerOperationRequestInput::builder()
                .with_operation_family(ForgeServerOperationFamily::ProductSessionCoordination)
                .with_operation_name("product_session.open_mutation")
                .with_basis_digest("basis-editor-1")
                .build(),
        )
        .expect("session creation request should admit without a fabricated identity");

    let posture = server
        .operation_admissions()
        .admit_declared(&admission, &operation_request)
        .expect("session creation posture should admit");

    assert_eq!(
        posture.authority_footprint().authority_kind(),
        ForgeServerOperationAuthorityKind::ProductSessionCoordination
    );
    assert!(matches!(
        posture
            .authority_metadata()
            .product_session_coordination_target(),
        Some((
            ForgeServerProductSessionCoordinationTarget::SessionCreation,
            "product-session"
        ))
    ));
    assert!(posture
        .authority_footprint()
        .canonical_digest()
        .contains("kind=workspace-branch"));
    assert!(!posture
        .authority_footprint()
        .canonical_digest()
        .contains("|session="));
}
