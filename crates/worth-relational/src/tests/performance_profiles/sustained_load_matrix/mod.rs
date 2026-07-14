use super::*;

mod chip_global_step_endurance;
mod commit_query_churn;
mod mixed_topology_query_churn;
mod replay_window_drift;
mod retention_pass_drift;
mod rocketship_hot_update_endurance;
mod rocketship_propagation_endurance;

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_sustained_load_matrix() {
    let suite = "sustained_load_matrix";

    commit_query_churn::certify_commit_query_churn_stability(suite);
    replay_window_drift::certify_replay_window_drift_stability(suite);
    retention_pass_drift::certify_retention_pass_drift_stability(suite);
    mixed_topology_query_churn::certify_mixed_topology_query_churn_stability(suite);
    rocketship_hot_update_endurance::certify_rocketship_hot_update_endurance(suite);
    rocketship_propagation_endurance::certify_rocketship_propagation_endurance(suite);
    chip_global_step_endurance::certify_chip_global_step_endurance(suite);
}
