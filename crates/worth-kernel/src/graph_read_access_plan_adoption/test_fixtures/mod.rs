use crate::graph_read_access_declarations::{
    current_worth_graph_read_access_admission_posture_closeout,
    current_worth_graph_read_access_declaration_catalog_closeout,
    current_worth_graph_read_access_declaration_closeout,
    current_worth_graph_read_declaration_deletion_firewall_closeout,
    current_worth_graph_read_requirement_derivation_closeout,
    phase_one_closeout_from_milestone_seven_seed_for_tests,
    WorthGraphReadAccessDeclarationMilestoneEightSeed,
};
use crate::graph_read_access_inventory::{
    current_worth_graph_read_access_milestone_six_closeout_for_tests,
    WorthGraphReadAccessMilestoneSevenSeed,
};

pub(crate) fn production_milestone_eight_seed() -> WorthGraphReadAccessDeclarationMilestoneEightSeed
{
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    milestone_eight_seed_from_milestone_seven_seed_for_tests(milestone_six.milestone_seven_seed())
}

pub(crate) fn milestone_eight_seed_from_milestone_seven_seed_for_tests(
    milestone_seven_seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> WorthGraphReadAccessDeclarationMilestoneEightSeed {
    let phase_one = phase_one_closeout_from_milestone_seven_seed_for_tests(milestone_seven_seed)
        .expect("Milestone 7 seed should admit");
    let phase_two = current_worth_graph_read_access_declaration_catalog_closeout(&phase_one)
        .expect("Phase 2 catalog should build");
    let phase_four = current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 4 requirements should derive");
    let phase_five =
        current_worth_graph_read_access_admission_posture_closeout(phase_four.phase_five_seed())
            .expect("Phase 5 posture should build");
    let phase_six = current_worth_graph_read_declaration_deletion_firewall_closeout(
        phase_five.phase_six_seed(),
    )
    .expect("Phase 6 deletion firewall should build");
    current_worth_graph_read_access_declaration_closeout(phase_six.phase_seven_seed())
        .expect("Milestone 7 closeout should build")
        .milestone_eight_seed()
        .clone()
}
