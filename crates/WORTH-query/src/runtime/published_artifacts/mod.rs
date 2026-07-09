mod counters;
mod diagnostics;
mod entry;
mod registry;

use std::sync::Arc;

use crate::runtime::{WorthQueryDerivedArtifactBinding, WorthQueryRuntimeAsyncResultState};

pub use counters::WorthQueryPublishedArtifactCounterSnapshot;
pub(in crate::runtime) use counters::WorthQueryPublishedArtifactCounters;
pub use diagnostics::{
    WorthQueryPublishedArtifactDiagnostics, WorthQueryPublishedArtifactGenerationDiagnostic,
};
pub(in crate::runtime) use entry::WorthQueryPublishedArtifactEntry;
pub(in crate::runtime) use registry::WorthQueryPublishedArtifactRegistry;

#[derive(Clone, Debug)]
pub(in crate::runtime) enum WorthQueryPublishedArtifactResolution {
    Published {
        binding: Arc<WorthQueryDerivedArtifactBinding>,
        async_result_state: Option<WorthQueryRuntimeAsyncResultState>,
    },
    Unpublished {
        async_result_state: WorthQueryRuntimeAsyncResultState,
    },
    MissingGeneration,
    MissingView,
}
