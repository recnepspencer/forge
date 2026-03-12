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

pub(super) fn default_profile_config(
    profile: RelationalRuntimeProfile,
) -> RelationalRuntimeConfig {
    let base = |runtime_name: &str,
                diagnostics: RelationalDiagnosticsProfile,
                history_retention: HistoryRetentionClass,
                mvcc: MvccConfig,
                retention_policy: RetentionPolicy,
                storage_layout: StorageLayoutConfig,
                payload_policy: PayloadPolicy,
                symbol_policy: SymbolPolicy,
                visibility_cache_policy: VisibilityCachePolicy,
                durability: DurabilityPolicy,
                adjacency_policy: AdjacencyPolicy,
                cross_context_policy: CrossContextPolicy,
                cascade_delete_policy: CascadeDeletePolicy,
                publication: PublicationConfig,
                compiled_lane_policy: CompiledLanePolicy,
                initial_entity_capacity: usize,
                initial_relation_capacity: usize| RelationalRuntimeConfig {
        profile,
        execution: ExecutionConfig {
            runtime_name: runtime_name.to_string(),
            execution_model: RelationalExecutionModel::SerialAuthority,
            planning: PlanningContract::default(),
            commit_authority: CommitAuthorityContract::default(),
            compiled_lane_policy,
        },
        diagnostics: DiagnosticsConfig { profile: diagnostics },
        history: HistoryConfig {
            version_graph_policy: VersionGraphPolicy::CanonicalSerializedPublication,
            retention: history_retention,
            main_branch: BranchId("main".to_string()),
        },
        schema: SchemaConfig {
            registry: RelationalSchemaRegistry::default(),
            invariant_catalog: InvariantCatalog::default(),
        },
        identity: IdentityConfig {
            symbol_policy,
            symbol_table: StringInterner::default().snapshot(),
        },
        storage: StorageConfig {
            initial_entity_capacity,
            initial_relation_capacity,
            mvcc,
            retention: retention_policy,
            layout: storage_layout,
            payload_policy,
            adjacency_policy,
            cross_context_policy,
            cascade_delete_policy,
        },
        visibility: VisibilityConfig {
            cache_policy: visibility_cache_policy,
        },
        publication: PublicationSection { policy: publication },
        durability: DurabilityConfig { policy: durability },
        config_override: RelationalConfigOverride::default(),
        config_provenance: ConfigProvenance {
            profile,
            entries: Default::default(),
        },
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
            DurabilityPolicy {
                mode: DurabilityMode::InMemoryCanonical,
                log: DurableLogPolicy {
                    retention_mode: DurableLogRetentionMode::RetainAllInMemory,
                    max_in_memory_envelopes: 4_096,
                    compact_after_checkpoint: false,
                },
                checkpoints: CheckpointPolicy {
                    compact_after_checkpoint: false,
                },
                store_layout: None,
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
            DurabilityPolicy {
                mode: DurabilityMode::InMemoryCanonical,
                log: DurableLogPolicy {
                    retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
                    max_in_memory_envelopes: 2_048,
                    compact_after_checkpoint: true,
                },
                checkpoints: CheckpointPolicy {
                    compact_after_checkpoint: true,
                },
                store_layout: None,
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
            DurabilityPolicy {
                mode: DurabilityMode::InMemoryCanonical,
                log: DurableLogPolicy {
                    retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
                    max_in_memory_envelopes: 1_024,
                    compact_after_checkpoint: true,
                },
                checkpoints: CheckpointPolicy {
                    compact_after_checkpoint: true,
                },
                store_layout: None,
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
            DurabilityPolicy {
                mode: DurabilityMode::InMemoryCanonical,
                log: DurableLogPolicy {
                    retention_mode: DurableLogRetentionMode::CompactAfterCheckpoint,
                    max_in_memory_envelopes: 1_024,
                    compact_after_checkpoint: true,
                },
                checkpoints: CheckpointPolicy {
                    compact_after_checkpoint: true,
                },
                store_layout: None,
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
