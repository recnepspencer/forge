use super::compile_fail_support;

#[test]
fn bootstrap_catalog_surfaces_reject_raw_struct_shortcuts() {
    for fixture in fixtures() {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "bootstrap",
            fixture.name,
            fixture.expected_stderr,
            &["forge_store_physical_format"],
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
            name: "bootstrap_catalog_struct_literal_is_not_public.rs",
            expected_stderr: &["private", "BootstrapLayoutCatalog"],
        },
        CompileFailFixture {
            name: "bootstrap_catalog_read_admission_struct_literal_is_not_public.rs",
            expected_stderr: &["private", "BootstrapCatalogReadAdmission"],
        },
        CompileFailFixture {
            name: "raw_persisted_layout_cannot_reopen_bootstrap_lane.rs",
            expected_stderr: &["PlatformPhysicalReplayArtifact", "mismatched types"],
        },
    ]
}
