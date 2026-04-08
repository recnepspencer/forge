use super::*;

impl BridgeDiagnosticsFacade {
    pub fn explain_route_record(&self, record: &BridgeRouteRecord) -> BridgeRouteExplanation {
        BridgeRouteExplanation::from_route_record(record)
    }

    pub fn explain_last_route_record(&self) -> Option<BridgeRouteExplanation> {
        self.last_route_record()
            .map(|record| BridgeRouteExplanation::from_route_record(&record))
    }

    pub fn explain_continuity_record(
        &self,
        record: &BridgeCanonicalContinuityRecord,
    ) -> BridgeContinuityExplanation {
        BridgeContinuityExplanation::from_canonical_record(record)
    }

    pub fn explain_last_continuity_record(&self) -> Option<BridgeContinuityExplanation> {
        self.last_canonical_continuity_record()
            .map(|record| BridgeContinuityExplanation::from_canonical_record(&record))
    }

    pub fn explain_bulk_record(
        &self,
        record: &BridgeCanonicalBulkPlanRecord,
    ) -> BridgeBulkPlanExplanation {
        BridgeBulkPlanExplanation::from_canonical_record(record)
    }

    pub fn explain_last_bulk_record(&self) -> Option<BridgeBulkPlanExplanation> {
        self.last_bulk_record()
            .map(|record| BridgeBulkPlanExplanation::from_canonical_record(&record))
    }

    pub fn explain_historical_evaluation_record(
        &self,
        record: &BridgeCanonicalHistoricalEvaluationRecord,
    ) -> BridgeHistoricalEvaluationExplanation {
        BridgeHistoricalEvaluationExplanation::from_canonical_record(record)
    }

    pub fn explain_last_historical_evaluation_record(
        &self,
    ) -> Option<BridgeHistoricalEvaluationExplanation> {
        self.last_historical_evaluation_record()
            .map(|record| BridgeHistoricalEvaluationExplanation::from_canonical_record(&record))
    }

    pub fn explain_stream_checkpoint(
        &self,
        checkpoint: &ConsumerCheckpointToken,
    ) -> BridgeStreamCheckpointExplanation {
        BridgeStreamCheckpointExplanation::from_checkpoint(checkpoint)
    }

    pub fn explain_last_stream_checkpoint(&self) -> Option<BridgeStreamCheckpointExplanation> {
        self.last_stream_checkpoint()
            .map(|checkpoint| BridgeStreamCheckpointExplanation::from_checkpoint(&checkpoint))
    }

    pub fn explain_stream_replay_record(
        &self,
        record: &CanonicalStreamReplayRecord,
    ) -> BridgeStreamReplayExplanation {
        BridgeStreamReplayExplanation::from_replay_record(record)
    }

    pub fn explain_last_stream_replay_record(&self) -> Option<BridgeStreamReplayExplanation> {
        self.last_stream_replay_record()
            .map(|record| BridgeStreamReplayExplanation::from_replay_record(&record))
    }

    pub fn last_canonical_route_record(&self) -> Option<BridgeCanonicalRouteRecord> {
        self.last_route_record()
            .map(BridgeCanonicalRouteRecord::from_route_record)
    }

    pub fn handle(&self) -> BridgeDiagnosticsHandle {
        BridgeDiagnosticsHandle {
            config: Arc::clone(&self.config),
            state: Arc::clone(&self.state),
        }
    }

}
