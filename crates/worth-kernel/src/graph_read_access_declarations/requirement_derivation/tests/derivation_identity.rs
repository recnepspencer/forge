use crate::graph_read_access_declarations::current_worth_graph_read_requirement_derivation_closeout;
use crate::graph_read_access_inventory::{
    same_family_multiple_callers_milestone_seven_seed_for_tests,
    same_family_multiple_callers_reversed_milestone_seven_seed_for_tests,
};

use super::common::phase_two_closeout_from_seed;

#[test]
fn requirement_derivation_identity_is_stable_under_catalog_ordering() {
    let forward = phase_two_closeout_from_seed(
        &same_family_multiple_callers_milestone_seven_seed_for_tests(),
    );
    let reversed = phase_two_closeout_from_seed(
        &same_family_multiple_callers_reversed_milestone_seven_seed_for_tests(),
    );
    let forward = current_worth_graph_read_requirement_derivation_closeout(&forward)
        .expect("forward catalog should derive Phase 4");
    let reversed = current_worth_graph_read_requirement_derivation_closeout(&reversed)
        .expect("reversed catalog should derive Phase 4");

    assert_eq!(forward.closeout_digest(), reversed.closeout_digest());
    assert_eq!(
        forward.derivation_summary().summary_digest(),
        reversed.derivation_summary().summary_digest()
    );
    assert_eq!(
        forward.requirement_records()[0].record_digest(),
        reversed.requirement_records()[0].record_digest()
    );
}
