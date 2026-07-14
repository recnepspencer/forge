use super::compile_fail_support;

#[test]
fn btree_execution_rejects_coarse_and_seeded_production_shortcuts() {
    for fixture in fixtures() {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "counter_evidence",
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

const fn fixtures() -> [CompileFailFixture; 4] {
    [
        CompileFailFixture {
            name: "coarse_execution_counter_lane_is_not_public.rs",
            expected_stderr: &["no method named", "counters"],
        },
        CompileFailFixture {
            name: "seeded_execution_helper_lane_is_not_public.rs",
            expected_stderr: &["unresolved import", "execute_baseline_btree_point_lookup"],
        },
        CompileFailFixture {
            name: "seeded_execution_transcript_lane_is_not_public.rs",
            expected_stderr: &["unresolved import", "execute_baseline_btree_transcript"],
        },
        CompileFailFixture {
            name: "layout_access_performance_receipt_fields_are_private.rs",
            expected_stderr: &["LayoutAccessPerformanceReceipt", "private"],
        },
    ]
}
