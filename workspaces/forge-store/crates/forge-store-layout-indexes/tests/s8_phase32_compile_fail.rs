mod compile_fail_support;

#[test]
fn phase32_public_authority_surfaces_reject_external_bypass() {
    for fixture in fixtures() {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "phase32",
            fixture.name,
            fixture.expected_stderr,
            fixture.extern_crates,
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
    extern_crates: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 9] {
    [
        fixture(
            "caller_defined_ready_access_receipt_is_not_constructible.rs",
            &["S8ExecutionReadyAccessReceipt", "private field"],
            &[],
        ),
        fixture(
            "ready_access_receipt_constructor_is_not_public.rs",
            &["from_recipe", "private associated function"],
            &[],
        ),
        fixture(
            "caller_defined_executed_access_receipt_is_not_constructible.rs",
            &["S8ExecutedAccessReceipt", "private field"],
            &[],
        ),
        fixture(
            "counter_snapshot_exact_constructor_is_not_public.rs",
            &["exact", "private associated function"],
            &[],
        ),
        fixture(
            "counter_snapshot_support_constructor_is_not_public.rs",
            &["snapshot_support", "function or associated item"],
            &[],
        ),
        fixture(
            "support_counter_evidence_derived_constructor_is_not_public.rs",
            &[
                "snapshot_support_counter_evidence",
                "branch_delta_support_counter_evidence",
                "stable_basis_support_counter_evidence",
                "continuation_support_counter_evidence",
            ],
            &[],
        ),
        fixture(
            "layout_readmission_witness_is_not_constructible.rs",
            &["S8LayoutReadmissionWitness", "private field"],
            &[],
        ),
        fixture(
            "private_execution_module_cannot_bypass_access_execution_facade.rs",
            &["module `execution` is private"],
            &[],
        ),
        fixture(
            "private_corruption_module_cannot_bypass_layout_readmission_facade.rs",
            &["module `corruption` is private"],
            &[],
        ),
    ]
}

const fn fixture(
    name: &'static str,
    expected_stderr: &'static [&'static str],
    extern_crates: &'static [&'static str],
) -> CompileFailFixture {
    CompileFailFixture {
        name,
        expected_stderr,
        extern_crates,
    }
}
