mod admission;
mod bridge_lowering;
mod contracts;
mod cost;
mod counters;
mod error;
mod path_classes;
mod planner;
mod report;
mod request;
mod resolution;
#[cfg(test)]
mod tests;

pub use admission::{
    HistoricalEvaluationAdmission, HistoricalPathAdmitted, HistoricalPathSubstitutionDenied,
};
pub(crate) use bridge_lowering::{
    lower_materialization_from_decision_log, lower_policy_resolution,
};
pub use contracts::{
    HistoricalPathComplexityContract, HistoricalPathReuseDescriptor,
    HistoricalReconstructionBudget, HistoricalReplaySpanBudget,
};
pub use cost::{
    HistoricalPathCostPosture, HistoricalPerformanceStatusMarker,
    PerformancePredictionDriftOutcome, ReplayTailReuseEligibility, RetainedStateReuseEligibility,
};
pub use counters::HistoricalCounterSnapshot;
pub use error::{HistoricalEvaluationError, HistoricalEvaluationFailureClass};
pub use path_classes::{
    AdmittedHistoricalPathClass, HistoricalPathCompatibilityOutcome, RequestedHistoricalPathClass,
    ResolvedHistoricalPathClass,
};
pub use planner::{
    admit_historical_evaluation_path, materialization_metadata_from_resolved,
    resolve_historical_materialization_path,
};
pub use report::HistoricalPathVocabularyReport;
pub use request::{
    HistoricalCapabilityDescriptor, HistoricalEvaluationRequest,
    HistoricalMaterializationDescriptor, HistoricalPathRequested,
};
pub use resolution::{HistoricalMaterializationPathMetadata, HistoricalPathResolved};
