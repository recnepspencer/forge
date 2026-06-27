#[path = "trybuild_support.rs"]
mod trybuild_support;
fn parity_compile_pass(path: &str) {
    trybuild_support::new_test_cases().pass(path);
}

fn parity_compile_fail(path: &str) {
    trybuild_support::new_test_cases().compile_fail(path);
}

#[test]
fn file_rust_replacement_parity_public_types_are_importable() {
    parity_compile_pass(
        "tests/ui/runtime_file_rust_replacement_parity/pass/parity_public_types_are_importable.rs",
    );
}

#[test]
fn file_rust_replacement_parity_receipts_are_not_publicly_mintable() {
    parity_compile_fail(
        "tests/ui/runtime_file_rust_replacement_parity/fail/parity_receipt_fields_not_public.rs",
    );
}

#[test]
fn file_rust_replacement_pipeline_reports_are_not_publicly_mintable() {
    parity_compile_fail(
        "tests/ui/runtime_file_rust_replacement_parity/fail/pipeline_report_fields_not_public.rs",
    );
}

#[test]
fn file_rust_replacement_semantic_receipts_are_not_publicly_mintable() {
    parity_compile_fail(
        "tests/ui/runtime_file_rust_replacement_parity/fail/semantic_receipt_fields_not_public.rs",
    );
}

#[test]
fn rust_cannot_construct_replacement_candidate_directly() {
    parity_compile_fail(
        "tests/ui/runtime_file_rust_replacement_parity/fail/rust_cannot_construct_replacement_candidate.rs",
    );
}

#[test]
fn rust_cannot_inject_active_plan_nodes_directly() {
    parity_compile_fail(
        "tests/ui/runtime_file_rust_replacement_parity/fail/rust_cannot_inject_active_plan_nodes.rs",
    );
}

