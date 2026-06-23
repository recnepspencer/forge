mod exact_counts;
mod later_milestone_claims;
mod milestone_seven_seed;
mod seed_payload_contracts;

use super::super::current_worth_graph_read_access_surface_inventory_for_tests;
use super::super::inventory_lane::{
    WorthGraphReadAccessInventoryCloseout, WorthGraphReadAccessInventorySeed,
};

fn current_inventory_closeout() -> WorthGraphReadAccessInventoryCloseout {
    current_worth_graph_read_access_surface_inventory_for_tests(
        WorthGraphReadAccessInventorySeed::for_tests(),
    )
    .expect("current graph-read access inventory should close for Milestone 6 closeout tests")
}
