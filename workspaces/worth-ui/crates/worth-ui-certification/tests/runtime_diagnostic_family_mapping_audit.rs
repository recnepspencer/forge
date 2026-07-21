use worth_ui_certification::topology::audit_runtime_diagnostic_family_mapping;

#[test]
fn every_runtime_diagnostic_family_has_a_compiler_visible_mapping_home() {
    let violations = audit_runtime_diagnostic_family_mapping(super::workspace_source_inventory());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
