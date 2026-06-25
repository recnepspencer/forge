use super::super::WorthGraphReadAccessMilestoneSixCloseout;
use super::current_inventory_closeout;

#[test]
fn milestone_seven_seed_contains_no_old_graph_read_folklore() {
    let closeout = WorthGraphReadAccessMilestoneSixCloseout::from_inventory_closeout(
        current_inventory_closeout(),
    )
    .expect("current inventory should produce final Milestone 6 closeout");
    let seed = closeout.milestone_seven_seed();

    assert_eq!(seed.declaration_candidates().len(), 5);
    assert_eq!(seed.capability_gaps().len(), 2);
    assert_eq!(seed.deletion_items().len(), 1);
    assert!(!seed.claims_execution_authority());
    assert!(!seed.contains_uncapped_old_graph_read_folklore_as_declaration_or_gap());

    assert!(seed
        .declaration_candidates()
        .iter()
        .all(|candidate| candidate.inventory_row_identity().source_path() != OLD_GRAPH_READ_PATH));
    assert!(seed
        .capability_gaps()
        .iter()
        .all(|gap| gap.inventory_row_identity().source_path() != OLD_GRAPH_READ_PATH));
    assert!(seed
        .deletion_items()
        .iter()
        .any(|item| item.inventory_row_identity().source_path() == OLD_GRAPH_READ_PATH));
}

const OLD_GRAPH_READ_PATH: &str = "crates/worth-kernel/src/query_adoption/graph_read_access";
