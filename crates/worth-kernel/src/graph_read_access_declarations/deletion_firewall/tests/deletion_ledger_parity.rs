use std::collections::BTreeSet;

use super::deletion_fingerprints::{
    deletion_item_fingerprints, deletion_item_source_paths, deletion_ledger_row_digests,
    deletion_ledger_row_fingerprints, old_graph_read_source_path,
};
use super::phase_chain_fixture::phase_six_seed_from_seed;
use crate::graph_read_access_declarations::{
    current_worth_graph_read_access_declaration_catalog_closeout,
    current_worth_graph_read_declaration_deletion_firewall_closeout,
    current_worth_graph_read_requirement_derivation_closeout,
    phase_one_closeout_from_milestone_seven_seed_for_tests,
    WorthGraphReadDeclarationDeletionStatus,
};
use crate::graph_read_access_inventory::current_worth_graph_read_access_milestone_six_closeout_for_tests;

#[test]
fn deletion_ledger_matches_milestone_six_items() {
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    let phase_six_seed = phase_six_seed_from_seed(&milestone_six.milestone_seven_seed());
    let closeout = current_worth_graph_read_declaration_deletion_firewall_closeout(&phase_six_seed)
        .expect("Phase 6 should close over production deletion proof");
    let repeated_closeout =
        current_worth_graph_read_declaration_deletion_firewall_closeout(&phase_six_seed)
            .expect("Phase 6 deletion proof should be stable");

    assert_eq!(
        deletion_item_fingerprints(milestone_six.milestone_seven_seed().deletion_items()),
        deletion_ledger_row_fingerprints(closeout.deletion_ledger_report().rows())
    );
    assert_eq!(closeout.deletion_ledger_report().capped_residue_count(), 0);
    assert!(closeout
        .deletion_ledger_report()
        .rows()
        .iter()
        .all(|row| row.status() == WorthGraphReadDeclarationDeletionStatus::Deleted));
    assert_eq!(
        deletion_ledger_row_digests(closeout.deletion_ledger_report().rows()),
        deletion_ledger_row_digests(repeated_closeout.deletion_ledger_report().rows())
    );
}

#[test]
fn replacement_phases_carry_deletion_or_residue_proof() {
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    let phase_one = phase_one_closeout_from_milestone_seven_seed_for_tests(
        &milestone_six.milestone_seven_seed(),
    )
    .expect("Milestone 7 seed should admit");
    let phase_two = current_worth_graph_read_access_declaration_catalog_closeout(&phase_one)
        .expect("Phase 2 catalog should build");
    let phase_four = current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 4 requirement derivation should build");
    let phase_six_seed =
        crate::graph_read_access_declarations::current_worth_graph_read_access_admission_posture_closeout(
            phase_four.phase_five_seed(),
        )
        .expect("Phase 5 admission posture should build")
        .phase_six_seed()
        .clone();
    let deletion_sources = deletion_item_source_paths(phase_six_seed.deletion_items());

    assert!(!phase_six_seed.deletion_items().is_empty());
    assert!(phase_two
        .declaration_catalog()
        .records()
        .iter()
        .flat_map(|record| record.source_row_identities())
        .all(|identity| !deletion_sources.contains(identity.source_path())));
    assert!(phase_four.requirement_records().iter().all(|record| record
        .requirement_source_trace()
        .source_row_identities()
        .iter()
        .all(|identity| !deletion_sources.contains(identity.source_path()))));
    assert_eq!(
        deletion_sources,
        BTreeSet::from([old_graph_read_source_path()])
    );
}
