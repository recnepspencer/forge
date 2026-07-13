use super::compile_fail_support;

#[test]
fn layout_migration_surfaces_reject_raw_struct_shortcuts() {
    for fixture in fixtures() {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "migration",
            fixture.name,
            fixture.expected_stderr,
            &[],
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 2] {
    [
        CompileFailFixture {
            name: "layout_migration_plan_struct_literal_is_not_public.rs",
            expected_stderr: &["private", "LayoutMigrationPlan"],
        },
        CompileFailFixture {
            name: "layout_rebind_required_struct_literal_is_not_public.rs",
            expected_stderr: &["private", "LayoutRebindRequired"],
        },
    ]
}
