use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;

use crate::routing::BridgeCanonicalBulkPlanRecord;
use crate::source::{SourceFailureRecord, SourceMaterializationRecord};
use crate::stream::{CanonicalStreamReplayRecord, ConsumerCheckpointToken};

use super::continuity::BridgeCanonicalContinuityRecord;
use super::history::{
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationFailureRecord,
};
use super::merge::BridgeCanonicalMergeRecord;
use super::records::{BridgeFailureRecord, BridgeRouteRecord};
use super::structural::{
    BridgeCanonicalStructuralBranchComparisonRecord, BridgeCanonicalStructuralRemapRecord,
};
use crate::speculation::{
    BridgePreviewDiscardRecord, BridgePreviewExecutionRecord, BridgePreviewPromotionRecord,
};

mod config;
mod evict;
mod query;
mod record;
mod speculation;

pub(crate) use config::BridgeDiagnosticsConfig;

#[derive(Debug, Default)]
pub(crate) struct BridgeDiagnosticsState {
    route_records: VecDeque<Arc<BridgeRouteRecord>>,
    bulk_records: VecDeque<Arc<BridgeCanonicalBulkPlanRecord>>,
    continuity_records: VecDeque<Arc<BridgeCanonicalContinuityRecord>>,
    merge_records: VecDeque<Arc<BridgeCanonicalMergeRecord>>,
    historical_records: VecDeque<Arc<BridgeCanonicalHistoricalEvaluationRecord>>,
    historical_failures: VecDeque<Arc<BridgeHistoricalEvaluationFailureRecord>>,
    source_materialization_records: VecDeque<Arc<SourceMaterializationRecord>>,
    source_failure_records: VecDeque<Arc<SourceFailureRecord>>,
    structural_remap_records: VecDeque<Arc<BridgeCanonicalStructuralRemapRecord>>,
    structural_branch_comparison_records:
        VecDeque<Arc<BridgeCanonicalStructuralBranchComparisonRecord>>,
    preview_execution_records: VecDeque<Arc<BridgePreviewExecutionRecord>>,
    preview_discard_records: VecDeque<Arc<BridgePreviewDiscardRecord>>,
    preview_promotion_records: VecDeque<Arc<BridgePreviewPromotionRecord>>,
    stream_checkpoints: VecDeque<Arc<ConsumerCheckpointToken>>,
    stream_replay_records: VecDeque<Arc<CanonicalStreamReplayRecord>>,
    failure_records: VecDeque<Arc<BridgeFailureRecord>>,
    latest_route_by_route_identity: BTreeMap<String, Arc<BridgeRouteRecord>>,
    latest_bulk_by_workload_identity: BTreeMap<String, Arc<BridgeCanonicalBulkPlanRecord>>,
    latest_continuity_by_route_identity: BTreeMap<String, Arc<BridgeCanonicalContinuityRecord>>,
    latest_merge_by_record_identity: BTreeMap<String, Arc<BridgeCanonicalMergeRecord>>,
    latest_historical_by_record_identity:
        BTreeMap<String, Arc<BridgeCanonicalHistoricalEvaluationRecord>>,
    latest_historical_by_decision_log_identity:
        BTreeMap<String, Arc<BridgeCanonicalHistoricalEvaluationRecord>>,
    latest_historical_failure_by_declaration_identity:
        BTreeMap<String, Arc<BridgeHistoricalEvaluationFailureRecord>>,
    latest_source_materialization_by_record_identity:
        BTreeMap<String, Arc<SourceMaterializationRecord>>,
    latest_source_failure_by_declaration_identity: BTreeMap<String, Arc<SourceFailureRecord>>,
    latest_structural_remap_by_record_identity:
        BTreeMap<String, Arc<BridgeCanonicalStructuralRemapRecord>>,
    latest_structural_branch_comparison_by_record_identity:
        BTreeMap<String, Arc<BridgeCanonicalStructuralBranchComparisonRecord>>,
    latest_preview_execution_by_record_identity:
        BTreeMap<String, Arc<BridgePreviewExecutionRecord>>,
    latest_preview_execution_by_session_identity:
        BTreeMap<String, Arc<BridgePreviewExecutionRecord>>,
    latest_preview_discard_by_record_identity: BTreeMap<String, Arc<BridgePreviewDiscardRecord>>,
    latest_preview_discard_by_session_identity: BTreeMap<String, Arc<BridgePreviewDiscardRecord>>,
    latest_preview_promotion_by_record_identity:
        BTreeMap<String, Arc<BridgePreviewPromotionRecord>>,
    latest_preview_promotion_by_session_identity:
        BTreeMap<String, Arc<BridgePreviewPromotionRecord>>,
    reserved_preview_session_identities: BTreeSet<String>,
    latest_stream_checkpoint_by_identity: BTreeMap<String, Arc<ConsumerCheckpointToken>>,
    latest_stream_replay_by_identity: BTreeMap<String, Arc<CanonicalStreamReplayRecord>>,
    latest_stream_replay_by_checkpoint_identity: BTreeMap<String, Arc<CanonicalStreamReplayRecord>>,
    latest_route_by_invalidation_identity: BTreeMap<String, Arc<BridgeRouteRecord>>,
    latest_route_by_source_commit: BTreeMap<String, Arc<BridgeRouteRecord>>,
}
