#[test]
fn cross_capability_options_are_rejected() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/declarative_read/aggregate_declaration_cannot_open.rs");
    cases.compile_fail("tests/ui/declarative_read/live_declaration_cannot_run.rs");
    cases.compile_fail("tests/ui/declarative_read/declaration_requires_context_before_run.rs");
    cases.compile_fail(
        "tests/ui/declarative_history_comparison/comparison_rejects_history_context.rs",
    );
    cases.compile_fail(
        "tests/ui/declarative_workflow/preview_context_cannot_authorize_writeback.rs",
    );
}

#[test]
fn receipts_cannot_promote_to_authority() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/declarative_workflow/read_receipt_cannot_consume_projection.rs");
    cases.compile_fail(
        "tests/ui/declarative_workflow/inspection_receipt_cannot_authorize_inspection.rs",
    );
    cases.compile_fail(
        "tests/ui/public_authority_surface/causal_receipt_cannot_author_inspection.rs",
    );
    cases.compile_fail(
        "tests/ui/declarative_workflow/success_envelope_cannot_be_constructed.rs",
    );
}
