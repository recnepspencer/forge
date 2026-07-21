//! Replay-audience surface: exact re-exports from the Query engine.

pub use worth_query::facade::certification::{
    admit_installed_historical_replay_basis, issue_query_certification_replay_capability,
    replay_installed_workflow, replay_installed_workflow_historical,
    WorthQueryCertificationReplayAdmissionDenial, WorthQueryCertificationReplayCapability,
    WorthQueryCertificationReplayCounters, WorthQueryCertificationReplayOutcome,
    WorthQueryCertificationReplayResult, WorthQueryCertificationReplayStop,
    WorthQueryHistoricalReplayAdmission, WorthQueryHistoricalReplayAdmissionDenial,
    WorthQueryInstalledHistoricalReplayPath, WorthQueryReplayBasisRelationship,
};
/// Narrow cert-only scoped replay basis.
///
/// ```
/// use worth_query_replay::facade::ScopedReplayBasis;
/// # fn _inspect(capability: &ScopedReplayBasis) {
/// #     let _ = capability;
/// # }
/// ```
pub use worth_query::facade::foundation::ScopedReplayBasis;
pub use worth_query::facade::history::WorthQueryHistoricalContext;
pub use worth_query::facade::domain::{
    WorthQueryReplayComparison, WorthQueryReplayDivergence,
};
