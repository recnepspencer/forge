use crate::validator_invariant_catalog::WorthTopologySelectedGraphObligationEnforcementSourceFirewallReport;

#[test]
fn source_firewall_flags_injected_local_ceremony_residue() {
    let report =
        WorthTopologySelectedGraphObligationEnforcementSourceFirewallReport::from_source_pairs([
            (
                "clean.rs",
                "fn uses_query_consumer_kit_boundary_without_local_ceremony() {}",
            ),
            (
                "dirty.rs",
                "fn fabricated_graph_obligation_receipt() { let _ = \"private_legality_graph\"; }",
            ),
        ]);

    assert_eq!(report.scanned_file_count(), 2);
    assert!(!report.is_clean());
    assert_eq!(report.violations().len(), 2);
    assert!(report
        .violations()
        .iter()
        .any(|violation| violation == "dirty.rs::fabricated_graph_obligation_receipt"));
    assert!(report
        .violations()
        .iter()
        .any(|violation| violation == "dirty.rs::private_legality_graph"));
    assert!(report.report_digest().contains("scanned-file-count:2"));
}
