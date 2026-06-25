use super::super::source_firewall::scan_source;

#[test]
fn source_firewall_rejects_manual_plan_hint_residue() {
    for needle in [
        "operator_read_plan_hint",
        "local_access_mode_switch",
        "execution_strategy_hint_enum",
        "manual_read_plan_list",
        "manual_read_plan()",
    ] {
        let err = scan_source(
            "manual_hint.rs",
            &format!("const RESIDUE: &str = \"{needle}\";"),
        )
        .expect_err("manual strategy rediscovery residue must be rejected");
        assert!([
            "operator_read_plan_hint",
            "local_access_mode_switch",
            "execution_strategy_hint_enum",
            "manual_read_plan_list",
            "manual_read_plan_call",
        ]
        .contains(&err.forbidden_pattern()));
    }
}
