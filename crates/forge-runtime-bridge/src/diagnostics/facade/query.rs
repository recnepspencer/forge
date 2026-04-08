use super::*;

impl BridgeDiagnosticsFacade {
    pub(crate) fn new(policy: BridgeRuntimePolicy) -> Self {
        let retention_budget = policy.retention_budget();
        Self {
            config: Arc::new(BridgeDiagnosticsConfig {
                tier: policy.diagnostics_tier(),
                records_enabled: policy.record_route_artifacts(),
                replay_enabled: policy.allow_replay_artifacts(),
                route_record_limit: retention_budget.route_record_limit(),
                failure_record_limit: retention_budget.failure_record_limit(),
            }),
            state: Arc::new(RwLock::new(BridgeDiagnosticsState::default())),
        }
    }

    pub fn tier(&self) -> BridgeDiagnosticsTier {
        self.config.tier
    }

    pub fn records_enabled(&self) -> bool {
        self.config.records_enabled
    }

    pub fn route_record_limit(&self) -> usize {
        self.config.route_record_limit
    }

    pub fn failure_record_limit(&self) -> usize {
        self.config.failure_record_limit
    }

    pub fn route_records(&self) -> Vec<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .route_records()
    }

    pub fn failure_records(&self) -> Vec<BridgeFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .failure_records()
    }

    pub fn bulk_records(&self) -> Vec<BridgeCanonicalBulkPlanRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .bulk_records()
    }

    pub fn continuity_records(&self) -> Vec<BridgeCanonicalContinuityRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .continuity_records()
    }

    pub fn historical_evaluation_records(&self) -> Vec<BridgeCanonicalHistoricalEvaluationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_records()
    }

    pub fn historical_evaluation_failures(&self) -> Vec<BridgeHistoricalEvaluationFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_failures()
    }

    pub fn stream_checkpoints(&self) -> Vec<ConsumerCheckpointToken> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_checkpoints()
    }

    pub fn stream_replay_records(&self) -> Vec<CanonicalStreamReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_replay_records()
    }

    pub fn last_failure_record(&self) -> Option<BridgeFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_failure_record()
    }

    pub fn last_canonical_continuity_record(&self) -> Option<BridgeCanonicalContinuityRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_continuity_record()
    }

    pub fn last_bulk_record(&self) -> Option<BridgeCanonicalBulkPlanRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_bulk_record()
    }

    pub fn last_historical_evaluation_record(&self) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_historical_record()
    }

    pub fn last_historical_evaluation_failure(&self) -> Option<BridgeHistoricalEvaluationFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_historical_failure()
    }

    pub fn last_stream_checkpoint(&self) -> Option<ConsumerCheckpointToken> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_stream_checkpoint()
    }

    pub fn last_stream_replay_record(&self) -> Option<CanonicalStreamReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_stream_replay_record()
    }

    pub fn last_route_record(&self) -> Option<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_route_record()
    }

    pub fn route_record_for_route_identity(
        &self,
        route_identity: &str,
    ) -> Option<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .route_record_for_route_identity(route_identity)
    }

    pub fn route_record_for_invalidation_identity(
        &self,
        invalidation_identity: &str,
    ) -> Option<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .route_record_for_invalidation_identity(invalidation_identity)
    }

    pub fn route_record_for_source_commit(&self, source_commit: &str) -> Option<BridgeRouteRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .route_record_for_source_commit(source_commit)
    }

    pub fn continuity_record_for_route_identity(
        &self,
        route_identity: &str,
    ) -> Option<BridgeCanonicalContinuityRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .continuity_record_for_route_identity(route_identity)
    }

    pub fn bulk_record_for_workload_identity(
        &self,
        workload_identity: &str,
    ) -> Option<BridgeCanonicalBulkPlanRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .bulk_record_for_workload_identity(workload_identity)
    }

    pub fn historical_record_for_record_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_record_for_record_identity(record_identity)
    }

    pub fn historical_record_for_decision_log_identity(
        &self,
        decision_log_identity: &str,
    ) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_record_for_decision_log_identity(decision_log_identity)
    }

    pub fn historical_failure_for_declaration_identity(
        &self,
        declaration_identity: &str,
    ) -> Option<BridgeHistoricalEvaluationFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .historical_failure_for_declaration_identity(declaration_identity)
    }

    pub fn stream_checkpoint_for_identity(
        &self,
        checkpoint_identity: &str,
    ) -> Option<ConsumerCheckpointToken> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_checkpoint_for_identity(checkpoint_identity)
    }

    pub fn stream_replay_record_for_identity(
        &self,
        replay_record_identity: &str,
    ) -> Option<CanonicalStreamReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_replay_record_for_identity(replay_record_identity)
    }

    pub fn stream_replay_record_for_checkpoint_identity(
        &self,
        checkpoint_identity: &str,
    ) -> Option<CanonicalStreamReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_replay_record_for_checkpoint_identity(checkpoint_identity)
    }

    pub fn replay_records(&self) -> Vec<BridgeReplayRecord> {
        if !self.config.replay_enabled {
            return Vec::new();
        }

        self.route_records()
            .into_iter()
            .map(BridgeReplayRecord::from_route_record)
            .collect()
    }

    pub fn canonical_route_records(&self) -> Vec<BridgeCanonicalRouteRecord> {
        self.route_records()
            .into_iter()
            .map(BridgeCanonicalRouteRecord::from_route_record)
            .collect()
    }

}
