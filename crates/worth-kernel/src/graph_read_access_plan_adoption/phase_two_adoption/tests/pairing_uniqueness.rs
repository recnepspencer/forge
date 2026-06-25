use super::{
    adoption_ledger_from_rows, production_phase_one_closeout, read_family_row, requirement_row,
};

#[test]
fn duplicate_structured_seed_pairings_fail_closed() {
    let duplicate_family = read_family_row("catalog-a", "topology_family", "operator-a-touch");
    let error = adoption_ledger_from_rows(
        vec![duplicate_family.clone(), duplicate_family],
        vec![requirement_row(
            "requirement-a",
            "catalog-a",
            "topology_family",
        )],
        &[],
    )
    .expect_err("duplicate read-family rows must not create duplicate adoption attempts");

    assert_eq!(
        error.kind(),
        super::super::errors::WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind::DuplicateStructuredSeedPairing
    );
}

#[test]
fn carried_capability_gaps_remain_visible_as_phase_two_rows() {
    let phase_one = production_phase_one_closeout();
    let ledger = adoption_ledger_from_rows(
        phase_one.read_family_identities().to_vec(),
        phase_one.requirement_row_evidence().to_vec(),
        phase_one.admission_capability_gaps(),
    )
    .expect("production rows should build a Phase 2 adoption ledger");

    assert_eq!(
        ledger.carried_capability_gaps().len(),
        phase_one.admission_capability_gaps().len()
    );
    assert_eq!(
        ledger.carried_capability_gap_count(),
        phase_one.admission_capability_gaps().len()
    );
    assert!(ledger.carried_capability_gaps().iter().all(|gap| {
        !gap.source_gap_digest().is_empty()
            && !gap.source_requirement_record_digest().is_empty()
            && !gap.query_family_anchor_digest().is_empty()
            && !gap.owner().is_empty()
            && !gap.blocker().is_empty()
            && !gap.removal_trigger().is_empty()
            && !gap.row_digest().is_empty()
    }));
}
