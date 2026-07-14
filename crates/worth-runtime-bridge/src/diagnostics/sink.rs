use super::failure_source::BridgeFailureSource;
use super::history::{
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationFailureRecord,
};
use super::records::BridgeRouteRecord;
use crate::error::{BridgeDeliveryError, BridgeReplayError};

pub(crate) trait DiagnosticSink: Send + Sync {
    fn record_route(&self, record: BridgeRouteRecord);
    fn record_historical_evaluation(&self, record: BridgeCanonicalHistoricalEvaluationRecord);
    fn record_historical_evaluation_failure(&self, record: BridgeHistoricalEvaluationFailureRecord);
    fn record_delivery_failure(&self, source: BridgeFailureSource, error: &BridgeDeliveryError);
    fn record_replay_failure(&self, source: BridgeFailureSource, error: &BridgeReplayError);
}
