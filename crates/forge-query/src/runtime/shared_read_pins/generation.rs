use std::collections::BTreeMap;
use std::sync::Arc;

use crate::runtime::shared_read::SharedReadDerivedViewState;

use super::ForgeQuerySharedReadPinRegistry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct ForgeQuerySharedReadGenerationId {
    ordinal: u64,
    snapshot_token: String,
}

impl ForgeQuerySharedReadGenerationId {
    pub(in crate::runtime) fn new(ordinal: u64, snapshot_token: impl Into<String>) -> Self {
        Self {
            ordinal,
            snapshot_token: snapshot_token.into(),
        }
    }

    pub(in crate::runtime) fn ordinal(&self) -> u64 {
        self.ordinal
    }

    pub(in crate::runtime) fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::runtime) struct ForgeQuerySharedReadPinnedSnapshot {
    generation: ForgeQuerySharedReadGenerationId,
    derived_views: BTreeMap<String, SharedReadDerivedViewState>,
}

impl ForgeQuerySharedReadPinnedSnapshot {
    pub(in crate::runtime) fn new(
        generation: ForgeQuerySharedReadGenerationId,
        derived_views: BTreeMap<String, SharedReadDerivedViewState>,
    ) -> Self {
        Self {
            generation,
            derived_views,
        }
    }

    pub(in crate::runtime) fn generation(&self) -> &ForgeQuerySharedReadGenerationId {
        &self.generation
    }

    pub(in crate::runtime) fn derived_views(&self) -> &BTreeMap<String, SharedReadDerivedViewState> {
        &self.derived_views
    }
}

#[derive(Debug)]
pub(in crate::runtime) struct ForgeQuerySharedReadGenerationLease {
    registry: ForgeQuerySharedReadPinRegistry,
    snapshot: Arc<ForgeQuerySharedReadPinnedSnapshot>,
}

impl PartialEq for ForgeQuerySharedReadGenerationLease {
    fn eq(&self, other: &Self) -> bool {
        self.snapshot == other.snapshot
    }
}

impl ForgeQuerySharedReadGenerationLease {
    pub(in crate::runtime) fn new(
        registry: ForgeQuerySharedReadPinRegistry,
        snapshot: Arc<ForgeQuerySharedReadPinnedSnapshot>,
    ) -> Self {
        Self { registry, snapshot }
    }

    pub(in crate::runtime) fn snapshot(&self) -> &ForgeQuerySharedReadPinnedSnapshot {
        &self.snapshot
    }

    pub(in crate::runtime) fn generation(&self) -> &ForgeQuerySharedReadGenerationId {
        self.snapshot.generation()
    }

    pub(in crate::runtime) fn is_generation_live(&self) -> bool {
        self.registry
            .contains_generation(self.snapshot.generation().ordinal())
    }
}

impl Clone for ForgeQuerySharedReadGenerationLease {
    fn clone(&self) -> Self {
        self.registry.pin_generation(self.snapshot.generation().ordinal());
        Self {
            registry: self.registry.clone(),
            snapshot: Arc::clone(&self.snapshot),
        }
    }
}

impl Drop for ForgeQuerySharedReadGenerationLease {
    fn drop(&mut self) {
        self.registry
            .release_generation(self.snapshot.generation().ordinal());
    }
}
