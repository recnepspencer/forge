use serde_json::{json, Value};
use worth_server::{
    WorthServerCompletedProductOperation, WorthServerDurableProductMutationConclusion,
    WorthServerProductOperationOutcome, WorthServerProductOperationSurfaceDenialCode,
};

#[path = "support/durable_product_mutation/mod.rs"]
mod durable_support;

use durable_support::{
    build_server, execute, session, DurableMutationCrashPoint, TestDurableProductExecutor,
};

#[test]
fn crash_boundary_matrix_resolves_without_duplicate_product_effects() {
    for crash in [
        DurableMutationCrashPoint::BeforeIntent,
        DurableMutationCrashPoint::AfterIntent,
        DurableMutationCrashPoint::AfterMutationBeforeCommit,
    ] {
        let executor = TestDurableProductExecutor::default();
        let first_server = build_server(&executor);
        let payload = crash_payload(crash);
        let interrupted = execute(
            &session(&first_server, "tenant-a", "workspace-42"),
            "product.host_connection.upsert",
            "product.host-connection.upsert.v1",
            payload.clone(),
            "basis:0",
            "crash-matrix-key",
        )
        .expect("pre-commit crash should return a typed failed operation");
        assert_failed_at(&interrupted, crash);
        assert_eq!(executor.commit_count(), 0);
        drop(first_server);

        let rebuilt_server = build_server(&executor);
        let retried = execute(
            &session(&rebuilt_server, "tenant-a", "workspace-42"),
            "product.host_connection.upsert",
            "product.host-connection.upsert.v1",
            payload,
            "basis:0",
            "crash-matrix-key",
        )
        .expect("rolled-back crash point should permit one fresh commit");
        assert!(matches!(
            retried.outcome(),
            WorthServerProductOperationOutcome::Success(_)
        ));
        assert_eq!(executor.commit_count(), 1);
    }

    let executor = TestDurableProductExecutor::default();
    let first_server = build_server(&executor);
    let payload = crash_payload(DurableMutationCrashPoint::AfterCommitBeforeAcknowledgment);
    let indeterminate = execute(
        &session(&first_server, "tenant-a", "workspace-42"),
        "product.host_connection.upsert",
        "product.host-connection.upsert.v1",
        payload.clone(),
        "basis:0",
        "post-commit-crash-key",
    )
    .expect_err("post-commit acknowledgment loss must remain indeterminate");
    assert_eq!(
        indeterminate.code(),
        WorthServerProductOperationSurfaceDenialCode::Indeterminate
    );
    assert!(indeterminate
        .facts()
        .and_then(|facts| facts.recovery_handle())
        .is_some());
    assert_eq!(executor.commit_count(), 1);
    drop(first_server);

    let rebuilt_server = build_server(&executor);
    let retried = execute(
        &session(&rebuilt_server, "tenant-a", "workspace-42"),
        "product.host_connection.upsert",
        "product.host-connection.upsert.v1",
        payload,
        "basis:0",
        "post-commit-crash-key",
    )
    .expect("post-commit retry should resolve the persisted completion");
    assert!(retried.retry_diagnostics().is_previously_committed());
    assert_eq!(executor.commit_count(), 1);
}

#[test]
fn recovery_is_tenant_scoped_and_provider_completion_is_revalidated() {
    let executor = TestDurableProductExecutor::default();
    let server = build_server(&executor);
    let tenant_a = session(&server, "tenant-a", "workspace-42");
    let denial = execute(
        &tenant_a,
        "product.deployment.transition",
        "product.deployment.transition.v1",
        crash_payload(DurableMutationCrashPoint::AfterCommitBeforeAcknowledgment),
        "basis:0",
        "recovery-key",
    )
    .expect_err("post-commit acknowledgment loss should be indeterminate");
    let recovery = denial
        .facts()
        .and_then(|facts| facts.recovery_handle())
        .expect("indeterminate denial should carry recovery authority")
        .clone();

    let cross_tenant = session(&server, "tenant-b", "workspace-42")
        .product_operations()
        .resolve_durable_mutation(&recovery)
        .expect_err("another tenant must not resolve the handle");
    assert_eq!(
        cross_tenant.code(),
        WorthServerProductOperationSurfaceDenialCode::RequestDenied
    );

    match tenant_a
        .product_operations()
        .resolve_durable_mutation(&recovery)
        .expect("owning tenant should resolve the handle")
    {
        WorthServerDurableProductMutationConclusion::PreviouslyCommitted(completion) => {
            assert_eq!(completion.next_basis().value(), "basis:1");
        }
        other => panic!("expected previously committed recovery, got {other:?}"),
    }

    execute(
        &tenant_a,
        "product.host_connection.upsert",
        "product.host-connection.upsert.v1",
        json!({ "connection_id": "host-7" }),
        "basis:0",
        "unrelated-completion-key",
    )
    .expect("unrelated completion should commit");
    executor.override_recovery_with(executor.completion_for(
        "tenant-a",
        "workspace-42",
        "host-connection",
        "unrelated-completion-key",
    ));
    let invalid = tenant_a
        .product_operations()
        .resolve_durable_mutation(&recovery)
        .expect_err("server must reject a provider completion outside recovery authority");
    assert_eq!(
        invalid.code(),
        WorthServerProductOperationSurfaceDenialCode::InvalidDurableCompletion
    );
    assert_eq!(server.counters().durable_product_recovery_failed, 1);
}

fn crash_payload(crash: DurableMutationCrashPoint) -> Value {
    json!({
        "connection_id": "host-crash",
        "crash_point": crash.as_str(),
    })
}

fn assert_failed_at(
    operation: &WorthServerCompletedProductOperation,
    crash: DurableMutationCrashPoint,
) {
    match operation.outcome() {
        WorthServerProductOperationOutcome::Failed(failure) => {
            assert_eq!(
                failure.reason_key(),
                format!("injected_crash_{}", crash.as_str())
            );
        }
        other => panic!("expected injected crash failure, got {other:?}"),
    }
}
