use super::compile_fail_support;

#[test]
fn public_facade_denies_flat_root_bypass() {
    for fixture in compile_fail_fixtures() {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "public_facade",
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

fn compile_fail_fixtures() -> [CompileFailFixture; 6] {
    [
        fixture(
            "root_access_shape_is_not_public.rs",
            &["private", "BTreeLookupReady"],
            &[],
        ),
        fixture(
            "root_access_shape_projection_is_removed.rs",
            &["no `AccessShape` in the root"],
            &[],
        ),
        fixture(
            "root_snapshot_rule_is_not_public.rs",
            &["AdmittedSnapshotLayoutRule"],
            &[],
        ),
        fixture(
            "root_phase_obligation_row_is_not_public.rs",
            &["S8PhaseSkeletonObligationRow"],
            &[],
        ),
        fixture(
            "root_layout_readmission_witness_is_not_public.rs",
            &["LayoutReadmissionWitness"],
            &[],
        ),
        fixture(
            "root_layout_customization_request_is_not_public.rs",
            &["FutureLayoutCustomizationRequest"],
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
