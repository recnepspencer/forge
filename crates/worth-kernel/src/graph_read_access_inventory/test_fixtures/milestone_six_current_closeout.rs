use super::super::inventory_lane;
use super::super::WorthGraphReadAccessMilestoneSixCloseout;

pub(crate) fn current_worth_graph_read_access_milestone_six_closeout_for_tests(
) -> WorthGraphReadAccessMilestoneSixCloseout {
    let inventory = inventory_lane::current_worth_graph_read_access_surface_inventory_for_tests(
        inventory_lane::WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .expect("test graph-read inventory should close");
    WorthGraphReadAccessMilestoneSixCloseout::from_inventory_closeout(inventory)
        .expect("test graph-read inventory should close Milestone 6")
}
