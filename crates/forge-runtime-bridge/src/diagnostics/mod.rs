mod continuity;
mod bulk;
mod explanation;
mod history;
mod records;
mod replay;
mod stream;
mod facade;
mod failure_source;
mod handle;
mod sink;
mod state;

pub use continuity::{
    BridgeCanonicalContinuityRecord, BridgeContinuityExplanation, BridgeContinuityReplaySummary,
    BridgeDeliveredContinuityResult,
    BRIDGE_CANONICAL_CONTINUITY_RECORD_SCHEMA_V1,
};
pub use bulk::BridgeBulkPlanExplanation;
pub use history::{
    BridgeCanonicalHistoricalEvaluationRecord, BridgeHistoricalEvaluationDecisionLog,
    BridgeHistoricalEvaluationCounters, BridgeHistoricalEvaluationExplanation,
    BridgeHistoricalEvaluationFailureClass, BridgeHistoricalEvaluationFailureRecord,
    BridgeHistoricalEvaluationRecord, BridgeHistoricalEvaluationReplaySummary,
    BridgeHistoricalMaterializationPath,
    BRIDGE_CANONICAL_HISTORICAL_EVALUATION_RECORD_SCHEMA_V1,
};
pub use explanation::{BridgeRouteExplanation, BridgeRouteExplanationEntry};
pub use records::{
    BridgeContractDiagnosticsRecord, BridgeFailureClass, BridgeFailureRecord,
    BridgeLoweringDiagnosticsRecord, BridgeRouteRecord, BridgeRouteRecordEntry,
    BridgeRouteRecordMatch, BridgeRouteSourceRecord, BridgeRoutingDiagnosticsRecord,
};
pub use replay::{
    BridgeCanonicalRouteRecord, BridgeReplayRecord, BridgeReplaySummary,
    BRIDGE_CANONICAL_ROUTE_RECORD_SCHEMA_V3,
};
pub use stream::{
    BridgeStreamCheckpointExplanation, BridgeStreamReplayExplanation,
    BridgeStreamResumeSummary,
};
pub use facade::BridgeDiagnosticsFacade;
pub use handle::BridgeDiagnosticsHandle;

pub(crate) use failure_source::BridgeFailureSource;
pub(crate) use sink::DiagnosticSink;
