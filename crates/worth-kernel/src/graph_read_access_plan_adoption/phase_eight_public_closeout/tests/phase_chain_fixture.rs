use crate::graph_read_access_plan_adoption::test_fixtures::production_milestone_eight_seed;
use crate::graph_read_access_plan_adoption::{
    current_worth_graph_read_access_execution_receipt_accounting_closeout,
    current_worth_graph_read_access_first_vertical_slice_closeout,
    current_worth_graph_read_access_hard_deletion_closeout,
    current_worth_graph_read_access_plan_adoption_closeout,
    current_worth_graph_read_access_plan_adoption_phase_one_closeout,
    current_worth_graph_read_access_plan_adoption_phase_two_closeout,
    current_worth_graph_read_access_posture_matrix_closeout,
    current_worth_graph_read_access_spatial_dense_posture_closeout,
    WorthGraphReadAccessHardDeletionPhaseEightSeed, WorthGraphReadAccessPlanAdoptionCloseout,
};

pub(crate) fn production_phase_eight_seed() -> WorthGraphReadAccessHardDeletionPhaseEightSeed {
    let seed = production_milestone_eight_seed();
    let phase_one = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Phase 1 should close from production seed");
    let phase_two = current_worth_graph_read_access_plan_adoption_phase_two_closeout(&phase_one)
        .expect("Phase 2 should close from Phase 1");
    let phase_three = current_worth_graph_read_access_posture_matrix_closeout(&phase_two)
        .expect("Phase 3 should close from Phase 2");
    let phase_four = current_worth_graph_read_access_first_vertical_slice_closeout(
        phase_three.phase_four_seed(),
    )
    .expect("Phase 4 should close from Phase 3");
    let phase_five = current_worth_graph_read_access_spatial_dense_posture_closeout(
        phase_four.phase_five_seed(),
    )
    .expect("Phase 5 should close from Phase 4");
    let phase_six = current_worth_graph_read_access_execution_receipt_accounting_closeout(
        phase_five.phase_six_seed(),
    )
    .expect("Phase 6 should close from Phase 5");
    current_worth_graph_read_access_hard_deletion_closeout(phase_six.phase_seven_seed())
        .expect("Phase 7 should close from Phase 6")
        .phase_eight_seed()
        .clone()
}

pub(crate) fn production_phase_eight_closeout() -> WorthGraphReadAccessPlanAdoptionCloseout {
    current_worth_graph_read_access_plan_adoption_closeout(&production_phase_eight_seed())
        .expect("Phase 8 should close from Phase 7")
}
