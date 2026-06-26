use crate::graph_read_access_declarations::{
    current_worth_graph_read_access_admission_posture_closeout,
    current_worth_graph_read_access_declaration_catalog_closeout,
    current_worth_graph_read_requirement_derivation_closeout,
    phase_one_closeout_from_milestone_seven_seed_for_tests,
    WorthGraphReadAccessDeclarationPhaseSixSeed,
};
use crate::graph_read_access_inventory::{
    current_worth_graph_read_access_milestone_six_closeout_for_tests,
    WorthGraphReadAccessMilestoneSevenSeed,
};

pub(crate) fn production_phase_six_seed() -> WorthGraphReadAccessDeclarationPhaseSixSeed {
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    phase_six_seed_from_seed(&milestone_six.milestone_seven_seed())
}

pub(crate) fn phase_six_seed_from_seed(
    seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> WorthGraphReadAccessDeclarationPhaseSixSeed {
    let phase_one = phase_one_closeout_from_milestone_seven_seed_for_tests(seed)
        .expect("Milestone 7 seed should admit");
    let phase_two = current_worth_graph_read_access_declaration_catalog_closeout(&phase_one)
        .expect("Phase 2 catalog should build");
    let phase_four = current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 4 requirement derivation should build");
    current_worth_graph_read_access_admission_posture_closeout(phase_four.phase_five_seed())
        .expect("Phase 5 admission posture should build")
        .phase_six_seed()
        .clone()
}
