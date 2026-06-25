use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
};

use super::super::super::capability_gaps::{
    WorthGraphReadExpectedDenial, WorthGraphReadMissingQueryCapability,
};
use super::super::super::current_worth_graph_read_access_surface_inventory_for_tests;
use super::super::super::inventory_lane::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessCostPosture,
    WorthGraphReadAccessInventoryCloseout, WorthGraphReadAccessInventoryRow,
    WorthGraphReadAccessInventorySeed,
};
use super::super::WorthGraphReadAccessPhaseSixCloseout;

#[test]
fn inventory_conversion_builds_seed_from_current_inventory_catalog() {
    let inventory = phase_six_inventory();
    let closeout = WorthGraphReadAccessPhaseSixCloseout::from_inventory(&inventory)
        .expect("current graph-read inventory should convert into Phase 6 ledger rows");

    assert_eq!(closeout.counters().declaration_candidate_count(), 5);
    assert_eq!(closeout.counters().capability_gap_count(), 2);
    assert_eq!(closeout.counters().deletion_item_count(), 1);
    assert_eq!(closeout.counters().excluded_certification_only_count(), 4);
    assert_eq!(closeout.counters().excluded_out_of_scope_count(), 0);

    let seed = closeout.milestone_seven_seed();
    assert_eq!(seed.declaration_candidates().len(), 5);
    assert_eq!(seed.capability_gaps().len(), 2);
    assert_eq!(seed.deletion_items().len(), 1);
    assert!(!seed.claims_execution_authority());
    assert!(!seed.contains_uncapped_old_graph_read_folklore_as_declaration_or_gap());
}

#[test]
fn inventory_conversion_preserves_inventory_row_context_for_required_rows() {
    let inventory = phase_six_inventory();
    let closeout = WorthGraphReadAccessPhaseSixCloseout::from_inventory(&inventory)
        .expect("current graph-read inventory should convert into Phase 6 ledger rows");

    for row in required_rows(&inventory) {
        let match_count = closeout_context_match_count(&closeout, row);
        assert_eq!(
            match_count,
            1,
            "Phase 6 ledger should contain exactly one row for {}",
            row.source_path()
        );
    }
}

#[test]
fn inventory_conversion_makes_broad_and_frontier_reads_query_capability_gaps() {
    let inventory = phase_six_inventory();
    let closeout = WorthGraphReadAccessPhaseSixCloseout::from_inventory(&inventory)
        .expect("current graph-read inventory should convert into Phase 6 ledger rows");

    let broad_boolean_row = row_for_path(
        &inventory,
        "crates/worth-spatial/src/workload_platform/planar_boolean_events",
    );
    let frontier_boolean_row = row_for_path(
        &inventory,
        "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction",
    );

    assert_gap_is_persistent_continuation_index(&closeout, broad_boolean_row);
    assert_gap_is_persistent_continuation_index(&closeout, frontier_boolean_row);
}

#[test]
fn inventory_conversion_keeps_old_adoption_as_deletion_only() {
    let inventory = phase_six_inventory();
    let closeout = WorthGraphReadAccessPhaseSixCloseout::from_inventory(&inventory)
        .expect("current graph-read inventory should convert into Phase 6 ledger rows");
    let old_adoption = row_for_path(
        &inventory,
        "crates/worth-kernel/src/query_adoption/graph_read_access",
    );

    assert_eq!(closeout_context_match_count(&closeout, old_adoption), 1);
    assert!(closeout
        .deletion_items()
        .iter()
        .any(|item| context_matches_row(item.inventory_row_context(), old_adoption)));
    assert!(!closeout
        .declaration_candidates()
        .iter()
        .any(|candidate| context_matches_row(candidate.inventory_row_context(), old_adoption)));
    assert!(!closeout
        .capability_gaps()
        .iter()
        .any(|gap| context_matches_row(gap.inventory_row_context(), old_adoption)));
    assert!(!closeout.claims_execution_authority());
    assert!(!closeout.contains_uncapped_old_graph_read_folklore_as_declaration_or_gap());
    assert!(!closeout.claims_later_milestone_completion());
}

fn phase_six_inventory() -> WorthGraphReadAccessInventoryCloseout {
    current_worth_graph_read_access_surface_inventory_for_tests(
        WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .expect("test inventory should close before Phase 6 ledger conversion")
}

fn required_rows(
    inventory: &WorthGraphReadAccessInventoryCloseout,
) -> impl Iterator<Item = &WorthGraphReadAccessInventoryRow> {
    inventory.rows().iter().filter(|row| {
        matches!(
            row.classification(),
            WorthGraphReadAccessClassification::QueryDeclarationCandidate
                | WorthGraphReadAccessClassification::QueryAccessCapabilityGap
                | WorthGraphReadAccessClassification::CappedResidue
                | WorthGraphReadAccessClassification::DeletionTarget
        )
    })
}

fn row_for_path<'a>(
    inventory: &'a WorthGraphReadAccessInventoryCloseout,
    source_path: &str,
) -> &'a WorthGraphReadAccessInventoryRow {
    inventory
        .rows()
        .iter()
        .find(|row| row.source_path() == source_path)
        .expect("test inventory should contain requested source path")
}

fn assert_gap_is_persistent_continuation_index(
    closeout: &WorthGraphReadAccessPhaseSixCloseout,
    row: &WorthGraphReadAccessInventoryRow,
) {
    let gap = closeout
        .capability_gaps()
        .iter()
        .find(|gap| context_matches_row(gap.inventory_row_context(), row))
        .expect("row should lower into a Query capability gap");
    assert!(matches!(
        row.cost_posture(),
        WorthGraphReadAccessCostPosture::BroadScan
            | WorthGraphReadAccessCostPosture::FrontierOrVisitedSet
    ));
    assert_eq!(
        gap.missing_capability(),
        WorthGraphReadMissingQueryCapability::PersistentContinuationIndex
    );
    assert_eq!(gap.expected_denial(), &expected_persistent_index_denial());
}

fn expected_persistent_index_denial() -> WorthGraphReadExpectedDenial {
    WorthGraphReadExpectedDenial::new(
        ForgeQueryGraphReadAccessDenialKind::RequiredPersistentIndex,
        ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
    )
}

fn closeout_context_match_count(
    closeout: &WorthGraphReadAccessPhaseSixCloseout,
    row: &WorthGraphReadAccessInventoryRow,
) -> usize {
    closeout
        .declaration_candidates()
        .iter()
        .filter(|candidate| context_matches_row(candidate.inventory_row_context(), row))
        .count()
        + closeout
            .capability_gaps()
            .iter()
            .filter(|gap| context_matches_row(gap.inventory_row_context(), row))
            .count()
        + closeout
            .deletion_items()
            .iter()
            .filter(|item| context_matches_row(item.inventory_row_context(), row))
            .count()
}

fn context_matches_row(
    context: &super::super::WorthGraphReadAccessInventoryRowContext,
    row: &WorthGraphReadAccessInventoryRow,
) -> bool {
    context.identity().source_path() == row.source_path()
        && context.identity().owner() == row.owner()
        && context.identity().current_caller() == row.current_caller()
        && context.classification() == row.classification()
        && context.cost_posture() == row.cost_posture()
        && context.deletion_action() == row.deletion_action()
        && context.milestone_seven_disposition() == row.milestone_seven_disposition()
        && context.scope_binding() == row.scope_binding()
}
