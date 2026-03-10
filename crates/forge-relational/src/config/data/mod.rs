use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::diagnostics::data::RelationalDiagnosticsProfile;
use crate::durability::data::DurabilityMode;
use crate::history::data::{BranchId, HistoryRetentionClass, VersionGraphPolicy};
use crate::logic::commit::CommitAuthorityContract;
use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
use crate::payloads::data::PayloadPolicy;
use crate::schema::data::RelationalSchemaRegistry;
use crate::symbols::data::SymbolPolicy;
use crate::symbols::data::{StringInterner, SymbolTableSnapshot};
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
    pub durable_log_policy: DurableLogPolicy,
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

impl RelationalRuntimeConfig {
    pub fn resolved(
        profile: RelationalRuntimeProfile,
        config_override: RelationalConfigOverride,
    ) -> Self {
        let mut config = match profile {
            RelationalRuntimeProfile::CertificationCore => Self {
                profile,
                runtime_name: "forge-relational".to_string(),
                execution_model: RelationalExecutionModel::SerialAuthority,
                planning: PlanningContract::default(),
                commit_authority: CommitAuthorityContract::default(),
                diagnostics: RelationalDiagnosticsProfile {
                    detailed_traces_enabled: true,
                    max_entries_per_artifact: 512,
                    ..RelationalDiagnosticsProfile::default()
                },
                version_graph_policy: VersionGraphPolicy::CanonicalSerializedPublication,
                history_retention: HistoryRetentionClass::AuditGrade,
                main_branch: BranchId("main".to_string()),
                schema_registry: RelationalSchemaRegistry::default(),
                invariant_catalog: InvariantCatalog::default(),
                mvcc: MvccConfig {
                    track_visibility_metadata: true,
                    snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
                    auto_reclaim_deleted_records: false,
                    reclaim_batch_size: 128,
                    retention_backend: RetentionBackend::PinTrackedRetention,
                },
                retention_policy: RetentionPolicy {
                    backend: RetentionBackend::PinTrackedRetention,
                    reclaim_batch_size: 128,
                },
                storage_layout: StorageLayoutConfig {
                    entity_chunk_size: 1024,
                    relation_chunk_size: 1024,
                    scan_packet_size: 512,
                },
                payload_policy: PayloadPolicy::default(),
                symbol_policy: SymbolPolicy::PreferInterned,
                durable_log_policy: DurableLogPolicy {
                    retention_mode: DurableLogRetentionMode::RetainAllInMemory,
                    max_in_memory_envelopes: 4_096,
                    compact_after_checkpoint: false,
                },
                adjacency_policy: AdjacencyPolicy {
                    backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                    small_degree_inline_capacity: 4,
                },
                cross_context_policy: CrossContextPolicy::SchemaControlled,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                publication: PublicationConfig {
                    coherent_publication_required: true,
                    max_patch_records_per_commit: 4096,
                    patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
                },
                compiled_lane_policy: CompiledLanePolicy::Disabled,
                durability_mode: DurabilityMode::InMemoryCanonical,
                config_override: RelationalConfigOverride::default(),
                config_provenance: ConfigProvenance {
                    profile,
                    entries: Default::default(),
                },
                initial_entity_capacity: 64,
                initial_relation_capacity: 64,
                symbol_table: StringInterner::default().snapshot(),
            },
            RelationalRuntimeProfile::GeometryKernel => Self {
                profile,
                runtime_name: "forge-relational-geometry".to_string(),
                execution_model: RelationalExecutionModel::SerialAuthority,
                planning: PlanningContract::default(),
                commit_authority: CommitAuthorityContract::default(),
                diagnostics: RelationalDiagnosticsProfile {
                    detailed_traces_enabled: true,
                    max_entries_per_artifact: 768,
                    ..RelationalDiagnosticsProfile::default()
                },
                version_graph_policy: VersionGraphPolicy::CanonicalSerializedPublication,
                history_retention: HistoryRetentionClass::AuditGrade,
                main_branch: BranchId("main".to_string()),
                schema_registry: RelationalSchemaRegistry::default(),
                invariant_catalog: InvariantCatalog::default(),
                mvcc: MvccConfig {
                    track_visibility_metadata: true,
                    snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
                    auto_reclaim_deleted_records: false,
                    reclaim_batch_size: 256,
                    retention_backend: RetentionBackend::PinTrackedRetention,
                },
                retention_policy: RetentionPolicy {
                    backend: RetentionBackend::PinTrackedRetention,
                    reclaim_batch_size: 256,
                },
                storage_layout: StorageLayoutConfig {
                    entity_chunk_size: 2048,
                    relation_chunk_size: 2048,
                    scan_packet_size: 1024,
                },
                payload_policy: PayloadPolicy {
                    default_class: crate::payloads::data::PayloadClass::OpaqueBytes,
                    allow_opaque_bytes: true,
                },
                symbol_policy: SymbolPolicy::PreferInterned,
                durable_log_policy: DurableLogPolicy {
                    retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
                    max_in_memory_envelopes: 2_048,
                    compact_after_checkpoint: true,
                },
                adjacency_policy: AdjacencyPolicy {
                    backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                    small_degree_inline_capacity: 8,
                },
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                publication: PublicationConfig {
                    coherent_publication_required: true,
                    max_patch_records_per_commit: 8192,
                    patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
                },
                compiled_lane_policy: CompiledLanePolicy::Disabled,
                durability_mode: DurabilityMode::InMemoryCanonical,
                config_override: RelationalConfigOverride::default(),
                config_provenance: ConfigProvenance {
                    profile,
                    entries: Default::default(),
                },
                initial_entity_capacity: 256,
                initial_relation_capacity: 256,
                symbol_table: StringInterner::default().snapshot(),
            },
            RelationalRuntimeProfile::ChipSimulation => Self {
                profile,
                runtime_name: "forge-relational-chip".to_string(),
                execution_model: RelationalExecutionModel::SerialAuthority,
                planning: PlanningContract::default(),
                commit_authority: CommitAuthorityContract::default(),
                diagnostics: RelationalDiagnosticsProfile {
                    detailed_traces_enabled: false,
                    max_entries_per_artifact: 384,
                    ..RelationalDiagnosticsProfile::default()
                },
                version_graph_policy: VersionGraphPolicy::CanonicalSerializedPublication,
                history_retention: HistoryRetentionClass::AuditGrade,
                main_branch: BranchId("main".to_string()),
                schema_registry: RelationalSchemaRegistry::default(),
                invariant_catalog: InvariantCatalog::default(),
                mvcc: MvccConfig {
                    track_visibility_metadata: true,
                    snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
                    auto_reclaim_deleted_records: false,
                    reclaim_batch_size: 512,
                    retention_backend: RetentionBackend::EpochChunkRetention,
                },
                retention_policy: RetentionPolicy {
                    backend: RetentionBackend::EpochChunkRetention,
                    reclaim_batch_size: 512,
                },
                storage_layout: StorageLayoutConfig {
                    entity_chunk_size: 4096,
                    relation_chunk_size: 4096,
                    scan_packet_size: 2048,
                },
                payload_policy: PayloadPolicy {
                    default_class: crate::payloads::data::PayloadClass::OpaqueBytes,
                    allow_opaque_bytes: true,
                },
                symbol_policy: SymbolPolicy::RequireInterned,
                durable_log_policy: DurableLogPolicy {
                    retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
                    max_in_memory_envelopes: 1_024,
                    compact_after_checkpoint: true,
                },
                adjacency_policy: AdjacencyPolicy {
                    backend: AdjacencyBackend::CompressedFanoutAdjacency,
                    small_degree_inline_capacity: 8,
                },
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
                publication: PublicationConfig {
                    coherent_publication_required: true,
                    max_patch_records_per_commit: 16384,
                    patch_surface_policy: PatchSurfacePolicy::DensePatchSurface,
                },
                compiled_lane_policy: CompiledLanePolicy::DerivedCompiledLane,
                durability_mode: DurabilityMode::InMemoryCanonical,
                config_override: RelationalConfigOverride::default(),
                config_provenance: ConfigProvenance {
                    profile,
                    entries: Default::default(),
                },
                initial_entity_capacity: 512,
                initial_relation_capacity: 512,
                symbol_table: StringInterner::default().snapshot(),
            },
            RelationalRuntimeProfile::AiWorkflow => Self {
                profile,
                runtime_name: "forge-relational-ai".to_string(),
                execution_model: RelationalExecutionModel::SerialAuthority,
                planning: PlanningContract::default(),
                commit_authority: CommitAuthorityContract::default(),
                diagnostics: RelationalDiagnosticsProfile {
                    detailed_traces_enabled: false,
                    max_entries_per_artifact: 256,
                    ..RelationalDiagnosticsProfile::default()
                },
                version_graph_policy: VersionGraphPolicy::CanonicalSerializedPublication,
                history_retention: HistoryRetentionClass::Durable,
                main_branch: BranchId("main".to_string()),
                schema_registry: RelationalSchemaRegistry::default(),
                invariant_catalog: InvariantCatalog::default(),
                mvcc: MvccConfig {
                    track_visibility_metadata: true,
                    snapshot_release_policy: SnapshotReleasePolicy::ReleaseOnRetentionPass,
                    auto_reclaim_deleted_records: true,
                    reclaim_batch_size: 512,
                    retention_backend: RetentionBackend::PinTrackedRetention,
                },
                retention_policy: RetentionPolicy {
                    backend: RetentionBackend::PinTrackedRetention,
                    reclaim_batch_size: 512,
                },
                storage_layout: StorageLayoutConfig {
                    entity_chunk_size: 2048,
                    relation_chunk_size: 1024,
                    scan_packet_size: 1024,
                },
                payload_policy: PayloadPolicy {
                    default_class: crate::payloads::data::PayloadClass::StructuredJson,
                    allow_opaque_bytes: true,
                },
                symbol_policy: SymbolPolicy::PreferInterned,
                durable_log_policy: DurableLogPolicy {
                    retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
                    max_in_memory_envelopes: 1_024,
                    compact_after_checkpoint: true,
                },
                adjacency_policy: AdjacencyPolicy {
                    backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                    small_degree_inline_capacity: 4,
                },
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                publication: PublicationConfig {
                    coherent_publication_required: true,
                    max_patch_records_per_commit: 8192,
                    patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
                },
                compiled_lane_policy: CompiledLanePolicy::Disabled,
                durability_mode: DurabilityMode::InMemoryCanonical,
                config_override: RelationalConfigOverride::default(),
                config_provenance: ConfigProvenance {
                    profile,
                    entries: Default::default(),
                },
                initial_entity_capacity: 128,
                initial_relation_capacity: 128,
                symbol_table: StringInterner::default().snapshot(),
            },
        };

        let mut provenance_entries = BTreeMap::new();
        provenance_entries.insert(
            "runtime_name".to_string(),
            provenance_entry(config_override.runtime_name.is_some()),
        );
        provenance_entries.insert(
            "initial_entity_capacity".to_string(),
            provenance_entry(config_override.initial_entity_capacity.is_some()),
        );
        provenance_entries.insert(
            "initial_relation_capacity".to_string(),
            provenance_entry(config_override.initial_relation_capacity.is_some()),
        );
        provenance_entries.insert(
            "mvcc".to_string(),
            provenance_entry(config_override.mvcc.is_some()),
        );
        provenance_entries.insert(
            "storage_layout".to_string(),
            provenance_entry(config_override.storage_layout.is_some()),
        );
        provenance_entries.insert(
            "publication".to_string(),
            provenance_entry(config_override.publication.is_some()),
        );
        provenance_entries.insert(
            "payload_policy".to_string(),
            provenance_entry(config_override.payload_policy.is_some()),
        );
        provenance_entries.insert(
            "symbol_policy".to_string(),
            provenance_entry(config_override.symbol_policy.is_some()),
        );
        provenance_entries.insert(
            "durable_log_policy".to_string(),
            provenance_entry(config_override.durable_log_policy.is_some()),
        );
        provenance_entries.insert(
            "adjacency_policy".to_string(),
            provenance_entry(config_override.adjacency_policy.is_some()),
        );
        provenance_entries.insert(
            "cross_context_policy".to_string(),
            provenance_entry(config_override.cross_context_policy.is_some()),
        );
        provenance_entries.insert(
            "cascade_delete_policy".to_string(),
            provenance_entry(config_override.cascade_delete_policy.is_some()),
        );
        provenance_entries.insert(
            "compiled_lane_policy".to_string(),
            provenance_entry(config_override.compiled_lane_policy.is_some()),
        );

        if let Some(runtime_name) = &config_override.runtime_name {
            config.runtime_name = runtime_name.clone();
        }
        if let Some(capacity) = config_override.initial_entity_capacity {
            config.initial_entity_capacity = capacity;
        }
        if let Some(capacity) = config_override.initial_relation_capacity {
            config.initial_relation_capacity = capacity;
        }
        if let Some(mvcc) = &config_override.mvcc {
            config.mvcc = mvcc.clone();
            config.retention_policy.backend = mvcc.retention_backend;
            config.retention_policy.reclaim_batch_size = mvcc.reclaim_batch_size;
        }
        if let Some(storage_layout) = &config_override.storage_layout {
            config.storage_layout = storage_layout.clone();
        }
        if let Some(publication) = &config_override.publication {
            config.publication = publication.clone();
        }
        if let Some(payload_policy) = &config_override.payload_policy {
            config.payload_policy = payload_policy.clone();
        }
        if let Some(symbol_policy) = &config_override.symbol_policy {
            config.symbol_policy = *symbol_policy;
        }
        if let Some(durable_log_policy) = &config_override.durable_log_policy {
            config.durable_log_policy = durable_log_policy.clone();
        }
        if let Some(adjacency_policy) = &config_override.adjacency_policy {
            config.adjacency_policy = adjacency_policy.clone();
        }
        if let Some(cross_context_policy) = &config_override.cross_context_policy {
            config.cross_context_policy = *cross_context_policy;
        }
        if let Some(cascade_delete_policy) = &config_override.cascade_delete_policy {
            config.cascade_delete_policy = *cascade_delete_policy;
        }
        if let Some(compiled_lane_policy) = &config_override.compiled_lane_policy {
            config.compiled_lane_policy = *compiled_lane_policy;
        }

        config.config_override = config_override;
        config.config_provenance = ConfigProvenance {
            profile,
            entries: provenance_entries,
        };
        config
    }
}

fn provenance_entry(overridden: bool) -> ConfigProvenanceEntry {
    if overridden {
        ConfigProvenanceEntry {
            source: ConfigValueSource::BuilderOverride,
            detail: "explicit builder override".to_string(),
        }
    } else {
        ConfigProvenanceEntry {
            source: ConfigValueSource::ProfileDefault,
            detail: "resolved from runtime profile".to_string(),
        }
    }
}
