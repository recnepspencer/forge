use super::compile_fail_support;

#[test]
fn public_authority_surfaces_reject_external_bypass() {
    for fixture in fixtures() {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "closeout",
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
            "counter_snapshot_exact_constructor_is_not_public.rs",
            &["S8AccessPathCounterSnapshot", "could not find"],
            &[],
        ),
        fixture(
            "counter_snapshot_support_constructor_is_not_public.rs",
            &["S8AccessPathCounterSnapshot", "could not find"],
            &[],
        ),
        fixture(
            "support_counter_evidence_derived_constructor_is_not_public.rs",
            &["layout_counters", "could not find"],
            &[],
        ),
        fixture(
            "layout_readmission_witness_is_not_constructible.rs",
            &["LayoutReadmissionWitness", "private field"],
            &[],
        ),
        fixture(
            "private_execution_module_cannot_bypass_access_execution_facade.rs",
            &["module `access` is private"],
            &[],
        ),
        fixture(
            "private_corruption_module_cannot_bypass_layout_readmission_facade.rs",
            &["module `readmission` is private"],
            &[],
        ),
        fixture(
            "layout_foundational_closeout_evidence_fields_are_private.rs",
            &["LayoutFoundationalCloseoutEvidence", "private"],
            &["worth_store_certification"],
        ),
        fixture(
            "layout_owner_execution_evidence_fields_are_private.rs",
            &["LayoutOwnerExecutionEvidence", "private"],
            &["worth_store_certification"],
        ),
        fixture(
            "layout_evidence_bundle_fields_are_private.rs",
            &["LayoutEvidenceBundle", "private"],
            &["worth_store_certification"],
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
