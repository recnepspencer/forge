#[derive(Debug, Clone, Copy)]
pub(in crate::harness::adapter::adapter_impl) struct WritebackCounterSnapshot {
    pub(in crate::harness::adapter::adapter_impl) writeback_family_lookup_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_family_dispatch_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_mapper_lowering_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_decision_record_append_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_request_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_effect_width: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_strategy_contract_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_strategy_rejection_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_idempotence_check_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_causality_match_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_loop_prevention_check_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_loop_prevention_rejection_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_noop_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_commit_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_failure_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_authority_denial_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_validation_rejection_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_replay_request_count: usize,
    pub(in crate::harness::adapter::adapter_impl) writeback_replay_mismatch_count: usize,
}

impl WritebackCounterSnapshot {
    pub(in crate::harness::adapter::adapter_impl) fn counters(
        self,
    ) -> crate::facade::BridgeWritebackCounters {
        crate::facade::BridgeWritebackCounters::new(
            self.writeback_family_lookup_count,
            self.writeback_family_dispatch_count,
            self.writeback_mapper_lowering_count,
            self.writeback_decision_record_append_count,
            self.writeback_request_count,
            self.writeback_effect_width,
            self.writeback_strategy_contract_count,
            self.writeback_strategy_rejection_count,
            self.writeback_idempotence_check_count,
            self.writeback_causality_match_count,
            self.writeback_loop_prevention_check_count,
            self.writeback_loop_prevention_rejection_count,
            self.writeback_noop_count,
            self.writeback_commit_count,
            self.writeback_failure_count,
            self.writeback_authority_denial_count,
            self.writeback_validation_rejection_count,
            self.writeback_replay_request_count,
            self.writeback_replay_mismatch_count,
        )
    }
}

pub(in crate::harness::adapter::adapter_impl::writeback) fn snapshot_from_counters(
    counters: &crate::facade::BridgeWritebackCounters,
) -> WritebackCounterSnapshot {
    WritebackCounterSnapshot {
        writeback_family_lookup_count: counters.writeback_family_lookup_count(),
        writeback_family_dispatch_count: counters.writeback_family_dispatch_count(),
        writeback_mapper_lowering_count: counters.writeback_mapper_lowering_count(),
        writeback_decision_record_append_count: counters.writeback_decision_record_append_count(),
        writeback_request_count: counters.writeback_request_count(),
        writeback_effect_width: counters.writeback_effect_width(),
        writeback_strategy_contract_count: counters.writeback_strategy_contract_count(),
        writeback_strategy_rejection_count: counters.writeback_strategy_rejection_count(),
        writeback_idempotence_check_count: counters.writeback_idempotence_check_count(),
        writeback_causality_match_count: counters.writeback_causality_match_count(),
        writeback_loop_prevention_check_count: counters.writeback_loop_prevention_check_count(),
        writeback_loop_prevention_rejection_count: counters
            .writeback_loop_prevention_rejection_count(),
        writeback_noop_count: counters.writeback_noop_count(),
        writeback_commit_count: counters.writeback_commit_count(),
        writeback_failure_count: counters.writeback_failure_count(),
        writeback_authority_denial_count: counters.writeback_authority_denial_count(),
        writeback_validation_rejection_count: counters.writeback_validation_rejection_count(),
        writeback_replay_request_count: counters.writeback_replay_request_count(),
        writeback_replay_mismatch_count: counters.writeback_replay_mismatch_count(),
    }
}

pub(in crate::harness::adapter::adapter_impl::writeback) fn aggregate_runtime_writeback_counters(
    runtimes: &[&crate::facade::RuntimeBridge],
) -> crate::facade::BridgeWritebackCounters {
    let mut totals = WritebackCounterSnapshot {
        writeback_family_lookup_count: 0,
        writeback_family_dispatch_count: 0,
        writeback_mapper_lowering_count: 0,
        writeback_decision_record_append_count: 0,
        writeback_request_count: 0,
        writeback_effect_width: 0,
        writeback_strategy_contract_count: 0,
        writeback_strategy_rejection_count: 0,
        writeback_idempotence_check_count: 0,
        writeback_causality_match_count: 0,
        writeback_loop_prevention_check_count: 0,
        writeback_loop_prevention_rejection_count: 0,
        writeback_noop_count: 0,
        writeback_commit_count: 0,
        writeback_failure_count: 0,
        writeback_authority_denial_count: 0,
        writeback_validation_rejection_count: 0,
        writeback_replay_request_count: 0,
        writeback_replay_mismatch_count: 0,
    };

    for runtime_bridge in runtimes {
        add_execution_record_counters(&mut totals, runtime_bridge);
        add_replay_record_counters(&mut totals, runtime_bridge);
    }

    totals.counters()
}

fn add_execution_record_counters(
    totals: &mut WritebackCounterSnapshot,
    runtime_bridge: &crate::facade::RuntimeBridge,
) {
    for record in runtime_bridge.diagnostics().writeback_execution_records() {
        add_counters(totals, record.counters());
    }
}

fn add_replay_record_counters(
    totals: &mut WritebackCounterSnapshot,
    runtime_bridge: &crate::facade::RuntimeBridge,
) {
    for record in runtime_bridge.diagnostics().writeback_replay_records() {
        add_counters(totals, record.counters());
    }
}

fn add_counters(
    totals: &mut WritebackCounterSnapshot,
    counters: &crate::facade::BridgeWritebackCounters,
) {
    totals.writeback_family_lookup_count += counters.writeback_family_lookup_count();
    totals.writeback_family_dispatch_count += counters.writeback_family_dispatch_count();
    totals.writeback_mapper_lowering_count += counters.writeback_mapper_lowering_count();
    totals.writeback_decision_record_append_count +=
        counters.writeback_decision_record_append_count();
    totals.writeback_request_count += counters.writeback_request_count();
    totals.writeback_effect_width += counters.writeback_effect_width();
    totals.writeback_strategy_contract_count += counters.writeback_strategy_contract_count();
    totals.writeback_strategy_rejection_count += counters.writeback_strategy_rejection_count();
    totals.writeback_idempotence_check_count += counters.writeback_idempotence_check_count();
    totals.writeback_causality_match_count += counters.writeback_causality_match_count();
    totals.writeback_loop_prevention_check_count +=
        counters.writeback_loop_prevention_check_count();
    totals.writeback_loop_prevention_rejection_count +=
        counters.writeback_loop_prevention_rejection_count();
    totals.writeback_noop_count += counters.writeback_noop_count();
    totals.writeback_commit_count += counters.writeback_commit_count();
    totals.writeback_failure_count += counters.writeback_failure_count();
    totals.writeback_authority_denial_count += counters.writeback_authority_denial_count();
    totals.writeback_validation_rejection_count += counters.writeback_validation_rejection_count();
    totals.writeback_replay_request_count += counters.writeback_replay_request_count();
    totals.writeback_replay_mismatch_count += counters.writeback_replay_mismatch_count();
}
