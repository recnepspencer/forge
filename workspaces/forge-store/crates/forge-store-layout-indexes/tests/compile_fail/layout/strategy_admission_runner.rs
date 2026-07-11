use super::compile_fail_support;

#[test]
fn strategy_admission_surfaces_reject_forgeable_shortcuts() {
    for fixture in fixtures() {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "strategy_admission",
            fixture.name,
            fixture.expected_stderr,
            &["forge_store_wal"],
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 4] {
    [
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_wal_layout.rs",
            expected_stderr: &["private field", "WalAppendLayoutReport"],
        },
        CompileFailFixture {
            name: "admitted_wal_append_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["private field", "WalAppendLayoutReport"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_wal_tail_layout.rs",
            expected_stderr: &["private field", "WalReplayTailCursorReport"],
        },
        CompileFailFixture {
            name: "admitted_checkpoint_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["private field", "WalReplayTailCursorReport"],
        },
    ]
}
