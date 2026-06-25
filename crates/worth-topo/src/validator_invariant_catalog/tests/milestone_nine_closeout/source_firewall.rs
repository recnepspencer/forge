use crate::validator_invariant_catalog::milestone_nine_closeout::WorthTopologyMilestoneNineSourceFirewallReport;

use super::fixtures::{milestone_nine_closeout, operator_cutover_closeout};

#[test]
fn source_firewall_permits_only_deletion_ledger_paths() {
    let cutover = operator_cutover_closeout();
    let closeout =
        crate::validator_invariant_catalog::WorthTopologyMilestoneNineCloseout::from_operator_cutover(
            cutover.phase_eight_seed(),
            &cutover,
        )
        .expect("Milestone 9 closeout should build");
    assert!(closeout.source_firewall().is_clean());
    assert!(closeout
        .source_firewall()
        .deletion_ledger_allowed_paths()
        .iter()
        .any(|path| path == "validation/rule_registry.rs"));
}

#[test]
fn source_firewall_rejects_old_authority_outside_ledger() {
    let closeout = milestone_nine_closeout();
    let dirty =
        WorthTopologyMilestoneNineSourceFirewallReport::from_source_pairs_with_deletion_ledger(
            [("new_operator_surface.rs", "DERIVED_TOPOLOGY_RULE_SPECS")],
            closeout.deletion_ledger(),
        );
    assert!(!dirty.is_clean());
    assert!(dirty
        .violations()
        .iter()
        .any(|violation| violation.contains("unledgered-occurrence")));
}

#[test]
fn source_firewall_rejects_growth_inside_a_ledgered_legacy_file() {
    let closeout = milestone_nine_closeout();
    let dirty =
        WorthTopologyMilestoneNineSourceFirewallReport::from_source_pairs_with_deletion_ledger(
            [(
                "validation/rule_registry.rs",
                "DERIVED_TOPOLOGY_RULE_SPECS DERIVED_TOPOLOGY_RULE_SPECS DERIVED_TOPOLOGY_RULE_SPECS DERIVED_TOPOLOGY_RULE_SPECS",
            )],
            closeout.deletion_ledger(),
    );
    assert!(!dirty.is_clean());
    assert!(dirty
        .violations()
        .iter()
        .any(|violation| violation.contains("exceeded-ledger-cap")));
}
