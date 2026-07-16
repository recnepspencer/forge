mod adjudication;
mod artifact_identity;
mod bounds;
mod canonical_trace;
mod checked_operator;
mod counter_identity;
mod counterexample;
mod counters;
mod cross_protocol;
mod execution;
mod invariant_configuration;
mod invocation;
mod localization;
mod output;
mod receipt_loss;
mod refinement_coverage;
mod statistics;
mod trace_validation;
mod verdict;

pub use adjudication::{
    adjudicate_shared_frontier_trace, receipt_loss_outcome, ProtocolExecutionOutcome,
};
pub use artifact_identity::{
    ExecutedProtocolCheck, ProtocolArtifactIdentityInspectionDenial, ProtocolCheckArtifactIdentity,
};
pub use bounds::ProtocolCheckBounds;
pub use canonical_trace::{
    CanonicalProtocolAction, CanonicalProtocolTrace, CanonicalProtocolTraceDenial,
    ProtocolFrontierIdentity,
};
pub use checked_operator::{
    require_checked_operator_bindings, CheckedOperatorBinding, CheckedOperatorBindingDenial,
};
pub use counter_identity::{ProtocolArtifactIdentityPosture, ProtocolCounterEvidenceIdentity};
pub use counterexample::{ProtocolCounterexample, ProtocolCounterexampleState};
pub use counters::{
    project_checked_protocol_counters, project_counterexample_protocol_counters,
    ProtocolConformanceCounterInput, ProtocolCounterProjectionDenial, ProtocolCounterSnapshot,
    ProtocolRunnerCounter,
};
pub use cross_protocol::{
    CrossProtocolLocalization, CrossProtocolLocalizationDenial, SharedFrontierIdentity,
};
pub use execution::{
    execute_protocol_check, execute_protocol_check_with_identity, ProtocolRunnerFailure,
    TlcRunnerPaths,
};
pub use invariant_configuration::{
    configured_invariant_count, ProtocolInvariantConfigurationDenial,
};
pub use invocation::{PinnedTlcToolchain, ProtocolCheckInvocation};
pub use localization::{
    AbstractionFunctionIdentity, CertificationLaneIdentity, CounterexampleLocalization,
    CounterexampleLocalizationDenial,
};
pub use output::{interpret_tlc_output, ProtocolCheckerOutputDenial};
pub use receipt_loss::{classify_receipt_loss, ReceiptLossClassification};
pub use refinement_coverage::{
    require_exact_protocol_refinement_coverage, ExactProtocolRefinementCoverageReceipt,
    ProtocolRefinementCoverageDenial,
};
pub use statistics::ProtocolCheckStatistics;
pub use trace_validation::{
    validate_canonical_protocol_trace, ProtocolTraceValidationDenial,
    ProtocolTraceValidationReceipt,
};
pub use verdict::ProtocolCheckVerdict;
