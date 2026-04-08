use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use crate::routing::BridgeCanonicalBulkPlanRecord;
use crate::stream::{CanonicalStreamReplayRecord, ConsumerCheckpointToken};

use super::continuity::BridgeCanonicalContinuityRecord;
use super::history::{
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationFailureRecord,
};
use super::records::{BridgeFailureRecord, BridgeRouteRecord};

mod config;
mod evict;
mod query;
mod record;

pub(crate) use config::BridgeDiagnosticsConfig;

#[derive(Debug, Default)]
pub(crate) struct BridgeDiagnosticsState {
    route_records: VecDeque<Arc<BridgeRouteRecord>>,
    bulk_records: VecDeque<Arc<BridgeCanonicalBulkPlanRecord>>,
    continuity_records: VecDeque<Arc<BridgeCanonicalContinuityRecord>>,
    historical_records: VecDeque<Arc<BridgeCanonicalHistoricalEvaluationRecord>>,
    historical_failures: VecDeque<Arc<BridgeHistoricalEvaluationFailureRecord>>,
    stream_checkpoints: VecDeque<Arc<ConsumerCheckpointToken>>,
    stream_replay_records: VecDeque<Arc<CanonicalStreamReplayRecord>>,
    failure_records: VecDeque<Arc<BridgeFailureRecord>>,
    latest_route_by_route_identity: BTreeMap<String, Arc<BridgeRouteRecord>>,
    latest_bulk_by_workload_identity: BTreeMap<String, Arc<BridgeCanonicalBulkPlanRecord>>,
    latest_continuity_by_route_identity: BTreeMap<String, Arc<BridgeCanonicalContinuityRecord>>,
    latest_historical_by_record_identity: BTreeMap<String, Arc<BridgeCanonicalHistoricalEvaluationRecord>>,
    latest_historical_by_decision_log_identity: BTreeMap<String, Arc<BridgeCanonicalHistoricalEvaluationRecord>>,
    latest_historical_failure_by_declaration_identity: BTreeMap<String, Arc<BridgeHistoricalEvaluationFailureRecord>>,
    latest_stream_checkpoint_by_identity: BTreeMap<String, Arc<ConsumerCheckpointToken>>,
    latest_stream_replay_by_identity: BTreeMap<String, Arc<CanonicalStreamReplayRecord>>,
    latest_stream_replay_by_checkpoint_identity: BTreeMap<String, Arc<CanonicalStreamReplayRecord>>,
    latest_route_by_invalidation_identity: BTreeMap<String, Arc<BridgeRouteRecord>>,
    latest_route_by_source_commit: BTreeMap<String, Arc<BridgeRouteRecord>>,
}
