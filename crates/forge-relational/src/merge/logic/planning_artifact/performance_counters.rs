use crate::logic::runtime::RelationalRuntime;
use crate::merge::data::LoweredMergePlan;

pub(super) fn record_merge_planning_counters(
    runtime: &RelationalRuntime,
    plan: &LoweredMergePlan,
    touched_schema_kind_count: usize,
) {
    runtime.performance_access().count_merge_planning_request(
        touched_schema_kind_count,
        plan.ancestry.target.unique_commit_count,
        plan.ancestry.source.unique_commit_count,
        plan.ancestry.target.touched_record_count,
        plan.ancestry.source.touched_record_count,
    );
    runtime.performance_access().count_merge_identity_discovery(
        plan.identity_summary.candidate_count,
        plan.identity_summary.effective_declarations.len(),
    );
    runtime
        .performance_access()
        .count_merge_conflict_classification(plan.conflict_summary.classified_record_count);
    runtime
        .performance_access()
        .count_merge_causal_annotation(plan.causal_summary.classified_record_count);
    runtime
        .performance_access()
        .count_merge_policy_resolution(plan.policy_summary.resolved_record_count);
    runtime.performance_access().count_merge_lowering(
        plan.lowered_summary.record_count,
        plan.decision_log.decisions.len(),
    );
}
