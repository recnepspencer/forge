#[test]
fn platform_pulse_cost_slopes_and_capacity_denials_remain_explicit() {
    crate::milestone_312_post_classification_cost::
        prove_rebind_post_classification_cost_is_independent_of_unrelated_width();
    crate::milestone_312_planning_guards::prove_rebind_planning_denies_exhaustion_and_stale_basis();
}
