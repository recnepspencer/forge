mod counters;
mod diagnostics;
mod entry;
mod registry;

use std::sync::Arc;

use crate::runtime::{ForgeQueryDerivedArtifactBinding, ForgeQueryRuntimeAsyncResultState};

pub use counters::ForgeQueryPublishedArtifactCounterSnapshot;
pub(in crate::runtime) use counters::ForgeQueryPublishedArtifactCounters;
pub use diagnostics::{
    ForgeQueryPublishedArtifactDiagnostics, ForgeQueryPublishedArtifactGenerationDiagnostic,
};
pub(in crate::runtime) use entry::ForgeQueryPublishedArtifactEntry;
pub(in crate::runtime) use registry::ForgeQueryPublishedArtifactRegistry;

#[derive(Clone, Debug)]
pub(in crate::runtime) enum ForgeQueryPublishedArtifactResolution {
    Published {
        binding: Arc<ForgeQueryDerivedArtifactBinding>,
        async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
    },
    Unpublished {
        async_result_state: ForgeQueryRuntimeAsyncResultState,
    },
    MissingGeneration,
    MissingView,
}
