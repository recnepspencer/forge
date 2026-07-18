use super::compile_fail_support;

#[test]
fn caller_defined_rules_cannot_open_replay_layouts() {
    compile_fail_support::assert_compile_fails_in_ui_dir(
        "btree",
        "caller_defined_rule_cannot_open_replay_layout.rs",
        &["BoundedRecoverySourceAdmission", "private field"],
        &["worth_store_recovery_physics"],
    );
}
