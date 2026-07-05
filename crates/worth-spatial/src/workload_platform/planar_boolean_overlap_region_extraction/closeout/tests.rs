use super::summum_bonum::PlanarBooleanOverlapRegionSummumBonumCloseoutCounters;

#[test]
fn summum_bonum_counters_start_with_no_pairwise_rediscovery() {
    let counters = PlanarBooleanOverlapRegionSummumBonumCloseoutCounters::default();
    assert_eq!(counters.pairwise_rediscovery_attempts(), 0);
}
