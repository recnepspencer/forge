use super::*;
mod branch_divergence;
mod feature_adoption_execution;
mod persisted_commit_floor;
mod phase_timing;
mod planning_divergent_update;
mod prepare_execute_split;
mod verify_execute_split;
mod zero_diagnostics_execution;

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_merge_lineage_matrix() {
    let suite = "merge_lineage_matrix";

    planning_divergent_update::certify_merge_planning_divergent_update(suite);
    feature_adoption_execution::certify_merge_execution_feature_adoption(suite);
    zero_diagnostics_execution::certify_merge_execution_zero_diagnostics_budget(suite);
    prepare_execute_split::certify_merge_prepare_vs_execute_feature_adoption(suite);
    persisted_commit_floor::certify_merge_execution_vs_persisted_commit_floor(suite);
    verify_execute_split::certify_merge_verify_vs_execute_feature_adoption(suite);
    phase_timing::certify_merge_execute_phase_timing_feature_adoption(suite);
    branch_divergence::certify_lineage_branch_divergence_breadth(suite);
}
