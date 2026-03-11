use std::collections::BTreeMap;

use crate::config::data::*;
use crate::diagnostics::data::RelationalDiagnosticsProfile;
use crate::durability::data::DurabilityMode;
use crate::history::data::{BranchId, HistoryRetentionClass, VersionGraphPolicy};
use crate::logic::commit::CommitAuthorityContract;
use crate::logic::planning::{PlanningContract, RelationalExecutionModel};
use crate::payloads::data::{PayloadClass, PayloadPolicy};
use crate::schema::data::RelationalSchemaRegistry;
use crate::symbols::data::{StringInterner, SymbolPolicy};
use crate::validation::data::InvariantCatalog;

impl RelationalRuntimeConfig {
    pub fn resolved(
        profile: RelationalRuntimeProfile,
        config_override: RelationalConfigOverride,
    ) -> Self {
        let mut config = default_profile_config(profile);
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
            "visibility_cache_policy".to_string(),
            provenance_entry(config_override.visibility_cache_policy.is_some()),
        );
        provenance_entries.insert(
            "durable_log_policy".to_string(),
            provenance_entry(config_override.durable_log_policy.is_some()),
        );
        provenance_entries.insert(
            "durable_store_layout".to_string(),
            provenance_entry(config_override.durable_store_layout.is_some()),
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
        if let Some(visibility_cache_policy) = &config_override.visibility_cache_policy {
            config.visibility_cache_policy = visibility_cache_policy.clone();
        }
        if let Some(durable_log_policy) = &config_override.durable_log_policy {
            config.durable_log_policy = durable_log_policy.clone();
        }
        if let Some(durable_store_layout) = &config_override.durable_store_layout {
            config.durable_store_layout = Some(durable_store_layout.clone());
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

fn default_profile_config(profile: RelationalRuntimeProfile) -> RelationalRuntimeConfig {
    let base = |runtime_name: &str,
                diagnostics: RelationalDiagnosticsProfile,
                history_retention: HistoryRetentionClass,
                mvcc: MvccConfig,
                retention_policy: RetentionPolicy,
                storage_layout: StorageLayoutConfig,
                payload_policy: PayloadPolicy,
                symbol_policy: SymbolPolicy,
                visibility_cache_policy: VisibilityCachePolicy,
                durable_log_policy: DurableLogPolicy,
                adjacency_policy: AdjacencyPolicy,
                cross_context_policy: CrossContextPolicy,
                cascade_delete_policy: CascadeDeletePolicy,
                publication: PublicationConfig,
                compiled_lane_policy: CompiledLanePolicy,
                initial_entity_capacity: usize,
                initial_relation_capacity: usize| RelationalRuntimeConfig {
        profile,
        runtime_name: runtime_name.to_string(),
        execution_model: RelationalExecutionModel::SerialAuthority,
        planning: PlanningContract::default(),
        commit_authority: CommitAuthorityContract::default(),
        diagnostics,
        version_graph_policy: VersionGraphPolicy::CanonicalSerializedPublication,
        history_retention,
        main_branch: BranchId("main".to_string()),
        schema_registry: RelationalSchemaRegistry::default(),
        invariant_catalog: InvariantCatalog::default(),
        mvcc,
        retention_policy,
        storage_layout,
        payload_policy,
        symbol_policy,
        visibility_cache_policy,
        durable_log_policy,
        durable_store_layout: None,
        adjacency_policy,
        cross_context_policy,
        cascade_delete_policy,
        publication,
        compiled_lane_policy,
        durability_mode: DurabilityMode::InMemoryCanonical,
        config_override: RelationalConfigOverride::default(),
        config_provenance: ConfigProvenance {
            profile,
            entries: Default::default(),
        },
        initial_entity_capacity,
        initial_relation_capacity,
        symbol_table: StringInterner::default().snapshot(),
    };

    match profile {
        RelationalRuntimeProfile::CertificationCore => base(
            "forge-relational",
            RelationalDiagnosticsProfile {
                detailed_traces_enabled: true,
                max_entries_per_artifact: 512,
                ..RelationalDiagnosticsProfile::default()
            },
            HistoryRetentionClass::AuditGrade,
            MvccConfig {
                track_visibility_metadata: true,
                snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
                auto_reclaim_deleted_records: false,
                reclaim_batch_size: 128,
                retention_backend: RetentionBackend::PinTrackedRetention,
            },
            RetentionPolicy {
                backend: RetentionBackend::PinTrackedRetention,
                reclaim_batch_size: 128,
            },
            StorageLayoutConfig {
                entity_chunk_size: 1024,
                relation_chunk_size: 1024,
                scan_packet_size: 512,
            },
            PayloadPolicy::default(),
            SymbolPolicy::PreferInterned,
            VisibilityCachePolicy {
                enabled: true,
                protect_branch_heads: true,
                protect_replay_retained: true,
                protect_active_snapshots: true,
                recent_version_window: 32,
            },
            DurableLogPolicy {
                retention_mode: DurableLogRetentionMode::RetainAllInMemory,
                max_in_memory_envelopes: 4_096,
                compact_after_checkpoint: false,
            },
            AdjacencyPolicy {
                backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                small_degree_inline_capacity: 4,
            },
            CrossContextPolicy::SchemaControlled,
            CascadeDeletePolicy::CascadeDeleteRelations,
            PublicationConfig {
                coherent_publication_required: true,
                max_patch_records_per_commit: 4096,
                max_published_snapshot_handles: 256,
                patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
            },
            CompiledLanePolicy::Disabled,
            64,
            64,
        ),
        RelationalRuntimeProfile::GeometryKernel => base(
            "forge-relational-geometry",
            RelationalDiagnosticsProfile {
                detailed_traces_enabled: true,
                max_entries_per_artifact: 768,
                ..RelationalDiagnosticsProfile::default()
            },
            HistoryRetentionClass::AuditGrade,
            MvccConfig {
                track_visibility_metadata: true,
                snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
                auto_reclaim_deleted_records: false,
                reclaim_batch_size: 256,
                retention_backend: RetentionBackend::PinTrackedRetention,
            },
            RetentionPolicy {
                backend: RetentionBackend::PinTrackedRetention,
                reclaim_batch_size: 256,
            },
            StorageLayoutConfig {
                entity_chunk_size: 2048,
                relation_chunk_size: 2048,
                scan_packet_size: 1024,
            },
            PayloadPolicy {
                default_class: PayloadClass::OpaqueBytes,
                allow_opaque_bytes: true,
            },
            SymbolPolicy::PreferInterned,
            VisibilityCachePolicy {
                enabled: true,
                protect_branch_heads: true,
                protect_replay_retained: true,
                protect_active_snapshots: true,
                recent_version_window: 2,
            },
            DurableLogPolicy {
                retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
                max_in_memory_envelopes: 2_048,
                compact_after_checkpoint: true,
            },
            AdjacencyPolicy {
                backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                small_degree_inline_capacity: 8,
            },
            CrossContextPolicy::AllowExplicit,
            CascadeDeletePolicy::CascadeDeleteRelations,
            PublicationConfig {
                coherent_publication_required: true,
                max_patch_records_per_commit: 8192,
                max_published_snapshot_handles: 64,
                patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
            },
            CompiledLanePolicy::Disabled,
            256,
            256,
        ),
        RelationalRuntimeProfile::ChipSimulation => base(
            "forge-relational-chip",
            RelationalDiagnosticsProfile {
                detailed_traces_enabled: false,
                max_entries_per_artifact: 384,
                ..RelationalDiagnosticsProfile::default()
            },
            HistoryRetentionClass::AuditGrade,
            MvccConfig {
                track_visibility_metadata: true,
                snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
                auto_reclaim_deleted_records: false,
                reclaim_batch_size: 512,
                retention_backend: RetentionBackend::EpochChunkRetention,
            },
            RetentionPolicy {
                backend: RetentionBackend::EpochChunkRetention,
                reclaim_batch_size: 512,
            },
            StorageLayoutConfig {
                entity_chunk_size: 4096,
                relation_chunk_size: 4096,
                scan_packet_size: 2048,
            },
            PayloadPolicy {
                default_class: PayloadClass::OpaqueBytes,
                allow_opaque_bytes: true,
            },
            SymbolPolicy::RequireInterned,
            VisibilityCachePolicy {
                enabled: true,
                protect_branch_heads: true,
                protect_replay_retained: true,
                protect_active_snapshots: true,
                recent_version_window: 2,
            },
            DurableLogPolicy {
                retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
                max_in_memory_envelopes: 1_024,
                compact_after_checkpoint: true,
            },
            AdjacencyPolicy {
                backend: AdjacencyBackend::CompressedFanoutAdjacency,
                small_degree_inline_capacity: 8,
            },
            CrossContextPolicy::AllowExplicit,
            CascadeDeletePolicy::RetainDanglingForAudit,
            PublicationConfig {
                coherent_publication_required: true,
                max_patch_records_per_commit: 16384,
                max_published_snapshot_handles: 64,
                patch_surface_policy: PatchSurfacePolicy::DensePatchSurface,
            },
            CompiledLanePolicy::DerivedCompiledLane,
            512,
            512,
        ),
        RelationalRuntimeProfile::AiWorkflow => base(
            "forge-relational-ai",
            RelationalDiagnosticsProfile {
                detailed_traces_enabled: false,
                max_entries_per_artifact: 256,
                ..RelationalDiagnosticsProfile::default()
            },
            HistoryRetentionClass::Durable,
            MvccConfig {
                track_visibility_metadata: true,
                snapshot_release_policy: SnapshotReleasePolicy::ReleaseOnRetentionPass,
                auto_reclaim_deleted_records: true,
                reclaim_batch_size: 512,
                retention_backend: RetentionBackend::PinTrackedRetention,
            },
            RetentionPolicy {
                backend: RetentionBackend::PinTrackedRetention,
                reclaim_batch_size: 512,
            },
            StorageLayoutConfig {
                entity_chunk_size: 2048,
                relation_chunk_size: 1024,
                scan_packet_size: 1024,
            },
            PayloadPolicy {
                default_class: PayloadClass::StructuredJson,
                allow_opaque_bytes: true,
            },
            SymbolPolicy::PreferInterned,
            VisibilityCachePolicy {
                enabled: true,
                protect_branch_heads: true,
                protect_replay_retained: true,
                protect_active_snapshots: true,
                recent_version_window: 16,
            },
            DurableLogPolicy {
                retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
                max_in_memory_envelopes: 1_024,
                compact_after_checkpoint: true,
            },
            AdjacencyPolicy {
                backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
                small_degree_inline_capacity: 4,
            },
            CrossContextPolicy::AllowExplicit,
            CascadeDeletePolicy::CascadeDeleteRelations,
            PublicationConfig {
                coherent_publication_required: true,
                max_patch_records_per_commit: 8192,
                max_published_snapshot_handles: 128,
                patch_surface_policy: PatchSurfacePolicy::StructuredPatchSurface,
            },
            CompiledLanePolicy::Disabled,
            128,
            128,
        ),
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
