use crate::graph_read_access_declarations::{
    current_worth_graph_read_access_declaration_catalog_closeout,
    current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six,
    phase_one_closeout_from_milestone_seven_seed_for_tests,
    WorthGraphReadAccessDeclarationPhaseOneCloseout,
    WorthGraphReadAccessDeclarationPhaseTwoCloseout,
    WorthGraphReadAccessDeclarationPhaseTwoErrorKind,
};
use crate::graph_read_access_inventory::{
    conflicting_requirement_milestone_seven_seed_for_tests,
    current_worth_graph_read_access_milestone_six_closeout_for_tests,
    same_family_multiple_callers_milestone_seven_seed_for_tests,
    same_family_multiple_callers_reversed_milestone_seven_seed_for_tests,
    same_semantics_different_provenance_milestone_seven_seeds_for_tests,
    semantic_authority_pair_milestone_seven_seeds_for_tests,
    WorthGraphReadAccessMilestoneSevenSeed,
};

mod authority_lowering;

#[test]
fn catalog_registers_each_seed_candidate_once() {
    let phase_one = phase_one_closeout();
    let phase_two = phase_two_closeout(&phase_one);

    assert_eq!(
        phase_two.catalog_summary().source_candidate_count(),
        phase_one.declaration_candidates().len()
    );
    assert_eq!(
        phase_two.catalog_summary().catalog_record_count(),
        phase_two.declaration_catalog().records().len()
    );
    assert_eq!(
        phase_two.catalog_summary().catalog_digest(),
        phase_two.declaration_catalog().catalog_digest()
    );
    assert!(phase_two
        .declaration_catalog()
        .records()
        .iter()
        .all(|record| record.key().has_complete_declaration_dimensions()));
    assert!(!phase_two.claims_execution_authority());
    assert!(!phase_two.claims_admitted_access_plans_complete());
    assert!(!phase_two.claims_graph_read_receipts_complete());
}

#[test]
fn catalog_rejects_conflicting_touched_authority_keys() {
    let phase_one =
        phase_one_closeout_from_seed(&conflicting_requirement_milestone_seven_seed_for_tests());

    let error = current_worth_graph_read_access_declaration_catalog_closeout(&phase_one)
        .expect_err("same touched authority/read shape with different requirements must conflict");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessDeclarationPhaseTwoErrorKind::ConflictingTouchedAuthorityReadShape
    );
}

#[test]
fn one_catalog_family_can_cover_multiple_callers() {
    let phase_one = phase_one_closeout_from_seed(
        &same_family_multiple_callers_milestone_seven_seed_for_tests(),
    );
    let phase_two = phase_two_closeout(&phase_one);

    assert_eq!(phase_two.catalog_summary().source_candidate_count(), 2);
    assert_eq!(phase_two.catalog_summary().catalog_record_count(), 1);
    assert_eq!(phase_two.catalog_summary().merged_source_row_count(), 1);

    let record = only_catalog_record(&phase_two);
    let identities = record.source_row_identities();
    assert_eq!(record.source_candidate_count(), 2);
    assert_ne!(
        identities[0].current_caller(),
        identities[1].current_caller()
    );
    assert_ne!(identities[0].source_path(), identities[1].source_path());
    assert!(!record
        .query_family_anchor()
        .claims_query_read_family_constructed());
}

#[test]
fn catalog_identity_is_stable_under_source_ordering() {
    let forward = phase_two_closeout(&phase_one_closeout_from_seed(
        &same_family_multiple_callers_milestone_seven_seed_for_tests(),
    ));
    let reversed = phase_two_closeout(&phase_one_closeout_from_seed(
        &same_family_multiple_callers_reversed_milestone_seven_seed_for_tests(),
    ));

    assert_eq!(
        forward.declaration_catalog().catalog_digest(),
        reversed.declaration_catalog().catalog_digest()
    );
    assert_eq!(
        only_catalog_record(&forward).declaration_identity_digest(),
        only_catalog_record(&reversed).declaration_identity_digest()
    );
}

#[test]
fn catalog_identity_uses_semantic_authority_not_source_provenance() {
    let (same_semantics_left, same_semantics_right) =
        same_semantics_different_provenance_milestone_seven_seeds_for_tests();
    let left = phase_two_closeout(&phase_one_closeout_from_seed(&same_semantics_left));
    let right = phase_two_closeout(&phase_one_closeout_from_seed(&same_semantics_right));

    assert_eq!(
        only_catalog_record(&left).declaration_identity_digest(),
        only_catalog_record(&right).declaration_identity_digest()
    );
    assert_ne!(
        only_catalog_record(&left).source_row_identities(),
        only_catalog_record(&right).source_row_identities()
    );

    let (authority_a, authority_b) = semantic_authority_pair_milestone_seven_seeds_for_tests();
    let authority_a = phase_two_closeout(&phase_one_closeout_from_seed(&authority_a));
    let authority_b = phase_two_closeout(&phase_one_closeout_from_seed(&authority_b));
    assert_ne!(
        only_catalog_record(&authority_a).declaration_identity_digest(),
        only_catalog_record(&authority_b).declaration_identity_digest()
    );
}

#[test]
fn catalog_record_requires_complete_declaration_dimensions() {
    let phase_two = phase_two_closeout(&phase_one_closeout());

    assert!(phase_two
        .declaration_catalog()
        .records()
        .iter()
        .all(|record| record.key().has_complete_declaration_dimensions()));
}

#[test]
fn phase_three_seed_reuses_phase_two_catalog_identity() {
    let phase_one = phase_one_closeout();
    let phase_two = phase_two_closeout(&phase_one);

    assert_eq!(
        phase_two.declaration_catalog().catalog_digest(),
        phase_two
            .milestone_seven_phase_three_seed()
            .catalog_digest()
    );
    assert_eq!(
        phase_two.declaration_catalog().records(),
        phase_two
            .milestone_seven_phase_three_seed()
            .catalog_records()
    );
    assert!(!phase_two
        .milestone_seven_phase_three_seed()
        .claims_execution_authority());
}

fn phase_one_closeout() -> WorthGraphReadAccessDeclarationPhaseOneCloseout {
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six(
        &milestone_six,
    )
    .expect("Milestone 6 fixture should admit into Phase 1")
}

pub(crate) fn phase_one_closeout_from_seed(
    seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> WorthGraphReadAccessDeclarationPhaseOneCloseout {
    phase_one_closeout_from_milestone_seven_seed_for_tests(seed)
        .expect("Milestone 7 fixture should admit into Phase 1")
}

pub(crate) fn phase_two_closeout(
    phase_one: &WorthGraphReadAccessDeclarationPhaseOneCloseout,
) -> WorthGraphReadAccessDeclarationPhaseTwoCloseout {
    current_worth_graph_read_access_declaration_catalog_closeout(phase_one)
        .expect("Phase 1 declaration candidates should build a Phase 2 catalog")
}

pub(crate) fn only_catalog_record(
    phase_two: &WorthGraphReadAccessDeclarationPhaseTwoCloseout,
) -> &super::catalog_record::WorthGraphReadDeclarationCatalogRecord {
    let records = phase_two.declaration_catalog().records();
    assert_eq!(records.len(), 1);
    &records[0]
}
