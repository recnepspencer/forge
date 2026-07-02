use topology::derived_invalidation_milestone_ten_closeout::DerivedInvalidationMilestoneTenProductSummaryRow;
use topology::derived_invalidation_route_input::admit_topology_invalidation_route_input;
use topology::derived_invalidation_selected_plan::DerivedInvalidationSelectedPlan;

fn main() {
    let _ = admit_topology_invalidation_route_input(
        fake_product_summary_row(),
        fake_selected_plan(),
    );
}

fn fake_product_summary_row() -> &'static DerivedInvalidationMilestoneTenProductSummaryRow {
    panic!("compile-fail fixture does not execute")
}

fn fake_selected_plan() -> &'static DerivedInvalidationSelectedPlan {
    panic!("compile-fail fixture does not execute")
}
