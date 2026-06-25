mod broad_boolean_predicates;
mod dense_frontier_boundaries;
mod grouped_admission;
mod phase_six_seed;
mod source_firewall;
mod spatial_required_postures;

use crate::graph_read_access_inventory::{
    topology_spatial_and_broad_boolean_milestone_seven_seed_for_tests,
    WorthGraphReadAccessMilestoneSevenSeed,
};
use crate::graph_read_access_plan_adoption::test_fixtures::{
    milestone_eight_seed_from_milestone_seven_seed_for_tests, production_milestone_eight_seed,
};
use crate::graph_read_access_plan_adoption::{
    current_worth_graph_read_access_first_vertical_slice_closeout,
    current_worth_graph_read_access_plan_adoption_phase_one_closeout,
    current_worth_graph_read_access_plan_adoption_phase_two_closeout,
    current_worth_graph_read_access_posture_matrix_closeout,
    WorthGraphReadAccessFirstVerticalSliceSeed,
};

use super::closeout::{
    current_worth_graph_read_access_spatial_dense_posture_closeout,
    WorthGraphReadAccessSpatialDensePostureCloseout,
};

fn production_phase_five_seed() -> WorthGraphReadAccessFirstVerticalSliceSeed {
    let seed = production_milestone_eight_seed();
    phase_five_seed_from_milestone_eight_seed(seed)
}

fn broad_boolean_phase_five_closeout() -> WorthGraphReadAccessSpatialDensePostureCloseout {
    phase_five_closeout_from_milestone_seven_seed(
        &topology_spatial_and_broad_boolean_milestone_seven_seed_for_tests(),
    )
}

fn phase_five_closeout_from_milestone_seven_seed(
    milestone_seven_seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> WorthGraphReadAccessSpatialDensePostureCloseout {
    let seed = milestone_eight_seed_from_milestone_seven_seed_for_tests(milestone_seven_seed);
    current_worth_graph_read_access_spatial_dense_posture_closeout(
        &phase_five_seed_from_milestone_eight_seed(seed),
    )
    .expect("Phase 5 should close from broad family seed")
}

fn phase_five_seed_from_milestone_eight_seed(
    seed: crate::graph_read_access_declarations::WorthGraphReadAccessDeclarationMilestoneEightSeed,
) -> WorthGraphReadAccessFirstVerticalSliceSeed {
    let phase_one = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Phase 1 should close from production seed");
    let phase_two = current_worth_graph_read_access_plan_adoption_phase_two_closeout(&phase_one)
        .expect("Phase 2 should close from Phase 1");
    let phase_three = current_worth_graph_read_access_posture_matrix_closeout(&phase_two)
        .expect("Phase 3 should close from Phase 2");
    current_worth_graph_read_access_first_vertical_slice_closeout(phase_three.phase_four_seed())
        .expect("Phase 4 should close from Phase 3")
        .phase_five_seed()
        .clone()
}

fn production_phase_five_closeout() -> WorthGraphReadAccessSpatialDensePostureCloseout {
    current_worth_graph_read_access_spatial_dense_posture_closeout(&production_phase_five_seed())
        .expect("Phase 5 should close from Phase 4 seed")
}
