use forge_query::facade::ForgeQueryGraphObligationSupportStatus;

use super::execution_inputs::{
    relational_invariant_closeout, relational_invariant_closeout_for_loop_successor_program_slot,
    relational_invariant_closeout_for_rewire_slot,
};

#[test]
fn selected_relational_invariant_families_follow_query_selected_rows() {
    let closeout = relational_invariant_closeout();

    assert!(!closeout.selected_invariant_family_rows().is_empty());
    assert_eq!(
        closeout
            .selected_invariant_family_rows()
            .iter()
            .filter(|row| row.support_status() == ForgeQueryGraphObligationSupportStatus::Supported)
            .count(),
        closeout.selected_invariant_family_rows().len()
    );
    assert_eq!(closeout.claims_invariant_execution_receipts(), false);
    assert_eq!(closeout.counters().execution_receipt_count(), 0);
}

#[test]
fn declared_invariant_family_selects_across_matching_touched_operations() {
    let first = relational_invariant_closeout_for_rewire_slot(30);
    let second = relational_invariant_closeout_for_loop_successor_program_slot(130);

    let first_identities = selected_invariant_identity_digests(&first);
    let second_identities = selected_invariant_identity_digests(&second);

    assert_eq!(first_identities, second_identities);
    assert!(!first_identities.is_empty());
}

fn selected_invariant_identity_digests(
    closeout: &crate::validator_invariant_catalog::WorthTopologyRelationalInvariantCatalogCloseout,
) -> Vec<String> {
    closeout
        .selected_invariant_family_rows()
        .iter()
        .map(|row| row.worth_family_identity_digest().to_string())
        .collect()
}
