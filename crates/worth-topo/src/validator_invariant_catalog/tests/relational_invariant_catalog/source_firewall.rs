use super::execution_inputs::relational_invariant_closeout;
use crate::validator_invariant_catalog::WorthTopologyRelationalInvariantCatalogSourceFirewallReport;

#[test]
fn relational_invariant_catalog_source_firewall_scans_lane_and_finds_no_residue() {
    let closeout = relational_invariant_closeout();
    let firewall = closeout.source_firewall();

    assert!(firewall.scanned_file_count() >= 8);
    assert!(firewall.forbidden_token_count() >= 4);
    assert!(firewall.violations().is_empty());
    assert_eq!(closeout.counters().source_firewall_violation_count(), 0);
}

#[test]
fn relational_invariant_catalog_source_firewall_rejects_injected_old_authority() {
    let firewall =
        WorthTopologyRelationalInvariantCatalogSourceFirewallReport::from_source_pairs([
            (
                "hostile_static_pack.rs",
                "fn bad() { let _ = milestone_one_runtime_builder(); }",
            ),
            (
                "hostile_custom_registration.rs",
                "fn bad() { let _ = CustomInvariantRegistration::new(rule); }",
            ),
        ]);

    assert_eq!(firewall.scanned_file_count(), 2);
    assert_eq!(firewall.violations().len(), 2);
    assert!(firewall
        .violations()
        .iter()
        .any(|violation| violation.contains("milestone_one_runtime_builder")));
    assert!(firewall
        .violations()
        .iter()
        .any(|violation| violation.contains("CustomInvariantRegistration::new")));
}
