use crate::graph_read_access_declarations::{
    current_worth_graph_read_access_admission_posture_closeout,
    current_worth_graph_read_access_declaration_catalog_closeout,
    current_worth_graph_read_requirement_derivation_closeout,
    phase_one_closeout_from_milestone_seven_seed_for_tests,
    WorthGraphReadAccessAdmissionPostureCloseout, WorthGraphReadRequirementDerivationCloseout,
};
use crate::graph_read_access_inventory::{
    current_worth_graph_read_access_milestone_six_closeout_for_tests,
    same_family_multiple_callers_milestone_seven_seed_for_tests,
    same_family_multiple_callers_reversed_milestone_seven_seed_for_tests,
    WorthGraphReadAccessMilestoneSevenSeed,
};

pub(crate) fn production_requirement_derivation_closeout(
) -> WorthGraphReadRequirementDerivationCloseout {
    requirement_derivation_closeout_from_seed(&production_seed())
}

pub(crate) fn production_admission_posture_closeout() -> WorthGraphReadAccessAdmissionPostureCloseout
{
    let phase_four = production_requirement_derivation_closeout();
    current_worth_graph_read_access_admission_posture_closeout(phase_four.phase_five_seed())
        .expect("Phase 5 should classify production Phase 4 seed")
}

pub(crate) fn reversed_admission_posture_closeout_pair() -> (
    WorthGraphReadAccessAdmissionPostureCloseout,
    WorthGraphReadAccessAdmissionPostureCloseout,
) {
    let forward = requirement_derivation_closeout_from_seed(
        &same_family_multiple_callers_milestone_seven_seed_for_tests(),
    );
    let reversed = requirement_derivation_closeout_from_seed(
        &same_family_multiple_callers_reversed_milestone_seven_seed_for_tests(),
    );
    (
        current_worth_graph_read_access_admission_posture_closeout(forward.phase_five_seed())
            .expect("forward Phase 5 closeout should build"),
        current_worth_graph_read_access_admission_posture_closeout(reversed.phase_five_seed())
            .expect("reversed Phase 5 closeout should build"),
    )
}

fn production_seed() -> WorthGraphReadAccessMilestoneSevenSeed {
    current_worth_graph_read_access_milestone_six_closeout_for_tests()
        .milestone_seven_seed()
        .clone()
}

fn requirement_derivation_closeout_from_seed(
    seed: &WorthGraphReadAccessMilestoneSevenSeed,
) -> WorthGraphReadRequirementDerivationCloseout {
    let phase_one = phase_one_closeout_from_milestone_seven_seed_for_tests(seed)
        .expect("Milestone 7 seed should admit into Phase 1");
    let phase_two = current_worth_graph_read_access_declaration_catalog_closeout(&phase_one)
        .expect("Phase 1 should build a catalog");
    current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 2 should build Phase 4 requirement records")
}
