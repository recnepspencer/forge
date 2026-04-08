use super::*;

impl BridgeDiagnosticsState {
    pub(crate) fn route_records(&self) -> Vec<BridgeRouteRecord> {
        self.route_records.iter().map(|record| (**record).clone()).collect()
    }

    pub(crate) fn bulk_records(&self) -> Vec<BridgeCanonicalBulkPlanRecord> {
        self.bulk_records.iter().map(|record| (**record).clone()).collect()
    }

    pub(crate) fn failure_records(&self) -> Vec<BridgeFailureRecord> {
        self.failure_records.iter().map(|record| (**record).clone()).collect()
    }

    pub(crate) fn continuity_records(&self) -> Vec<BridgeCanonicalContinuityRecord> {
        self.continuity_records.iter().map(|record| (**record).clone()).collect()
    }

    pub(crate) fn historical_records(&self) -> Vec<BridgeCanonicalHistoricalEvaluationRecord> {
        self.historical_records.iter().map(|record| (**record).clone()).collect()
    }

    pub(crate) fn historical_failures(&self) -> Vec<BridgeHistoricalEvaluationFailureRecord> {
        self.historical_failures.iter().map(|record| (**record).clone()).collect()
    }

    pub(crate) fn stream_checkpoints(&self) -> Vec<ConsumerCheckpointToken> {
        self.stream_checkpoints.iter().map(|record| (**record).clone()).collect()
    }

    pub(crate) fn stream_replay_records(&self) -> Vec<CanonicalStreamReplayRecord> {
        self.stream_replay_records.iter().map(|record| (**record).clone()).collect()
    }

    pub(crate) fn last_route_record(&self) -> Option<BridgeRouteRecord> {
        self.route_records.back().map(|record| (**record).clone())
    }

    pub(crate) fn last_failure_record(&self) -> Option<BridgeFailureRecord> {
        self.failure_records.back().map(|record| (**record).clone())
    }

    pub(crate) fn last_bulk_record(&self) -> Option<BridgeCanonicalBulkPlanRecord> {
        self.bulk_records.back().map(|record| (**record).clone())
    }

    pub(crate) fn last_continuity_record(&self) -> Option<BridgeCanonicalContinuityRecord> {
        self.continuity_records.back().map(|record| (**record).clone())
    }

    pub(crate) fn last_historical_record(&self) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.historical_records.back().map(|record| (**record).clone())
    }

    pub(crate) fn last_historical_failure(&self) -> Option<BridgeHistoricalEvaluationFailureRecord> {
        self.historical_failures.back().map(|record| (**record).clone())
    }

    pub(crate) fn last_stream_checkpoint(&self) -> Option<ConsumerCheckpointToken> {
        self.stream_checkpoints.back().map(|record| (**record).clone())
    }

    pub(crate) fn last_stream_replay_record(&self) -> Option<CanonicalStreamReplayRecord> {
        self.stream_replay_records.back().map(|record| (**record).clone())
    }

    pub(crate) fn route_record_for_route_identity(&self, route_identity: &str) -> Option<BridgeRouteRecord> {
        self.latest_route_by_route_identity.get(route_identity).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn route_record_for_invalidation_identity(
        &self,
        invalidation_identity: &str,
    ) -> Option<BridgeRouteRecord> {
        self.latest_route_by_invalidation_identity.get(invalidation_identity).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn route_record_for_source_commit(&self, source_commit: &str) -> Option<BridgeRouteRecord> {
        self.latest_route_by_source_commit.get(source_commit).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn continuity_record_for_route_identity(
        &self,
        route_identity: &str,
    ) -> Option<BridgeCanonicalContinuityRecord> {
        self.latest_continuity_by_route_identity.get(route_identity).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn bulk_record_for_workload_identity(
        &self,
        workload_identity: &str,
    ) -> Option<BridgeCanonicalBulkPlanRecord> {
        self.latest_bulk_by_workload_identity.get(workload_identity).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn historical_record_for_record_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.latest_historical_by_record_identity.get(record_identity).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn historical_record_for_decision_log_identity(
        &self,
        decision_log_identity: &str,
    ) -> Option<BridgeCanonicalHistoricalEvaluationRecord> {
        self.latest_historical_by_decision_log_identity.get(decision_log_identity).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn historical_failure_for_declaration_identity(
        &self,
        declaration_identity: &str,
    ) -> Option<BridgeHistoricalEvaluationFailureRecord> {
        self.latest_historical_failure_by_declaration_identity.get(declaration_identity).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn stream_checkpoint_for_identity(&self, checkpoint_identity: &str) -> Option<ConsumerCheckpointToken> {
        self.latest_stream_checkpoint_by_identity.get(checkpoint_identity).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn stream_replay_record_for_identity(
        &self,
        replay_record_identity: &str,
    ) -> Option<CanonicalStreamReplayRecord> {
        self.latest_stream_replay_by_identity.get(replay_record_identity).cloned().map(|record| (*record).clone())
    }

    pub(crate) fn stream_replay_record_for_checkpoint_identity(
        &self,
        checkpoint_identity: &str,
    ) -> Option<CanonicalStreamReplayRecord> {
        self.latest_stream_replay_by_checkpoint_identity.get(checkpoint_identity).cloned().map(|record| (*record).clone())
    }
}
