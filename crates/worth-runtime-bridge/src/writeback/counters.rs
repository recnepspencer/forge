use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackCounters {
    writeback_family_lookup_count: usize,
    writeback_family_dispatch_count: usize,
    writeback_mapper_lowering_count: usize,
    writeback_decision_record_append_count: usize,
    writeback_request_count: usize,
    writeback_effect_width: usize,
    writeback_strategy_contract_count: usize,
    writeback_strategy_rejection_count: usize,
    writeback_idempotence_check_count: usize,
    writeback_causality_match_count: usize,
    writeback_loop_prevention_check_count: usize,
    writeback_loop_prevention_rejection_count: usize,
    writeback_noop_count: usize,
    writeback_commit_count: usize,
    writeback_failure_count: usize,
    writeback_authority_denial_count: usize,
    writeback_validation_rejection_count: usize,
    writeback_replay_request_count: usize,
    writeback_replay_mismatch_count: usize,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackCounters {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        writeback_family_lookup_count: usize,
        writeback_family_dispatch_count: usize,
        writeback_mapper_lowering_count: usize,
        writeback_decision_record_append_count: usize,
        writeback_request_count: usize,
        writeback_effect_width: usize,
        writeback_strategy_contract_count: usize,
        writeback_strategy_rejection_count: usize,
        writeback_idempotence_check_count: usize,
        writeback_causality_match_count: usize,
        writeback_loop_prevention_check_count: usize,
        writeback_loop_prevention_rejection_count: usize,
        writeback_noop_count: usize,
        writeback_commit_count: usize,
        writeback_failure_count: usize,
        writeback_authority_denial_count: usize,
        writeback_validation_rejection_count: usize,
        writeback_replay_request_count: usize,
        writeback_replay_mismatch_count: usize,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-counters|family-lookup-count:{}|family-dispatch-count:{}|mapper-lowering-count:{}|decision-record-append-count:{}|request-count:{}|effect-width:{}|strategy-contract-count:{}|strategy-rejection-count:{}|idempotence-check-count:{}|causality-match-count:{}|loop-prevention-check-count:{}|loop-prevention-rejection-count:{}|noop-count:{}|commit-count:{}|failure-count:{}|authority-denial-count:{}|validation-rejection-count:{}|replay-request-count:{}|replay-mismatch-count:{}",
            writeback_family_lookup_count,
            writeback_family_dispatch_count,
            writeback_mapper_lowering_count,
            writeback_decision_record_append_count,
            writeback_request_count,
            writeback_effect_width,
            writeback_strategy_contract_count,
            writeback_strategy_rejection_count,
            writeback_idempotence_check_count,
            writeback_causality_match_count,
            writeback_loop_prevention_check_count,
            writeback_loop_prevention_rejection_count,
            writeback_noop_count,
            writeback_commit_count,
            writeback_failure_count,
            writeback_authority_denial_count,
            writeback_validation_rejection_count,
            writeback_replay_request_count,
            writeback_replay_mismatch_count,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            writeback_family_lookup_count,
            writeback_family_dispatch_count,
            writeback_mapper_lowering_count,
            writeback_decision_record_append_count,
            writeback_request_count,
            writeback_effect_width,
            writeback_strategy_contract_count,
            writeback_strategy_rejection_count,
            writeback_idempotence_check_count,
            writeback_causality_match_count,
            writeback_loop_prevention_check_count,
            writeback_loop_prevention_rejection_count,
            writeback_noop_count,
            writeback_commit_count,
            writeback_failure_count,
            writeback_authority_denial_count,
            writeback_validation_rejection_count,
            writeback_replay_request_count,
            writeback_replay_mismatch_count,
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-counters:sha256:{digest:x}")),
        }
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn writeback_request_count(&self) -> usize {
        self.writeback_request_count
    }

    pub fn writeback_family_lookup_count(&self) -> usize {
        self.writeback_family_lookup_count
    }

    pub fn writeback_family_dispatch_count(&self) -> usize {
        self.writeback_family_dispatch_count
    }

    pub fn writeback_mapper_lowering_count(&self) -> usize {
        self.writeback_mapper_lowering_count
    }

    pub fn writeback_decision_record_append_count(&self) -> usize {
        self.writeback_decision_record_append_count
    }

    pub fn writeback_effect_width(&self) -> usize {
        self.writeback_effect_width
    }

    pub fn writeback_strategy_contract_count(&self) -> usize {
        self.writeback_strategy_contract_count
    }

    pub fn writeback_strategy_rejection_count(&self) -> usize {
        self.writeback_strategy_rejection_count
    }

    pub fn writeback_idempotence_check_count(&self) -> usize {
        self.writeback_idempotence_check_count
    }

    pub fn writeback_causality_match_count(&self) -> usize {
        self.writeback_causality_match_count
    }

    pub fn writeback_loop_prevention_check_count(&self) -> usize {
        self.writeback_loop_prevention_check_count
    }

    pub fn writeback_loop_prevention_rejection_count(&self) -> usize {
        self.writeback_loop_prevention_rejection_count
    }

    pub fn writeback_noop_count(&self) -> usize {
        self.writeback_noop_count
    }

    pub fn writeback_commit_count(&self) -> usize {
        self.writeback_commit_count
    }

    pub fn writeback_failure_count(&self) -> usize {
        self.writeback_failure_count
    }

    pub fn writeback_authority_denial_count(&self) -> usize {
        self.writeback_authority_denial_count
    }

    pub fn writeback_validation_rejection_count(&self) -> usize {
        self.writeback_validation_rejection_count
    }

    pub fn writeback_replay_request_count(&self) -> usize {
        self.writeback_replay_request_count
    }

    pub fn writeback_replay_mismatch_count(&self) -> usize {
        self.writeback_replay_mismatch_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
