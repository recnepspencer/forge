use serde::{Deserialize, Serialize};

use crate::durability::data::{DurabilityMode, DurableStoreLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RelationalRuntimeProfile {
    CertificationCore,
    GeometryKernel,
    ChipSimulation,
    AiWorkflow,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisibilityCachePolicy {
    pub enabled: bool,
    pub protect_branch_heads: bool,
    pub protect_replay_retained: bool,
    pub protect_active_snapshots: bool,
    pub recent_version_window: usize,
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
pub struct CheckpointPolicy {
    pub compact_after_checkpoint: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurabilityPolicy {
    pub mode: DurabilityMode,
    pub log: DurableLogPolicy,
    pub checkpoints: CheckpointPolicy,
    pub store_layout: Option<DurableStoreLayout>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MutationConfig {
    pub(crate) patch_surface_policy: PatchSurfacePolicy,
    pub(crate) cascade_delete_policy: CascadeDeletePolicy,
    pub(crate) adjacency_policy: AdjacencyPolicy,
    pub(crate) cross_context_policy: CrossContextPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationConfig {
    pub coherent_publication_required: bool,
    pub max_patch_records_per_commit: usize,
    pub max_published_snapshot_handles: usize,
    pub patch_surface_policy: PatchSurfacePolicy,
}
