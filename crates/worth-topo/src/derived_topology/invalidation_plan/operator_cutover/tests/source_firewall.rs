use super::super::{
    current_operator_cutover_source_firewall, DerivedInvalidationOperatorCutoverSourceFirewall,
};

#[test]
fn operator_cutover_source_firewall_reports_no_forbidden_old_authority() {
    let firewall = current_operator_cutover_source_firewall();

    assert!(firewall.violations().is_empty());
    assert!(!firewall.report_digest().is_empty());
}

#[test]
fn operator_cutover_source_firewall_rejects_operator_authored_dirty_lists() {
    let firewall = DerivedInvalidationOperatorCutoverSourceFirewall::from_sources_for_tests([(
        "topology_operators/application/mod.rs",
        "let _old_authority = operator_dirty_products;",
    )]);

    assert_eq!(firewall.violations().len(), 1);
    assert_eq!(
        firewall.violations()[0].forbidden_surface(),
        "operator_dirty_products"
    );
}

#[test]
fn operator_cutover_source_firewall_rejects_projection_dirty_expansion() {
    let firewall = DerivedInvalidationOperatorCutoverSourceFirewall::from_sources_for_tests([(
        "projection/runtime_boundary/read_stage.rs",
        "fn legacy_path() { expand_dirty_scope(); }",
    )]);

    assert_eq!(firewall.violations().len(), 1);
    assert_eq!(
        firewall.violations()[0].forbidden_surface(),
        "expand_dirty_scope"
    );
}
