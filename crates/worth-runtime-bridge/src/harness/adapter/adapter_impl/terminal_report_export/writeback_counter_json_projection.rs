use serde_json::json;

pub(in crate::harness::adapter::adapter_impl) fn writeback_counter_snapshot_json(
    counters: &crate::facade::BridgeWritebackCounters,
) -> serde_json::Value {
    json!({
        "writeback_family_lookup_count": counters.writeback_family_lookup_count(),
        "writeback_family_dispatch_count": counters.writeback_family_dispatch_count(),
        "writeback_mapper_lowering_count": counters.writeback_mapper_lowering_count(),
        "writeback_decision_record_append_count": counters.writeback_decision_record_append_count(),
        "writeback_request_count": counters.writeback_request_count(),
        "writeback_effect_width": counters.writeback_effect_width(),
        "writeback_strategy_contract_count": counters.writeback_strategy_contract_count(),
        "writeback_strategy_rejection_count": counters.writeback_strategy_rejection_count(),
        "writeback_idempotence_check_count": counters.writeback_idempotence_check_count(),
        "writeback_causality_match_count": counters.writeback_causality_match_count(),
        "writeback_loop_prevention_check_count": counters.writeback_loop_prevention_check_count(),
        "writeback_loop_prevention_rejection_count": counters.writeback_loop_prevention_rejection_count(),
        "writeback_noop_count": counters.writeback_noop_count(),
        "writeback_commit_count": counters.writeback_commit_count(),
        "writeback_failure_count": counters.writeback_failure_count(),
        "writeback_authority_denial_count": counters.writeback_authority_denial_count(),
        "writeback_validation_rejection_count": counters.writeback_validation_rejection_count(),
        "writeback_replay_request_count": counters.writeback_replay_request_count(),
        "writeback_replay_mismatch_count": counters.writeback_replay_mismatch_count(),
    })
}
