mod bulk;
mod continuity;
mod explanation;
mod facade;
mod failure_source;
mod handle;
mod history;
mod merge;
mod records;
mod replay;
mod sink;
mod source;
mod state;
mod stream;
mod structural;

pub use bulk::BridgeBulkPlanExplanation;
pub use continuity::{
    BridgeCanonicalContinuityRecord, BridgeContinuityExplanation, BridgeContinuityReplaySummary,
    BridgeDeliveredContinuityResult, BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1,
};
pub use explanation::{BridgeRouteExplanation, BridgeRouteExplanationEntry};
pub use facade::BridgeDiagnosticsFacade;
pub use handle::BridgeDiagnosticsHandle;
pub use history::{
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationCounters,
    BridgeHistoricalEvaluationDecisionLog, BridgeHistoricalEvaluationExplanation,
    BridgeHistoricalEvaluationFailureClass, BridgeHistoricalEvaluationFailureRecord,
    BridgeHistoricalEvaluationRecord, BridgeHistoricalEvaluationReplaySummary,
    BridgeHistoricalMaterializationPath, BRIDGE_CANONICAL_HISTORICAL_EVALUATION_RECORD_SCHEMA_V1,
};
pub use merge::{
    BridgeCanonicalMergeRecord, BridgeMergeExplanation, BridgeMergeRecord,
    BridgeMergeRecordIdentity, BridgeMergeReplaySummary, BRIDGE_CANONICAL_MERGE_RECORD_SCHEMA_V1,
};
pub use records::{
    BridgeContractDiagnosticsRecord, BridgeFailureClass, BridgeFailureRecord,
    BridgeLoweringDiagnosticsRecord, BridgeRouteRecord, BridgeRouteRecordEntry,
    BridgeRouteRecordMatch, BridgeRouteSourceRecord, BridgeRoutingDiagnosticsRecord,
};
pub use replay::{
    BridgeCanonicalRouteRecord, BridgeReplayRecord, BridgeReplaySummary,
    BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V3,
};
pub use source::{BridgeSourceFailureExplanation, BridgeSourceMaterializationExplanation};
pub use stream::{
    BridgeStreamCheckpointExplanation, BridgeStreamReplayExplanation, BridgeStreamResumeSummary,
};
pub use structural::{
    BridgeCanonicalStructuralBranchComparisonRecord, BridgeCanonicalStructuralRemapRecord,
    BridgeStructuralBranchComparisonExplanation, BridgeStructuralBranchComparisonRecord,
    BridgeStructuralBranchComparisonReplaySummary, BridgeStructuralCounters,
    BridgeStructuralRemapExplanation, BridgeStructuralRemapRecord,
    BridgeStructuralRemapReplaySummary,
    BRIDGE_CANONICAL_STRUCTURAL_BRANCH_COMPARISON_RECORD_SCHEMA_V1,
    BRIDGE_CANONICAL_STRUCTURAL_REMAP_RECORD_SCHEMA_V1,
};

pub(crate) use failure_source::BridgeFailureSource;
pub(crate) use sink::DiagnosticSink;
pub(crate) use structural::{
    validate_structural_replay_contract, validate_structural_replay_outcome,
};
