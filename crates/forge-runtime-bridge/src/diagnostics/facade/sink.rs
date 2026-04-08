use super::*;

impl DiagnosticSink for BridgeDiagnosticsFacade {
    fn record_route(&self, record: BridgeRouteRecord) {
        BridgeDiagnosticsFacade::record_route(self, record);
    }

    fn record_historical_evaluation(&self, record: BridgeCanonicalHistoricalEvaluationRecord) {
        BridgeDiagnosticsFacade::record_historical_evaluation(self, record);
    }

    fn record_historical_evaluation_failure(&self, record: BridgeHistoricalEvaluationFailureRecord) {
        BridgeDiagnosticsFacade::record_historical_evaluation_failure(self, record);
    }

    fn record_delivery_failure(&self, source: BridgeFailureSource, error: &BridgeDeliveryError) {
        BridgeDiagnosticsFacade::record_delivery_failure(self, source, error);
    }

    fn record_replay_failure(&self, source: BridgeFailureSource, error: &BridgeReplayError) {
        BridgeDiagnosticsFacade::record_replay_failure(self, source, error);
    }
}
