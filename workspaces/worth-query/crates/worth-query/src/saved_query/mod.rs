mod artifact;
mod digest;
mod error;
mod freeze;
mod future_support;
mod reuse;
mod support;

pub use artifact::{
    SavedQueryArtifact, SavedQueryMetadata, SavedQueryPersistenceClaim, SavedQueryPersistenceFamily,
};
pub use digest::SavedQueryArtifactDigest;
pub use error::{SavedQueryError, SavedQueryFailureClass};
pub use freeze::{freeze_composed_saved_query, freeze_direct_saved_query, SavedQueryFreezeContext};
pub use future_support::{
    runtime_backed_saved_query_future_support_profile, SavedQueryTemporalAsyncSurfacePosture,
    SchemaBasisEquivalenceEvidence,
};
pub use reuse::{
    evaluate_saved_query_reuse, SavedQueryBindingMatrixArtifact, SavedQueryBindingMatrixRow,
    SavedQueryRebindingDimension, SavedQueryRebindingLegality, SavedQueryReuseDecision,
    SavedQueryReuseDenial, SavedQueryReuseDescriptor, SavedQueryReuseOutcome,
};
pub use support::{runtime_backed_saved_query_support_profile, SavedQueryComplexityStatus};

#[cfg(test)]
mod tests;
