use super::failure_source::BridgeFailureSource;
use super::records::BridgeRouteRecord;
use crate::error::{BridgeDeliveryError, BridgeReplayError};

pub(crate) trait DiagnosticSink: Send + Sync {
    fn record_route(&self, record: BridgeRouteRecord);
    fn record_delivery_failure(&self, source: BridgeFailureSource, error: &BridgeDeliveryError);
    fn record_replay_failure(&self, source: BridgeFailureSource, error: &BridgeReplayError);
}
