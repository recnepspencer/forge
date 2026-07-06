use forge_store_blob_chunks::{
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass,
    BlobHarnessChunkTopology, BlobHarnessFailurePoint, BlobHarnessPlacementClass,
    BlobHarnessSecurityScopeClass, BlobHarnessSizeClass, BlobHarnessTopologyDenial,
};

use super::profile::BlobHarnessProfile;
use super::shortcut_denial::BlobHarnessShortcutDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobHarnessScenarioSeed {
    profile: BlobHarnessProfile,
    size_class: BlobHarnessSizeClass,
    chunk_size_class: BlobHarnessChunkSizeClass,
    placement_class: BlobHarnessPlacementClass,
    security_scope: BlobHarnessSecurityScopeClass,
    access_mode: BlobHarnessAccessMode,
    failure_point: BlobHarnessFailurePoint,
    actor_mix: BlobHarnessActorMix,
    topology: BlobHarnessChunkTopology,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobHarnessScenarioSeedBuilder {
    profile: BlobHarnessProfile,
    size_class: BlobHarnessSizeClass,
    chunk_size_class: BlobHarnessChunkSizeClass,
    placement_class: BlobHarnessPlacementClass,
    security_scope: BlobHarnessSecurityScopeClass,
    access_mode: BlobHarnessAccessMode,
    failure_point: BlobHarnessFailurePoint,
    actor_mix: BlobHarnessActorMix,
    include_chunk_counters: bool,
}

impl BlobHarnessScenarioSeed {
    pub const fn builder() -> BlobHarnessScenarioSeedBuilder {
        BlobHarnessScenarioSeedBuilder::new()
    }

    pub const fn profile(&self) -> BlobHarnessProfile {
        self.profile
    }

    pub const fn topology(&self) -> BlobHarnessChunkTopology {
        self.topology
    }

    pub const fn size_class(&self) -> BlobHarnessSizeClass {
        self.size_class
    }

    pub const fn chunk_size_class(&self) -> BlobHarnessChunkSizeClass {
        self.chunk_size_class
    }

    pub const fn placement_class(&self) -> BlobHarnessPlacementClass {
        self.placement_class
    }

    pub const fn security_scope(&self) -> BlobHarnessSecurityScopeClass {
        self.security_scope
    }

    pub const fn access_mode(&self) -> BlobHarnessAccessMode {
        self.access_mode
    }

    pub const fn failure_point(&self) -> BlobHarnessFailurePoint {
        self.failure_point
    }

    pub const fn actor_mix(&self) -> BlobHarnessActorMix {
        self.actor_mix
    }
}

impl BlobHarnessScenarioSeedBuilder {
    pub const fn new() -> Self {
        Self {
            profile: BlobHarnessProfile::Local,
            size_class: BlobHarnessSizeClass::LocalDeterministic,
            chunk_size_class: BlobHarnessChunkSizeClass::Fixed64KiB,
            placement_class: BlobHarnessPlacementClass::StoreLocal,
            security_scope: BlobHarnessSecurityScopeClass::ScopePreserving,
            access_mode: BlobHarnessAccessMode::ReadOnlyReplay,
            failure_point: BlobHarnessFailurePoint::NoFaultSeed,
            actor_mix: BlobHarnessActorMix::SeedReplayOnly,
            include_chunk_counters: true,
        }
    }

    pub const fn profile(mut self, profile: BlobHarnessProfile) -> Self {
        self.profile = profile;
        self.size_class = profile.size_class();
        self.chunk_size_class = profile.chunk_size_class();
        self
    }

    pub const fn blob_size_class_larger_than_memory(mut self) -> Self {
        self.size_class = BlobHarnessSizeClass::MemoryEnvelopeExceeding;
        self
    }

    pub const fn tiny_blob_shortcut(mut self) -> Self {
        self.size_class = BlobHarnessSizeClass::TinyShortcut;
        self
    }

    pub const fn chunk_size_class_fixed(mut self) -> Self {
        self.chunk_size_class = self.profile.chunk_size_class();
        self
    }

    pub const fn placement_external(mut self) -> Self {
        self.placement_class = BlobHarnessPlacementClass::ExternalPlacementObserved;
        self
    }

    pub const fn security_scope_preserving(mut self) -> Self {
        self.security_scope = BlobHarnessSecurityScopeClass::ScopePreserving;
        self
    }

    pub const fn read_only_access(mut self) -> Self {
        self.access_mode = BlobHarnessAccessMode::ReadOnlyReplay;
        self
    }

    pub const fn seed_actor_mix(mut self) -> Self {
        self.actor_mix = BlobHarnessActorMix::SeedReplayOnly;
        self
    }

    pub const fn without_chunk_counters(mut self) -> Self {
        self.include_chunk_counters = false;
        self
    }

    pub fn build(self) -> Result<BlobHarnessScenarioSeed, BlobHarnessShortcutDenial> {
        if self.size_class.is_shortcut_sized() {
            return Err(BlobHarnessShortcutDenial::TinyBlobCannotSatisfyProfileEnvelope);
        }
        if !self.include_chunk_counters {
            return Err(BlobHarnessShortcutDenial::MissingChunkCounters);
        }
        let topology =
            BlobHarnessChunkTopology::from_classes(self.size_class, self.chunk_size_class)
                .map_err(map_topology_denial)?;
        Ok(BlobHarnessScenarioSeed {
            profile: self.profile,
            size_class: self.size_class,
            chunk_size_class: self.chunk_size_class,
            placement_class: self.placement_class,
            security_scope: self.security_scope,
            access_mode: self.access_mode,
            failure_point: self.failure_point,
            actor_mix: self.actor_mix,
            topology,
        })
    }
}

impl Default for BlobHarnessScenarioSeedBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn map_topology_denial(denial: BlobHarnessTopologyDenial) -> BlobHarnessShortcutDenial {
    match denial {
        BlobHarnessTopologyDenial::TinyBlobShortcut => {
            BlobHarnessShortcutDenial::TinyBlobCannotSatisfyProfileEnvelope
        }
        BlobHarnessTopologyDenial::MissingChunkCounters => {
            BlobHarnessShortcutDenial::MissingChunkCounters
        }
    }
}
