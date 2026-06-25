mod batch_accounting;
mod caller_owned_work;
mod counter_boundary;
mod deterministic_identity;
mod phase_seven_seed;
mod receipt_identity;

use forge_query::facade::ForgeQueryWorkspace;
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::query_access_planning::{
    execute_topology_birth_query_access_for_request, PrimitiveConstructionConsumedQueryAccess,
};
use crate::construction::specs::RegularPrismSpec;
use crate::graph_read_access_plan_adoption::phase_four_vertical_slice::current_worth_graph_read_access_first_vertical_slice_closeout_with_construction_execution;
use crate::graph_read_access_plan_adoption::test_fixtures::production_milestone_eight_seed;
use crate::graph_read_access_plan_adoption::{
    current_worth_graph_read_access_execution_receipt_accounting_closeout,
    current_worth_graph_read_access_plan_adoption_phase_one_closeout,
    current_worth_graph_read_access_plan_adoption_phase_two_closeout,
    current_worth_graph_read_access_posture_matrix_closeout,
    current_worth_graph_read_access_spatial_dense_posture_closeout,
    WorthGraphReadAccessExecutionReceiptAccountingCloseout,
    WorthGraphReadAccessSpatialDensePhaseSixSeed,
};

fn production_phase_six_seed() -> WorthGraphReadAccessSpatialDensePhaseSixSeed {
    let seed = production_milestone_eight_seed();
    let phase_one = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Phase 1 should close from production seed");
    let phase_two = current_worth_graph_read_access_plan_adoption_phase_two_closeout(&phase_one)
        .expect("Phase 2 should close from Phase 1");
    let phase_three = current_worth_graph_read_access_posture_matrix_closeout(&phase_two)
        .expect("Phase 3 should close from Phase 2");
    let consumed_access = executed_construction_query_access();
    let phase_four =
        current_worth_graph_read_access_first_vertical_slice_closeout_with_construction_execution(
            phase_three.phase_four_seed(),
            &consumed_access,
        )
        .expect("Phase 4 should close from Phase 3 through a real construction Query receipt");
    current_worth_graph_read_access_spatial_dense_posture_closeout(phase_four.phase_five_seed())
        .expect("Phase 5 should close from Phase 4")
        .phase_six_seed()
        .clone()
}

fn production_phase_six_closeout() -> WorthGraphReadAccessExecutionReceiptAccountingCloseout {
    current_worth_graph_read_access_execution_receipt_accounting_closeout(
        &production_phase_six_seed(),
    )
    .expect("Phase 6 should close from Phase 5 seed")
}

fn executed_construction_query_access() -> PrimitiveConstructionConsumedQueryAccess {
    let mut workspace = primitive_topology_workspace("worth-kernel.phase-six.receipt-backed");
    let request = PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
        sides: 6,
        radius: 1.0,
        height: 2.0,
    })
    .into_request();
    execute_topology_birth_query_access_for_request(&mut workspace, &request)
        .expect("planned topology birth read should execute through Query")
}

fn primitive_topology_workspace(name: &str) -> ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("topology runtime builder should build")
        .build();
    topology_runtime(TopologyRuntimeAdapters::current_head(runtime), name)
        .expect("topology workspace should open")
}
