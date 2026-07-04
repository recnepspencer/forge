use crate::replay_undo_semantic_graph::current_topology_invalidation_declared_touch_proof;
use crate::validator_invariant_catalog::selection_from_touched_closure::current_topology_validator_invariant_selection_closeout_for_declared_touch;

#[test]
fn current_validator_selection_closeout_for_declared_touch_binds_same_touch_basis() {
    let proof = current_topology_invalidation_declared_touch_proof().expect("declared touch proof");
    let closeout =
        current_topology_validator_invariant_selection_closeout_for_declared_touch(&proof)
            .expect("current validator selection closeout");

    assert_eq!(
        closeout.selected_plan().routing_closure_digest(),
        closeout.phase_four_seed().routing_closure_digest()
    );
    assert!(!closeout.selected_plan().selected_plan_digest().is_empty());
}
