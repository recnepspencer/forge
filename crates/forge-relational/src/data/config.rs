use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::data::payload::PayloadPolicy;
use crate::data::symbols::SymbolPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RelationalRuntimeProfile {
    CertificationCore,
    GeometryKernel,
    ChipSimulation,
    AiWorkflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConfigValueSource {
    ProfileDefault,
    BuilderOverride,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigProvenanceEntry {
    pub source: ConfigValueSource,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigProvenance {
    pub profile: RelationalRuntimeProfile,
    pub entries: BTreeMap<String, ConfigProvenanceEntry>,
}

impl ConfigProvenance {
    pub fn source_for(&self, key: &str) -> Option<&ConfigProvenanceEntry> {
        self.entries.get(key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotReleasePolicy {
    ExplicitRelease,
    ReleaseOnRetentionPass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetentionBackend {
    PinTrackedRetention,
    EpochChunkRetention,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub backend: RetentionBackend,
    pub reclaim_batch_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MvccConfig {
    pub track_visibility_metadata: bool,
    pub snapshot_release_policy: SnapshotReleasePolicy,
    pub auto_reclaim_deleted_records: bool,
    pub reclaim_batch_size: usize,
    pub retention_backend: RetentionBackend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdjacencyBackend {
    InlineSmallDegreeAdjacency,
    CompressedFanoutAdjacency,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdjacencyPolicy {
    pub backend: AdjacencyBackend,
    pub small_degree_inline_capacity: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchSurfacePolicy {
    StructuredPatchSurface,
    DensePatchSurface,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompiledLanePolicy {
    Disabled,
    DerivedCompiledLane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DurableLogRetentionMode {
    RetainAllInMemory,
    CompactAfterCheckpoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableLogPolicy {
    pub retention_mode: DurableLogRetentionMode,
    pub max_in_memory_envelopes: usize,
    pub compact_after_checkpoint: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageLayoutConfig {
    pub entity_chunk_size: usize,
    pub relation_chunk_size: usize,
    pub scan_packet_size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrossContextPolicy {
    AllowExplicit,
    SchemaControlled,
    Forbid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CascadeDeletePolicy {
    RetainDanglingForAudit,
    CascadeDeleteRelations,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationConfig {
    pub coherent_publication_required: bool,
    pub max_patch_records_per_commit: usize,
    pub patch_surface_policy: PatchSurfacePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalConfigOverride {
    pub runtime_name: Option<String>,
    pub initial_entity_capacity: Option<usize>,
    pub initial_relation_capacity: Option<usize>,
    pub mvcc: Option<MvccConfig>,
    pub storage_layout: Option<StorageLayoutConfig>,
    pub publication: Option<PublicationConfig>,
    pub payload_policy: Option<PayloadPolicy>,
    pub symbol_policy: Option<SymbolPolicy>,
    pub durable_log_policy: Option<DurableLogPolicy>,
    pub adjacency_policy: Option<AdjacencyPolicy>,
    pub cross_context_policy: Option<CrossContextPolicy>,
    pub cascade_delete_policy: Option<CascadeDeletePolicy>,
    pub compiled_lane_policy: Option<CompiledLanePolicy>,
}

impl RelationalConfigOverride {
    pub fn is_empty(&self) -> bool {
        self.runtime_name.is_none()
            && self.initial_entity_capacity.is_none()
            && self.initial_relation_capacity.is_none()
            && self.mvcc.is_none()
            && self.storage_layout.is_none()
            && self.publication.is_none()
            && self.payload_policy.is_none()
            && self.symbol_policy.is_none()
            && self.durable_log_policy.is_none()
            && self.adjacency_policy.is_none()
            && self.cross_context_policy.is_none()
            && self.cascade_delete_policy.is_none()
            && self.compiled_lane_policy.is_none()
    }
}

impl Default for RelationalConfigOverride {
    fn default() -> Self {
        Self {
            runtime_name: None,
            initial_entity_capacity: None,
            initial_relation_capacity: None,
            mvcc: None,
            storage_layout: None,
            publication: None,
            payload_policy: None,
            symbol_policy: None,
            durable_log_policy: None,
            adjacency_policy: None,
            cross_context_policy: None,
            cascade_delete_policy: None,
            compiled_lane_policy: None,
        }
    }
}
