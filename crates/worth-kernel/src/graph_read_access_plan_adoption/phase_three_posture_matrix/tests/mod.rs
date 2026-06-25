mod cap_growth;
mod denial_receipt_boundary;
mod exact_posture_coverage;
mod phase_four_seed;
mod pre_execution_boundary;

use crate::graph_read_access_plan_adoption::phase_one_closeout::current_worth_graph_read_access_plan_adoption_phase_one_closeout;
use crate::graph_read_access_plan_adoption::phase_two_adoption::current_worth_graph_read_access_plan_adoption_phase_two_closeout;
use crate::graph_read_access_plan_adoption::test_fixtures::production_milestone_eight_seed;
use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessPlanAdoptionAttempt, WorthGraphReadAccessPlanAdoptionAttemptKind,
    WorthGraphReadAccessPlanAdoptionCarriedGapRow,
};

use super::closeout::{
    current_worth_graph_read_access_posture_matrix_closeout,
    WorthGraphReadAccessPostureMatrixCloseout,
};

fn production_phase_three_closeout() -> WorthGraphReadAccessPostureMatrixCloseout {
    let seed = production_milestone_eight_seed();
    let phase_one = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Phase 1 should close from production seed");
    let phase_two = current_worth_graph_read_access_plan_adoption_phase_two_closeout(&phase_one)
        .expect("Phase 2 should close from Phase 1");
    current_worth_graph_read_access_posture_matrix_closeout(&phase_two)
        .expect("Phase 3 should close from Phase 2")
}

fn production_phase_two_closeout(
) -> crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout {
    let seed = production_milestone_eight_seed();
    let phase_one = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Phase 1 should close from production seed");
    current_worth_graph_read_access_plan_adoption_phase_two_closeout(&phase_one)
        .expect("Phase 2 should close from Phase 1")
}

fn phase_two_closeout_with_attempts_for_tests(
    attempts: Vec<WorthGraphReadAccessPlanAdoptionAttempt>,
    carried_gaps: Vec<WorthGraphReadAccessPlanAdoptionCarriedGapRow>,
) -> crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout {
    crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout::for_posture_matrix_tests(
        attempts,
        carried_gaps,
    )
}

fn required_or_denied_attempt_for_tests(
    requirement_identity: &str,
    query_posture: &str,
    denial_kind: &str,
) -> WorthGraphReadAccessPlanAdoptionAttempt {
    WorthGraphReadAccessPlanAdoptionAttempt::for_posture_matrix_test(
        WorthGraphReadAccessPlanAdoptionAttemptKind::RequiredOrDeniedPosture,
        requirement_identity,
        query_posture,
        Some(denial_kind),
        Some("Query requires support before Worth may execute this graph read."),
        Some("Remove once the Query support posture is consumed by Phase 4."),
    )
}

fn carried_gap_for_tests(
    requirement_identity: &str,
) -> WorthGraphReadAccessPlanAdoptionCarriedGapRow {
    WorthGraphReadAccessPlanAdoptionCarriedGapRow::for_posture_matrix_test(requirement_identity)
}
