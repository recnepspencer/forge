use std::collections::BTreeSet;

use super::{
    adoption_ledger_from_rows, production_phase_two_closeout, read_family_row, requirement_row,
};

#[test]
fn registered_read_family_routes_matching_touched_authorities_without_operator_edits() {
    let ledger = adoption_ledger_from_rows(
        vec![
            read_family_row("catalog-a", "topology_family", "operator-a-touch"),
            read_family_row("catalog-a", "topology_family", "operator-b-touch"),
        ],
        vec![requirement_row(
            "requirement-a",
            "catalog-a",
            "topology_family",
        )],
        &[],
    )
    .expect("one registered read-family identity should route both matching authorities");

    let touched_authorities = ledger
        .pairings()
        .iter()
        .map(|pairing| pairing.touched_authority_input())
        .collect::<BTreeSet<_>>();

    assert_eq!(ledger.pairings().len(), 2);
    assert!(touched_authorities.contains("operator-a-touch"));
    assert!(touched_authorities.contains("operator-b-touch"));
    assert!(ledger
        .pairings()
        .iter()
        .all(|pairing| pairing.query_family_digest_seed() == "topology_family"));
}

#[test]
fn adoption_attempts_are_driven_by_pairings_not_execution_folklore_rows() {
    let closeout = production_phase_two_closeout();
    let pairing_digests = closeout
        .adoption_ledger()
        .pairings()
        .iter()
        .map(|pairing| pairing.pairing_digest())
        .collect::<BTreeSet<_>>();

    assert!(closeout
        .adoption_ledger()
        .attempts()
        .iter()
        .all(|attempt| pairing_digests.contains(attempt.source_pairing_digest())));
}
