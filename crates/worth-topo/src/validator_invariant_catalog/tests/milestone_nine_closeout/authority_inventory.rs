use crate::validator_invariant_catalog::{
    WorthTopologyMilestoneNineAuthorityOccurrenceInventory,
    WorthTopologyMilestoneNineAuthorityOccurrenceStatus,
    WorthTopologyMilestoneNineSourceFirewallReport,
};

use super::fixtures::milestone_nine_closeout;

#[test]
fn current_authority_occurrences_are_all_ledgered_within_cap() {
    let closeout = milestone_nine_closeout();
    let inventory =
        WorthTopologyMilestoneNineAuthorityOccurrenceInventory::current_from_deletion_ledger(
            closeout.deletion_ledger(),
        );
    assert!(!inventory.rows().is_empty());
    assert!(inventory.rows().iter().all(|row| {
        row.status() == WorthTopologyMilestoneNineAuthorityOccurrenceStatus::LedgeredWithinCap
    }));
    assert_eq!(inventory.violation_rows().len(), 0);
}

#[test]
fn ledger_cap_growth_fails_inventory_and_firewall() {
    let closeout = milestone_nine_closeout();
    let inventory =
        WorthTopologyMilestoneNineAuthorityOccurrenceInventory::from_source_pairs_and_deletion_ledger(
            [(
                "validation/rule_registry.rs",
                "DERIVED_TOPOLOGY_RULE_SPECS DERIVED_TOPOLOGY_RULE_SPECS DERIVED_TOPOLOGY_RULE_SPECS DERIVED_TOPOLOGY_RULE_SPECS",
            )],
            closeout.deletion_ledger(),
            WorthTopologyMilestoneNineSourceFirewallReport::forbidden_authority_patterns(),
        );
    assert!(inventory.violation_rows().iter().any(|row| {
        row.status() == WorthTopologyMilestoneNineAuthorityOccurrenceStatus::ExceededLedgerCap
    }));
    let firewall =
        WorthTopologyMilestoneNineSourceFirewallReport::from_source_pairs_with_deletion_ledger(
            [(
                "validation/rule_registry.rs",
                "DERIVED_TOPOLOGY_RULE_SPECS DERIVED_TOPOLOGY_RULE_SPECS DERIVED_TOPOLOGY_RULE_SPECS DERIVED_TOPOLOGY_RULE_SPECS",
            )],
            closeout.deletion_ledger(),
        );
    assert!(firewall
        .violations()
        .iter()
        .any(|violation| violation.contains("exceeded-ledger-cap")));
}

#[test]
fn unledgered_new_authority_fails_inventory() {
    let closeout = milestone_nine_closeout();
    let inventory =
        WorthTopologyMilestoneNineAuthorityOccurrenceInventory::from_source_pairs_and_deletion_ledger(
            [("new_operator_surface.rs", "DERIVED_TOPOLOGY_RULE_SPECS")],
            closeout.deletion_ledger(),
            WorthTopologyMilestoneNineSourceFirewallReport::forbidden_authority_patterns(),
        );
    assert!(inventory.violation_rows().iter().any(|row| {
        row.status() == WorthTopologyMilestoneNineAuthorityOccurrenceStatus::UnledgeredOccurrence
    }));
}
