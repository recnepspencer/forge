use super::compile_fail_support;

#[test]
fn recovery_shortcut_witness_surfaces_are_not_public() {
    for fixture in fixtures() {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "foundations",
            fixture.name,
            fixture.expected_stderr,
            &["worth_store_recovery_physics", "worth_store_contracts"],
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 3] {
    [
        CompileFailFixture {
            name: "recovery_quarantine_shortcut_witness_surface_is_not_public.rs",
            expected_stderr: &["private field", "PartialPublicationReplayReadWitness"],
        },
        CompileFailFixture {
            name: "recovery_import_shortcut_witness_surface_is_not_public.rs",
            expected_stderr: &["private field", "BoundedRecoverySourceAdmission"],
        },
        CompileFailFixture {
            name: "layout_quarantine_readmission_shortcut_surface_is_not_public.rs",
            expected_stderr: &["private field", "LayoutReadmissionWitness"],
        },
    ]
}
