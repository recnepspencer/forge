mod profiles;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::diagnostics::data::RelationalDiagnosticsProfile;
use crate::durability::data::{DurabilityMode, DurableStoreLayout};
use crate::history::data::{BranchId, HistoryRetentionClass, VersionGraphPolicy};
use crate::logic::commit::CommitAuthorityContract;
use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
use crate::payloads::data::PayloadPolicy;
use crate::schema::data::RelationalSchemaRegistry;
use crate::symbols::data::SymbolPolicy;
use crate::symbols::data::SymbolTableSnapshot;
use crate::validation::data::InvariantCatalog;

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
    pub visibility_cache_policy: Option<VisibilityCachePolicy>,
    pub durable_log_policy: Option<DurableLogPolicy>,
    pub durable_store_layout: Option<DurableStoreLayout>,
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
            && self.visibility_cache_policy.is_none()
            && self.durable_log_policy.is_none()
            && self.durable_store_layout.is_none()
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
            visibility_cache_policy: None,
            durable_log_policy: None,
            durable_store_layout: None,
            adjacency_policy: None,
            cross_context_policy: None,
            cascade_delete_policy: None,
            compiled_lane_policy: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalRuntimeConfig {
    pub profile: RelationalRuntimeProfile,
    pub runtime_name: String,
    pub execution_model: RelationalExecutionModel,
    pub planning: PlanningContract,
    pub commit_authority: CommitAuthorityContract,
    pub diagnostics: RelationalDiagnosticsProfile,
    pub version_graph_policy: VersionGraphPolicy,
    pub history_retention: HistoryRetentionClass,
    pub main_branch: BranchId,
    pub schema_registry: RelationalSchemaRegistry,
    pub invariant_catalog: InvariantCatalog,
    pub mvcc: MvccConfig,
    pub retention_policy: RetentionPolicy,
    pub storage_layout: StorageLayoutConfig,
    pub payload_policy: PayloadPolicy,
    pub symbol_policy: SymbolPolicy,
    pub visibility_cache_policy: VisibilityCachePolicy,
    pub durable_log_policy: DurableLogPolicy,
    pub durable_store_layout: Option<DurableStoreLayout>,
    pub adjacency_policy: AdjacencyPolicy,
    pub cross_context_policy: CrossContextPolicy,
    pub cascade_delete_policy: CascadeDeletePolicy,
    pub publication: PublicationConfig,
    pub compiled_lane_policy: CompiledLanePolicy,
    pub durability_mode: DurabilityMode,
    pub config_override: RelationalConfigOverride,
    pub config_provenance: ConfigProvenance,
    pub initial_entity_capacity: usize,
    pub initial_relation_capacity: usize,
    pub symbol_table: SymbolTableSnapshot,
}

impl Default for RelationalRuntimeConfig {
    fn default() -> Self {
        Self::resolved(
            RelationalRuntimeProfile::CertificationCore,
            RelationalConfigOverride::default(),
        )
    }
}

impl RelationalRuntimeConfig {}
