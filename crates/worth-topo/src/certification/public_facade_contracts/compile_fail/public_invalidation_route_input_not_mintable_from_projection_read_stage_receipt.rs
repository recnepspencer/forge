use topology::derived_invalidation_operator_cutover::DerivedInvalidationProjectionReadStageReceipt;
use topology::derived_invalidation_route_input::admit_topology_invalidation_route_input;
use topology::derived_invalidation_selected_plan::DerivedInvalidationSelectedPlan;

fn main() {
    let _ = admit_topology_invalidation_route_input(
        fake_projection_read_stage_receipt(),
        fake_selected_plan(),
    );
}

fn fake_projection_read_stage_receipt() -> &'static DerivedInvalidationProjectionReadStageReceipt {
    panic!("compile-fail fixture does not execute")
}

fn fake_selected_plan() -> &'static DerivedInvalidationSelectedPlan {
    panic!("compile-fail fixture does not execute")
}
