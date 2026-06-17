use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use arc_swap::ArcSwap;

use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::shared_read_pins::ForgeQuerySharedReadGenerationId;

use super::{
    ForgeQueryPublishedArtifactCounters, ForgeQueryPublishedArtifactDiagnostics,
    ForgeQueryPublishedArtifactEntry, ForgeQueryPublishedArtifactGenerationDiagnostic,
    ForgeQueryPublishedArtifactResolution,
};

#[derive(Clone, Debug, Default)]
pub(in crate::runtime) struct ForgeQueryPublishedArtifactRegistry {
    generations: Arc<ArcSwap<ForgeQueryPublishedArtifactGenerations>>,
    counters: ForgeQueryPublishedArtifactCounters,
}

impl ForgeQueryPublishedArtifactRegistry {
    pub(in crate::runtime) fn publish_generation(
        &self,
        generation: &ForgeQuerySharedReadGenerationId,
        entries: BTreeMap<String, ForgeQueryPublishedArtifactEntry>,
    ) {
        let mut generations = self.generations.load_full().as_ref().clone();
        generations.publish_generation(generation, entries);
        self.generations.store(Arc::new(generations));
    }

    pub(in crate::runtime) fn retain_generations(&self, retained_ordinals: &BTreeSet<u64>) {
        let mut generations = self.generations.load_full().as_ref().clone();
        let dropped = generations.retain_generations(retained_ordinals);
        self.counters.record_dropped_generations(dropped);
        self.generations.store(Arc::new(generations));
    }

    pub(in crate::runtime) fn resolve(
        &self,
        generation: &ForgeQuerySharedReadGenerationId,
        view_name: &str,
    ) -> ForgeQueryPublishedArtifactResolution {
        let resolution = self.generations.load().resolve(generation, view_name);
        if matches!(
            resolution,
            ForgeQueryPublishedArtifactResolution::Published { .. }
        ) {
            self.counters.record_registry_lease();
        }
        resolution
    }

    pub(in crate::runtime) fn counters(&self) -> ForgeQueryPublishedArtifactCounters {
        self.counters.clone()
    }

    pub(in crate::runtime) fn diagnostics(&self) -> ForgeQueryPublishedArtifactDiagnostics {
        ForgeQueryPublishedArtifactDiagnostics::new(
            self.counters.snapshot(),
            self.generations.load().generation_diagnostics(),
        )
    }
}

#[derive(Clone, Debug, Default)]
struct ForgeQueryPublishedArtifactGenerations {
    generations: BTreeMap<u64, ForgeQueryPublishedArtifactGeneration>,
}

impl ForgeQueryPublishedArtifactGenerations {
    fn publish_generation(
        &mut self,
        generation: &ForgeQuerySharedReadGenerationId,
        entries: BTreeMap<String, ForgeQueryPublishedArtifactEntry>,
    ) {
        self.generations.insert(
            generation.ordinal(),
            ForgeQueryPublishedArtifactGeneration::new(
                generation.ordinal(),
                generation.snapshot_identity().clone(),
                entries,
            ),
        );
    }

    fn retain_generations(&mut self, retained_ordinals: &BTreeSet<u64>) -> usize {
        let before = self.generations.len();
        self.generations
            .retain(|ordinal, _| retained_ordinals.contains(ordinal));
        before.saturating_sub(self.generations.len())
    }

    fn resolve(
        &self,
        generation: &ForgeQuerySharedReadGenerationId,
        view_name: &str,
    ) -> ForgeQueryPublishedArtifactResolution {
        let Some(published_generation) = self.generations.get(&generation.ordinal()) else {
            return ForgeQueryPublishedArtifactResolution::MissingGeneration;
        };
        published_generation.resolve(generation, view_name)
    }

    fn generation_diagnostics(&self) -> Vec<ForgeQueryPublishedArtifactGenerationDiagnostic> {
        self.generations
            .values()
            .map(ForgeQueryPublishedArtifactGeneration::diagnostic)
            .collect()
    }
}

#[derive(Clone, Debug)]
struct ForgeQueryPublishedArtifactGeneration {
    ordinal: u64,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    entries: BTreeMap<String, ForgeQueryPublishedArtifactEntry>,
}

impl ForgeQueryPublishedArtifactGeneration {
    fn new(
        ordinal: u64,
        snapshot_identity: ForgeQuerySnapshotIdentity,
        entries: BTreeMap<String, ForgeQueryPublishedArtifactEntry>,
    ) -> Self {
        Self {
            ordinal,
            snapshot_identity,
            entries,
        }
    }

    fn resolve(
        &self,
        generation: &ForgeQuerySharedReadGenerationId,
        view_name: &str,
    ) -> ForgeQueryPublishedArtifactResolution {
        if self.ordinal != generation.ordinal()
            || self.snapshot_identity != *generation.snapshot_identity()
        {
            return ForgeQueryPublishedArtifactResolution::MissingGeneration;
        }
        let Some(entry) = self.entries.get(view_name) else {
            return ForgeQueryPublishedArtifactResolution::MissingView;
        };
        match entry.published_binding() {
            Some(binding) => ForgeQueryPublishedArtifactResolution::Published {
                binding,
                async_result_state: entry.async_result_state(),
            },
            None => ForgeQueryPublishedArtifactResolution::Unpublished {
                async_result_state: entry
                    .async_result_state()
                    .expect("unpublished artifact entries carry async posture"),
            },
        }
    }

    fn diagnostic(&self) -> ForgeQueryPublishedArtifactGenerationDiagnostic {
        ForgeQueryPublishedArtifactGenerationDiagnostic::new(
            self.ordinal,
            self.snapshot_identity.clone(),
            self.entries.len(),
        )
    }
}
