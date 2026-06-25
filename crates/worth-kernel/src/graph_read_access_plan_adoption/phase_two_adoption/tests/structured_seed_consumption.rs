use super::{
    adoption_ledger_from_rows, production_phase_two_closeout, read_family_row, requirement_row,
};

#[test]
fn phase_two_consumes_structured_seed_rows_without_digest_only_matching() {
    let ledger = adoption_ledger_from_rows(
        vec![
            read_family_row("catalog-a", "topology_family", "topology-authority"),
            read_family_row("catalog-b", "topology_family", "near-miss-authority"),
        ],
        vec![requirement_row(
            "requirement-a",
            "catalog-a",
            "topology_family",
        )],
        &[],
    )
    .expect("only the structurally matching seed rows should pair");

    assert_eq!(
        ledger.pairings().len(),
        1,
        "mismatched catalog identity must not pair through family-name coincidence"
    );
    assert!(
        ledger
            .pairings()
            .iter()
            .all(|pairing| pairing.source_catalog_record_digest()
                != pairing.source_requirement_record_digest()),
        "pairing must retain both catalog identity and requirement-row identity"
    );
    assert!(ledger.pairings().iter().all(|pairing| {
        !pairing.read_family_identity_digest().is_empty()
            && !pairing.requirement_row_digest().is_empty()
            && !pairing.query_family_digest_seed().is_empty()
    }));
}

#[test]
fn phase_two_rejects_seed_rows_that_only_match_by_unstructured_digest_text() {
    let error = adoption_ledger_from_rows(
        vec![read_family_row(
            "catalog-a",
            "topology_family",
            "topology-authority",
        )],
        vec![requirement_row(
            "requirement-a",
            "catalog-b",
            "topology_family",
        )],
        &[],
    )
    .expect_err("near-miss structured seed rows must fail closed");

    assert_eq!(
        error.kind(),
        super::super::errors::WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind::MissingStructuredSeedPairing
    );
}

#[test]
fn phase_two_ledger_preserves_phase_one_closeout_identity() {
    let closeout = production_phase_two_closeout();

    assert!(closeout
        .adoption_ledger()
        .pairings()
        .iter()
        .all(|pairing| pairing.milestone_seven_closeout_digest()
            == closeout.phase_one_closeout_digest()));
    assert!(!closeout.adoption_ledger().ledger_digest().is_empty());
    assert!(!closeout.posture_report().report_digest().is_empty());
    assert!(!closeout.closeout_digest().is_empty());
}
