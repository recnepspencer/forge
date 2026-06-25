use super::super::{
    WorthGraphReadAccessMilestoneSixCloseout, WorthGraphReadAccessMilestoneSixReadiness,
};
use super::current_inventory_closeout;

#[test]
fn milestone_six_closeout_requires_exact_inventory_and_disposition_counts() {
    let closeout = WorthGraphReadAccessMilestoneSixCloseout::from_inventory_closeout(
        current_inventory_closeout(),
    )
    .expect("current inventory should produce final Milestone 6 closeout");
    let counters = closeout.counters();

    assert_eq!(
        closeout.readiness(),
        WorthGraphReadAccessMilestoneSixReadiness::ReadyForMilestoneSeven
    );
    assert_eq!(counters.inventory_row_count(), 12);
    assert_eq!(counters.declaration_candidate_count(), 5);
    assert_eq!(counters.capability_gap_count(), 2);
    assert_eq!(counters.deletion_target_count(), 1);
    assert_eq!(counters.deletion_item_count(), 1);
    assert_eq!(counters.capped_residue_count(), 0);
    assert_eq!(counters.certification_only_count(), 4);
    assert_eq!(counters.out_of_scope_count(), 0);
    assert_eq!(counters.deleted_source_count(), 1);
    assert_eq!(counters.existing_deleted_source_count(), 0);

    assert_eq!(
        closeout.inventory_closeout().counters().total_row_count(),
        counters.inventory_row_count()
    );
    assert_eq!(
        closeout
            .milestone_seven_seed()
            .declaration_candidates()
            .len(),
        counters.declaration_candidate_count()
    );
}
