use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use forge_server::{
    ForgeServerProductOperationExecutionBoundary, ForgeServerProductOperationInput,
    ForgeServerProductOperationOutcome, ForgeServerProductOperationSurfaceDenialCode,
    ForgeServerProductSessionCreationRequest, ForgeServerProductSessionDenialCode,
    ForgeServerProductSessionLifecycle,
};

#[path = "support/product_session_phase_ten/fixture.rs"]
mod fixture;

use fixture::{
    apply_payload, build_server, build_server_with_clock, direct_session,
    prepared_product_mutation_request, prepared_product_read_request, prepared_session_request,
    preview_payload, session_backed_editor_registration, ManualProductSessionClock,
};

#[test]
fn product_session_lifecycle_denies_expired_foreign_or_moved_sessions() {
    let calls = Arc::new(AtomicUsize::new(0));
    let clock = Arc::new(ManualProductSessionClock::new(1_000));
    let server = build_server_with_clock(
        vec![session_backed_editor_registration(calls.clone())],
        Some(clock.clone()),
    );
    let primary_session = direct_session(&server, "workspace-42", "branch-9");

    let preview_session = primary_session
        .product_sessions()
        .open_preview(
            ForgeServerProductSessionCreationRequest::for_operation(
                "product_editor.render_preview",
            )
            .with_basis_digest("basis:head")
            .with_expiry_seconds(300),
        )
        .expect("preview session should open");
    assert_eq!(
        preview_session.lifecycle(),
        ForgeServerProductSessionLifecycle::ReadOnlyPreview
    );
    let preview_completed = primary_session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new(
                "product_editor.render_preview",
                preview_payload(),
            )
            .with_basis_digest("basis:head")
            .with_product_session_identity(preview_session.identity().as_str()),
        )
        .expect("preview session should support read-only preview");
    assert!(matches!(
        preview_completed.outcome(),
        ForgeServerProductOperationOutcome::Success(_)
    ));

    let mutation_session = primary_session
        .product_sessions()
        .open_mutation(
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest("basis:head")
                .with_expiry_seconds(300),
        )
        .expect("mutation session should open");
    assert_eq!(
        mutation_session.lifecycle(),
        ForgeServerProductSessionLifecycle::MutationDraft
    );
    let applied = primary_session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_basis_digest("basis:head")
                .with_product_session_identity(mutation_session.identity().as_str()),
        )
        .expect("mutation session should admit mutation work");
    assert!(matches!(
        applied.outcome(),
        ForgeServerProductOperationOutcome::Success(_)
    ));

    let expired_session = primary_session
        .product_sessions()
        .open_mutation(
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest("basis:head")
                .with_expiry_seconds(1),
        )
        .expect("expired test session should open");
    clock.advance_millis(1_001);
    let expired = primary_session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_basis_digest("basis:head")
                .with_product_session_identity(expired_session.identity().as_str()),
        )
        .expect_err("expired session should deny before mutation scheduling");
    assert_eq!(
        expired.code(),
        ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied
    );
    assert_eq!(
        expired
            .facts()
            .and_then(|facts| facts.session_denial_code()),
        Some(ForgeServerProductSessionDenialCode::ExpiredProductSession)
    );
    assert_eq!(
        expired.facts().and_then(|facts| facts.execution_boundary()),
        Some(&ForgeServerProductOperationExecutionBoundary::RejectedBeforeAdapterExecution)
    );

    primary_session
        .product_sessions()
        .close(mutation_session.identity())
        .expect("close should succeed");
    let closed = primary_session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_basis_digest("basis:head")
                .with_product_session_identity(mutation_session.identity().as_str()),
        )
        .expect_err("closed session should deny");
    assert_eq!(
        closed.code(),
        ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied
    );
    assert_eq!(
        closed.facts().and_then(|facts| facts.session_denial_code()),
        Some(ForgeServerProductSessionDenialCode::ClosedProductSession)
    );

    let foreign_session = direct_session(&server, "workspace-else", "branch-9")
        .product_sessions()
        .open_mutation(
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest("basis:head")
                .with_expiry_seconds(300),
        )
        .expect("foreign workspace session should open");
    let foreign = primary_session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_basis_digest("basis:head")
                .with_product_session_identity(foreign_session.identity().as_str()),
        )
        .expect_err("foreign workspace session should deny");
    assert_eq!(
        foreign.code(),
        ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied
    );
    assert_eq!(
        foreign
            .facts()
            .and_then(|facts| facts.session_denial_code()),
        Some(ForgeServerProductSessionDenialCode::ForeignProductSession)
    );

    let moved_session = direct_session(&server, "workspace-42", "branch-else")
        .product_sessions()
        .open_mutation(
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest("basis:head")
                .with_expiry_seconds(300),
        )
        .expect("moved-session test session should open");
    let moved = primary_session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_basis_digest("basis:head")
                .with_product_session_identity(moved_session.identity().as_str()),
        )
        .expect_err("branch-moved session should deny");
    assert_eq!(
        moved.code(),
        ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied
    );
    assert_eq!(
        moved.facts().and_then(|facts| facts.session_denial_code()),
        Some(ForgeServerProductSessionDenialCode::SessionRebindRequired)
    );

    let counters = server.product_session_registry().counter_snapshot();
    assert_eq!(counters.sessions_created, 5);
    assert_eq!(counters.preview_sessions_created, 1);
    assert_eq!(counters.mutation_sessions_created, 4);
    assert_eq!(counters.lookups_attempted, 7);
    assert_eq!(counters.lookups_denied_expired, 1);
    assert_eq!(counters.lookups_denied_closed, 1);
    assert_eq!(counters.lookups_denied_foreign, 1);
    assert_eq!(counters.lookups_denied_moved, 1);
    assert_eq!(counters.closes_recorded, 1);
    assert_eq!(calls.load(Ordering::Relaxed), 2);
}

#[test]
fn product_session_identity_is_server_admitted_not_adapter_fabricated() {
    let calls = Arc::new(AtomicUsize::new(0));
    let server = build_server(vec![session_backed_editor_registration(calls.clone())]);
    let session = direct_session(&server, "workspace-42", "branch-9");

    let denial = session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_basis_digest("basis:head")
                .with_product_session_identity("product-session:forged"),
        )
        .expect_err("fabricated session identity should deny before adapter execution");

    assert_eq!(
        denial.code(),
        ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied
    );
    assert_eq!(
        denial.facts().and_then(|facts| facts.session_denial_code()),
        Some(ForgeServerProductSessionDenialCode::UnknownProductSessionIdentity)
    );
    assert_eq!(
        denial.facts().and_then(|facts| facts.execution_boundary()),
        Some(&ForgeServerProductOperationExecutionBoundary::RejectedBeforeAdapterExecution)
    );
}

#[test]
fn preview_and_mutation_session_postures_remain_distinct() {
    let calls = Arc::new(AtomicUsize::new(0));
    let server = build_server(vec![session_backed_editor_registration(calls.clone())]);
    let session = direct_session(&server, "workspace-42", "branch-9");

    let preview = session
        .product_sessions()
        .open_preview(
            ForgeServerProductSessionCreationRequest::for_operation(
                "product_editor.render_preview",
            )
            .with_basis_digest("basis:head")
            .with_expiry_seconds(300),
        )
        .expect("preview session should open");
    let mutation = session
        .product_sessions()
        .open_mutation(
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest("basis:head")
                .with_expiry_seconds(300),
        )
        .expect("mutation session should open");

    assert_ne!(preview.canonical_digest(), mutation.canonical_digest());
    assert_eq!(
        preview.lifecycle(),
        ForgeServerProductSessionLifecycle::ReadOnlyPreview
    );
    assert_eq!(
        mutation.lifecycle(),
        ForgeServerProductSessionLifecycle::MutationDraft
    );

    let preview_denial = session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_basis_digest("basis:head")
                .with_product_session_identity(preview.identity().as_str()),
        )
        .expect_err("preview session must not authorize mutation");
    assert_eq!(
        preview_denial.code(),
        ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied
    );
    assert_eq!(
        preview_denial
            .facts()
            .and_then(|facts| facts.session_denial_code()),
        Some(ForgeServerProductSessionDenialCode::PreviewSessionCannotMutate)
    );

    let mutation_completed = session
        .product_operations()
        .execute(
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_basis_digest("basis:head")
                .with_product_session_identity(mutation.identity().as_str()),
        )
        .expect("mutation session should authorize mutation");
    assert!(matches!(
        mutation_completed.outcome(),
        ForgeServerProductOperationOutcome::Success(_)
    ));

    let counters = server.product_session_registry().counter_snapshot();
    assert_eq!(counters.preview_sessions_created, 1);
    assert_eq!(counters.mutation_sessions_created, 1);
    assert_eq!(counters.lookups_denied_preview_for_mutation, 1);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[test]
fn compat_http_product_sessions_hold_server_admitted_identity_across_requests() {
    let calls = Arc::new(AtomicUsize::new(0));
    let server = build_server(vec![session_backed_editor_registration(calls.clone())]);
    let prepared_open_mutation = prepared_session_request(
        &server,
        "workspace-42",
        "branch-9",
        "product_session.open_mutation",
    );
    let prepared_open_preview = prepared_session_request(
        &server,
        "workspace-42",
        "branch-9",
        "product_session.open_preview",
    );
    let prepared_close =
        prepared_session_request(&server, "workspace-42", "branch-9", "product_session.close");

    let opened = server
        .compat_http()
        .product_sessions()
        .open_mutation_with_proof(
            &prepared_open_mutation,
            ForgeServerProductSessionCreationRequest::for_operation("product_editor.apply")
                .with_basis_digest("basis:head")
                .with_expiry_seconds(300),
        )
        .expect("compat session open should admit");

    assert_eq!(
        opened.plan().concurrency_class(),
        opened.scheduler_admission().concurrency_class()
    );
    assert!(opened
        .scheduler_admission()
        .scheduler_lane()
        .contains("product-session-create:"));

    let applied = server
        .compat_http()
        .product_operations()
        .execute(
            &prepared_product_mutation_request(
                &server,
                "workspace-42",
                "branch-9",
                "product_editor.apply",
                Some("basis:head"),
            ),
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_product_session_identity(opened.session().identity().as_str()),
        )
        .expect("compat product mutation should accept server-admitted session");
    assert!(applied.adapter_execution_attempted());

    let preview_opened = server
        .compat_http()
        .product_sessions()
        .open_preview(
            &prepared_open_preview,
            ForgeServerProductSessionCreationRequest::for_operation(
                "product_editor.render_preview",
            )
            .with_basis_digest("basis:head")
            .with_expiry_seconds(300),
        )
        .expect("compat preview session should open");
    let preview_completed = server
        .compat_http()
        .product_operations()
        .execute(
            &prepared_product_read_request(
                &server,
                "workspace-42",
                "branch-9",
                "product_editor.render_preview",
                Some("basis:head"),
            ),
            ForgeServerProductOperationInput::new(
                "product_editor.render_preview",
                preview_payload(),
            )
            .with_product_session_identity(preview_opened.identity().as_str()),
        )
        .expect("compat preview read should succeed");
    assert!(preview_completed.adapter_execution_attempted());

    let closed = server
        .compat_http()
        .product_sessions()
        .close_with_proof(&prepared_close, opened.session().identity())
        .expect("compat close should succeed");
    assert!(closed
        .scheduler_admission()
        .scheduler_lane()
        .contains(opened.session().identity().as_str()));

    let denial = server
        .compat_http()
        .product_operations()
        .execute(
            &prepared_product_mutation_request(
                &server,
                "workspace-42",
                "branch-9",
                "product_editor.apply",
                Some("basis:head"),
            ),
            ForgeServerProductOperationInput::new("product_editor.apply", apply_payload())
                .with_product_session_identity(opened.session().identity().as_str()),
        )
        .expect_err("closed compat session should deny across requests");
    assert_eq!(
        denial.facts().and_then(|facts| facts.session_denial_code()),
        Some(ForgeServerProductSessionDenialCode::ClosedProductSession)
    );
}
