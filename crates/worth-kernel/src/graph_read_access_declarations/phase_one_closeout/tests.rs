use crate::graph_read_access_declarations::{
    current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six,
    WorthGraphReadAccessDeclarationPhaseOneErrorKind,
};
use crate::graph_read_access_inventory::{
    current_worth_graph_read_access_milestone_six_closeout_for_tests,
    uncapped_old_graph_read_folklore_milestone_seven_seed_for_tests,
};

#[test]
fn declaration_lane_accepts_milestone_six_closeout_public_boundary() {
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    let seed = milestone_six.milestone_seven_seed();
    let closeout =
        current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six(
            &milestone_six,
        )
        .expect("Milestone 6 seed should admit into declaration Phase 1 closeout");

    assert_eq!(
        closeout.declaration_candidates(),
        seed.declaration_candidates()
    );
    assert_eq!(closeout.capability_gaps(), seed.capability_gaps());
    assert_eq!(closeout.deletion_items(), seed.deletion_items());
}

#[test]
fn declaration_lane_preserves_milestone_six_old_folklore_audit() {
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    let seed = milestone_six.milestone_seven_seed();
    let closeout =
        current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six(
            &milestone_six,
        )
        .expect("Milestone 6 seed should admit into declaration Phase 1 closeout");

    assert!(!seed.contains_uncapped_old_graph_read_folklore_as_declaration_or_gap());
    assert!(seed.deletion_items().iter().any(|item| {
        item.inventory_row_identity().source_path()
            == "crates/worth-kernel/src/query_adoption/graph_read_access"
    }));
    assert!(closeout.declaration_candidates().iter().all(|candidate| {
        candidate.inventory_row_identity().source_path()
            != "crates/worth-kernel/src/query_adoption/graph_read_access"
    }));
    assert!(closeout.capability_gaps().iter().all(|gap| {
        gap.inventory_row_identity().source_path()
            != "crates/worth-kernel/src/query_adoption/graph_read_access"
    }));
    assert!(closeout.deletion_items().iter().any(|item| {
        item.inventory_row_identity().source_path()
            == "crates/worth-kernel/src/query_adoption/graph_read_access"
    }));
}

#[test]
fn milestone_seven_seed_cannot_claim_execution_authority() {
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    let closeout =
        current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six(
            &milestone_six,
        )
        .expect("honest seed should admit");

    assert!(!milestone_six
        .milestone_seven_seed()
        .claims_execution_authority());
    assert!(!closeout.claims_execution_authority());
    assert!(!closeout.claims_query_declarations_complete());
    assert!(!closeout.claims_admitted_access_plans_complete());
    assert!(!closeout.claims_graph_read_receipts_complete());
}

#[test]
fn declaration_lane_preserves_seed_counts_exactly() {
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    let seed = milestone_six.milestone_seven_seed();
    let closeout =
        current_worth_graph_read_access_declaration_phase_one_closeout_from_milestone_six(
            &milestone_six,
        )
        .expect("Milestone 6 seed should admit into declaration Phase 1 closeout");

    assert_eq!(
        closeout.counters().declaration_candidate_count(),
        seed.declaration_candidates().len()
    );
    assert_eq!(
        closeout.counters().capability_gap_count(),
        seed.capability_gaps().len()
    );
    assert_eq!(
        closeout.counters().deletion_item_count(),
        seed.deletion_items().len()
    );
    assert_eq!(
        closeout.counters().excluded_certification_only_count(),
        seed.counters().excluded_certification_only_count()
    );
    assert_eq!(
        closeout.counters().excluded_out_of_scope_count(),
        seed.counters().excluded_out_of_scope_count()
    );
    assert_eq!(
        closeout.declaration_candidates().len(),
        seed.declaration_candidates().len()
    );
    assert_eq!(
        closeout.capability_gaps().len(),
        seed.capability_gaps().len()
    );
    assert_eq!(closeout.deletion_items().len(), seed.deletion_items().len());
}

#[test]
fn internal_seed_admission_rejects_uncapped_old_folklore() {
    let bad_seed = uncapped_old_graph_read_folklore_milestone_seven_seed_for_tests();

    assert!(!bad_seed.claims_execution_authority());
    assert!(bad_seed.contains_uncapped_old_graph_read_folklore_as_declaration_or_gap());

    let error = super::closeout::closeout_from_milestone_seven_seed(&bad_seed)
        .expect_err("uncapped old graph-read folklore must not admit into Phase 1");
    assert_eq!(
        error.kind(),
        WorthGraphReadAccessDeclarationPhaseOneErrorKind::SeedContainsUncappedOldGraphReadFolklore
    );
}

#[test]
fn declaration_lane_public_surface_does_not_expose_execution_vocabulary() {
    let declaration_sources = [
        include_str!("../mod.rs"),
        include_str!("closeout.rs"),
        include_str!("counters.rs"),
        include_str!("errors.rs"),
        include_str!("../seed_contract/mod.rs"),
        include_str!("../seed_contract/admitted_seed.rs"),
    ];
    let forbidden_public_fragments = [
        "pub fn execute",
        "pub fn runtime_graph_read",
        "pub fn graph_read_access_plan_consumption",
        "pub fn graph_read_receipt",
        "pub fn graph_read_streaming_receipt",
        "pub fn ephemeral_graph_index_receipt",
        "pub fn plan_admitted",
        "pub fn consume_plan",
        "pub fn local_graph_walk",
        "pub fn fallback_traversal",
        "pub struct ForgeQueryReadReceipt",
        "pub struct ForgeQueryAdmittedGraphReadAccessPlan",
    ];

    for source in declaration_sources {
        for forbidden in forbidden_public_fragments {
            assert!(
                !source.contains(forbidden),
                "Phase 1 declaration lane must not expose execution-shaped public surface: {forbidden}"
            );
        }
    }
}
