#[test]
fn graph_read_access_async_materialization_boundaries_reject_forged_artifacts() {
    let t = trybuild::TestCases::new();
    t.compile_fail(
        "tests/ui/graph_read_access_async_materialization/request_constructor_private.rs",
    );
    t.compile_fail("tests/ui/graph_read_access_async_materialization/job_constructor_private.rs");
    t.compile_fail(
        "tests/ui/graph_read_access_async_materialization/progress_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_async_materialization/checkpoint_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_async_materialization/receipt_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_async_materialization/cancellation_receipt_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_async_materialization/recovery_handle_constructor_private.rs",
    );
    t.compile_fail(
        "tests/ui/graph_read_access_async_materialization/resource_limit_receipt_constructor_private.rs",
    );
}
