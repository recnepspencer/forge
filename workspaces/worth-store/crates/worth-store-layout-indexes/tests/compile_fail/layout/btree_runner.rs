use super::compile_fail_support;

#[test]
fn btree_surfaces_reject_forgeable_admission_shortcuts() {
    for fixture in fixtures() {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "btree",
            fixture.name,
            fixture.expected_stderr,
            &["worth_store_recovery_physics"],
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 6] {
    [
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_replay_layout.rs",
            expected_stderr: &["BoundedRecoverySourceAdmission", "private field"],
        },
        CompileFailFixture {
            name: "admitted_replay_index_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["BoundedRecoverySourceAdmission", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_crash_boundary_layout.rs",
            expected_stderr: &["PartialPublicationReplayReadWitness", "private field"],
        },
        CompileFailFixture {
            name: "admitted_readmission_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["PartialPublicationReplayReadWitness", "private field"],
        },
        CompileFailFixture {
            name: "recovery_source_precedence_graph_is_not_public_without_certification.rs",
            expected_stderr: &["BoundedRecoverySourceAdmission", "private field"],
        },
        CompileFailFixture {
            name: "partial_publication_classification_is_not_public_without_certification.rs",
            expected_stderr: &["PartialPublicationReplayReadWitness", "private field"],
        },
    ]
}
