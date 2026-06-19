use forge_server::{
    ForgeServerCompatibilityAdmittedProductMutationCommand,
    ForgeServerForgeNativeProductMutationCommand, ForgeServerProductIdempotencyKey,
    ForgeServerProductOperationBaseDigest, ForgeServerProductOperationSurfaceDenialCode,
    ForgeServerProductSessionCreationRequest,
};

#[path = "support/product_operation_phase_eleven/backend.rs"]
mod backend;
#[path = "support/product_session_phase_ten/fixture.rs"]
mod fixture;

use backend::{stateful_editor_registration, StatefulProductEditorBackend};
use fixture::{
    apply_payload, apply_payload_with_title, build_server, direct_session,
    prepared_product_mutation_request_with_basis_and_header, prepared_session_request,
};

#[test]
fn forge_native_product_operation_replays_identical_idempotent_mutation_without_second_state_change(
) {
    let backend = StatefulProductEditorBackend::new();
    let initial_basis = backend.basis_digest();
    let server = build_server(vec![stateful_editor_registration(backend.clone())]);
    let session = direct_session(&server, "workspace-42", "branch-9");
    let mutation_session = session
        .product_sessions()
        .open_mutation(
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest(initial_basis.as_str())
                .with_expiry_seconds(300),
        )
        .expect("mutation session should open");
    let idempotency_key =
        ForgeServerProductIdempotencyKey::new("apply-rename-attempt-1").expect("valid key");

    let first = session
        .product_operations()
        .execute_mutation(
            ForgeServerForgeNativeProductMutationCommand::new(
                "product_editor.apply",
                apply_payload(),
            )
            .within(&mutation_session)
            .against_base_digest(base_digest(&initial_basis))
            .idempotent(idempotency_key.clone()),
        )
        .expect("first mutation should succeed");
    let basis_after_first = backend.basis_digest();
    let replayed = session
        .product_operations()
        .execute_mutation(
            ForgeServerForgeNativeProductMutationCommand::new(
                "product_editor.apply",
                apply_payload(),
            )
            .within(&mutation_session)
            .against_base_digest(base_digest(&initial_basis))
            .idempotent(idempotency_key),
        )
        .expect("identical mutation should replay");

    assert_eq!(backend.title(), "Rename");
    assert_ne!(basis_after_first, initial_basis);
    assert_eq!(backend.basis_digest(), basis_after_first);
    assert!(first.replay_diagnostics().is_authoritative());
    assert!(first.replay_diagnostics().adapter_execution_attempted());
    assert!(replayed.replay_diagnostics().is_replayed());
    assert!(replayed
        .replay_diagnostics()
        .adapter_execution_skipped_by_replay());
    assert_eq!(
        replayed
            .replay_receipt()
            .and_then(|receipt| receipt.authoritative_operation_digest()),
        Some(first.envelope().canonical_digest())
    );
}

#[test]
fn compat_product_operation_preserves_phase_eleven_replay_contract_via_request_shaped_command() {
    let backend = StatefulProductEditorBackend::new();
    let initial_basis = backend.basis_digest();
    let server = build_server(vec![stateful_editor_registration(backend.clone())]);
    let session_request = prepared_session_request(
        &server,
        "workspace-42",
        "branch-9",
        "product_session.open_mutation",
    );
    let prepared_request = prepared_product_mutation_request_with_basis_and_header(
        &server,
        "workspace-42",
        "branch-9",
        "product_editor.apply",
        &initial_basis,
        "idempotency-key",
        "compat-apply-attempt-1",
    );
    let opened = server
        .compat_http()
        .product_sessions()
        .open_mutation_for_product_operation(
            &session_request,
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest(initial_basis.as_str())
                .with_expiry_seconds(300),
        )
        .expect("compat mutation session should open");

    let first = server
        .compat_http()
        .product_operations()
        .execute_admitted_mutation(
            &prepared_request,
            ForgeServerCompatibilityAdmittedProductMutationCommand::new(
                "product_editor.apply",
                apply_payload(),
            )
            .with_session(opened.continuation()),
        )
        .expect("first compat mutation should succeed");
    let basis_after_first = backend.basis_digest();
    let replayed = server
        .compat_http()
        .product_operations()
        .execute_admitted_mutation(
            &prepared_request,
            ForgeServerCompatibilityAdmittedProductMutationCommand::new(
                "product_editor.apply",
                apply_payload(),
            )
            .with_session(opened.continuation()),
        )
        .expect("compat mutation replay should succeed");

    assert_eq!(backend.title(), "Rename");
    assert_ne!(basis_after_first, initial_basis);
    assert_eq!(backend.basis_digest(), basis_after_first);
    assert!(first.replay_diagnostics().is_authoritative());
    assert!(replayed.replay_diagnostics().is_replayed());
    assert!(replayed
        .replay_diagnostics()
        .adapter_execution_skipped_by_replay());
    assert_eq!(
        replayed
            .replay_receipt()
            .and_then(|receipt| receipt.authoritative_operation_digest()),
        Some(first.envelope().canonical_digest())
    );
}

#[test]
fn compat_product_operation_reuses_key_with_different_payload_as_conflict() {
    let backend = StatefulProductEditorBackend::new();
    let initial_basis = backend.basis_digest();
    let server = build_server(vec![stateful_editor_registration(backend.clone())]);
    let session_request = prepared_session_request(
        &server,
        "workspace-42",
        "branch-9",
        "product_session.open_mutation",
    );
    let prepared_request = prepared_product_mutation_request_with_basis_and_header(
        &server,
        "workspace-42",
        "branch-9",
        "product_editor.apply",
        &initial_basis,
        "idempotency-key",
        "compat-apply-attempt-2",
    );
    let opened = server
        .compat_http()
        .product_sessions()
        .open_mutation_for_product_operation(
            &session_request,
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest(initial_basis.as_str())
                .with_expiry_seconds(300),
        )
        .expect("compat mutation session should open");

    server
        .compat_http()
        .product_operations()
        .execute_admitted_mutation(
            &prepared_request,
            ForgeServerCompatibilityAdmittedProductMutationCommand::new(
                "product_editor.apply",
                apply_payload(),
            )
            .with_session(opened.continuation()),
        )
        .expect("first compat mutation should succeed");

    let denial = server
        .compat_http()
        .product_operations()
        .execute_admitted_mutation(
            &prepared_request,
            ForgeServerCompatibilityAdmittedProductMutationCommand::new(
                "product_editor.apply",
                apply_payload_with_title("Different"),
            )
            .with_session(opened.continuation()),
        )
        .expect_err("changing payload under the same key should conflict");

    assert_eq!(
        denial.code(),
        ForgeServerProductOperationSurfaceDenialCode::IdempotencyConflict
    );
    assert_eq!(
        denial
            .facts()
            .and_then(|facts| facts.idempotency_conflict())
            .map(|conflict| conflict.idempotency_key()),
        Some("compat-apply-attempt-2")
    );
    assert_eq!(backend.revision(), 1);
}

#[test]
fn product_mutation_without_snapshot_precondition_denies_before_adapter_execution() {
    let backend = StatefulProductEditorBackend::new();
    let initial_basis = backend.basis_digest();
    let server = build_server(vec![stateful_editor_registration(backend.clone())]);
    let session = direct_session(&server, "workspace-42", "branch-9");
    let mutation_session = session
        .product_sessions()
        .open_mutation(
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest(initial_basis.as_str())
                .with_expiry_seconds(300),
        )
        .expect("mutation session should open");

    let denial = session
        .product_operations()
        .execute_mutation(
            ForgeServerForgeNativeProductMutationCommand::new(
                "product_editor.apply",
                apply_payload(),
            )
            .within(&mutation_session),
        )
        .expect_err("missing snapshot precondition should deny");

    assert_eq!(
        denial.code(),
        ForgeServerProductOperationSurfaceDenialCode::PreconditionDenied
    );
    assert_eq!(backend.revision(), 0);
    assert_eq!(backend.title(), "Untitled");
}

fn base_digest(value: &str) -> ForgeServerProductOperationBaseDigest {
    ForgeServerProductOperationBaseDigest::new(value).expect("valid base digest")
}
