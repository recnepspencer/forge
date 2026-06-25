mod cutover_proof;
mod execution_binding;
mod phase_five_seed;
mod query_plan_projection;
mod receipt_boundary;
mod slice_selection;
mod source_firewall;

use forge_query::facade::ForgeQueryWorkspace;
use topology::certification::milestone_one_runtime_builder;
use topology::runtime_support::{topology_runtime, TopologyRuntimeAdapters};

use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::query_access_planning::{
    execute_topology_birth_query_access_for_request, PrimitiveConstructionConsumedQueryAccess,
};
use crate::construction::specs::RegularPrismSpec;
use crate::graph_read_access_plan_adoption::phase_one_closeout::current_worth_graph_read_access_plan_adoption_phase_one_closeout;
use crate::graph_read_access_plan_adoption::phase_three_posture_matrix::current_worth_graph_read_access_posture_matrix_closeout;
use crate::graph_read_access_plan_adoption::phase_two_adoption::current_worth_graph_read_access_plan_adoption_phase_two_closeout;
use crate::graph_read_access_plan_adoption::test_fixtures::production_milestone_eight_seed;
use crate::graph_read_access_plan_adoption::WorthGraphReadAccessPhaseFourSeed;

use super::closeout::{
    current_worth_graph_read_access_first_vertical_slice_closeout,
    current_worth_graph_read_access_first_vertical_slice_closeout_with_construction_execution,
    WorthGraphReadAccessFirstVerticalSliceCloseout,
};

fn production_phase_four_closeout() -> WorthGraphReadAccessFirstVerticalSliceCloseout {
    let seed = production_milestone_eight_seed();
    let phase_one = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Phase 1 should close from production seed");
    let phase_two = current_worth_graph_read_access_plan_adoption_phase_two_closeout(&phase_one)
        .expect("Phase 2 should close from Phase 1");
    let phase_three = current_worth_graph_read_access_posture_matrix_closeout(&phase_two)
        .expect("Phase 3 should close from Phase 2");
    current_worth_graph_read_access_first_vertical_slice_closeout(phase_three.phase_four_seed())
        .expect("Phase 4 should select a first vertical slice")
}

fn production_phase_four_receipt_closeout() -> WorthGraphReadAccessFirstVerticalSliceCloseout {
    let seed = production_phase_four_seed();
    let consumed_access = executed_construction_query_access();
    current_worth_graph_read_access_first_vertical_slice_closeout_with_construction_execution(
        &seed,
        &consumed_access,
    )
    .expect("Phase 4 should consume real construction Query receipt")
}

fn production_phase_four_seed() -> WorthGraphReadAccessPhaseFourSeed {
    let seed = production_milestone_eight_seed();
    let phase_one = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Phase 1 should close from production seed");
    let phase_two = current_worth_graph_read_access_plan_adoption_phase_two_closeout(&phase_one)
        .expect("Phase 2 should close from Phase 1");
    current_worth_graph_read_access_posture_matrix_closeout(&phase_two)
        .expect("Phase 3 should close from Phase 2")
        .phase_four_seed()
        .clone()
}

fn executed_construction_query_access() -> PrimitiveConstructionConsumedQueryAccess {
    let mut workspace = primitive_topology_workspace("worth-kernel.phase-four.receipt-backed");
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
