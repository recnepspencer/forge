use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::config::{
    ConfigProvenance, ConfigProvenanceEntry, ConfigValueSource, MvccConfig, PublicationConfig,
    RelationalConfigOverride, RelationalRuntimeProfile, SnapshotReleasePolicy, StorageLayoutConfig,
};
use crate::data::diagnostics::{
    DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticArtifact,
};
use crate::data::durability::DurabilityMode;
use crate::data::history::{BranchId, HistoryRetentionClass, VersionGraphPolicy};
use crate::data::identity::{EntityId, RelationId, VersionId};
use crate::data::schema::{KindResolution, RelationalSchemaRegistry};
use crate::data::snapshot::SnapshotHandle;
use crate::logic::commit::CommitAuthorityContract;
use crate::logic::planning::{PlanningContract, RelationalExecutionModel};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordLifecycleState {
    Live,
    DeletedRetained,
    PinnedBySnapshot,
    PinnedByBranch,
    PinnedByReplayRetention,
    Reclaimable,
    Reusable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantClass {
    AlwaysOnStructural,
    CommitBoundary,
    SnapshotAudit,
    HarnessHeavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantExecutionPoint {
    MutationSensitive,
    CommitBoundary,
    SnapshotPublication,
    HarnessAudit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantFailureEffect {
    BlockCommit,
    BlockPublication,
    AuditOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantViolation {
    pub class: InvariantClass,
    pub code: crate::data::diagnostics::DiagnosticCode,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageInvariantReport {
    pub violations: Vec<InvariantViolation>,
}

impl StorageInvariantReport {
    pub fn is_clean(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn for_class(&self, class: InvariantClass) -> Vec<&InvariantViolation> {
        self.violations
            .iter()
            .filter(|violation| violation.class == class)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCheckResult {
    pub class: InvariantClass,
    pub execution_point: InvariantExecutionPoint,
    pub failure_effect: InvariantFailureEffect,
    pub violations: Vec<InvariantViolation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantRule {
    LiveEntityRequiresKind,
    LiveRelationRequiresEndpoints,
    MaxMergedIntents(usize),
    MaxSnapshotEntities(usize),
    UniqueEntityPayloadField(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantCatalog {
    pub always_on_structural: Vec<InvariantRule>,
    pub commit_boundary: Vec<InvariantRule>,
    pub snapshot_audit: Vec<InvariantRule>,
    pub harness_heavy: Vec<InvariantRule>,
}

impl Default for InvariantCatalog {
    fn default() -> Self {
        Self {
            always_on_structural: vec![
                InvariantRule::LiveEntityRequiresKind,
                InvariantRule::LiveRelationRequiresEndpoints,
            ],
            commit_boundary: Vec::new(),
            snapshot_audit: Vec::new(),
            harness_heavy: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityReadRecord {
    pub entity_id: EntityId,
    pub kind: KindResolution,
    pub lifecycle: RecordLifecycleState,
    pub created_at_version: VersionId,
    pub retired_at_version: Option<VersionId>,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationReadRecord {
    pub relation_id: RelationId,
    pub kind: KindResolution,
    pub lifecycle: RecordLifecycleState,
    pub created_at_version: VersionId,
    pub retired_at_version: Option<VersionId>,
    pub source: EntityId,
    pub target: EntityId,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySchemaVersion(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReplayRecord {
    pub schema_version: ReplaySchemaVersion,
    pub commit_id: crate::data::history::CommitId,
    pub version_id: VersionId,
    pub snapshot_id: crate::data::snapshot::SnapshotId,
    pub patch: crate::data::diff::RelationalPatchRecord,
    pub schema_registry: RelationalSchemaRegistry,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalDiagnosticsFacade {
    pub artifacts: Vec<RelationalDiagnosticArtifact>,
}

impl RelationalDiagnosticsFacade {
    pub fn artifacts(&self) -> &[RelationalDiagnosticArtifact] {
        &self.artifacts
    }

    pub fn by_scope(&self, scope: DiagnosticsScope) -> Vec<&RelationalDiagnosticArtifact> {
        self.artifacts
            .iter()
            .filter(|artifact| artifact.scope == scope)
            .collect()
    }

    pub fn minimal_summaries(&self) -> Vec<&RelationalDiagnosticArtifact> {
        self.artifacts
            .iter()
            .filter(|artifact| artifact.kind == DiagnosticsArtifactKind::MinimalSummary)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalReadView {
    pub(super) snapshot: SnapshotHandle,
    pub(super) entities: Vec<EntityReadRecord>,
    pub(super) relations: Vec<RelationReadRecord>,
}

impl RelationalReadView {
    pub fn snapshot(&self) -> &SnapshotHandle {
        &self.snapshot
    }

    pub fn entities(&self) -> &[EntityReadRecord] {
        &self.entities
    }

    pub fn relations(&self) -> &[RelationReadRecord] {
        &self.relations
    }

    pub fn execute_packet(&self, packet: &crate::data::query::QueryWorkPacket) -> PacketResult {
        let mut entities = Vec::new();
        let mut relations = Vec::new();
        for target in &packet.targets {
            match target {
                crate::data::query::ReadTarget::Entity(entity_id) => {
                    if let Some(record) = self
                        .entities
                        .iter()
                        .find(|record| &record.entity_id == entity_id)
                    {
                        entities.push(record.clone());
                    }
                }
                crate::data::query::ReadTarget::Relation(relation_id) => {
                    if let Some(record) = self
                        .relations
                        .iter()
                        .find(|record| &record.relation_id == relation_id)
                    {
                        relations.push(record.clone());
                    }
                }
            }
        }
        PacketResult {
            execution_shape: crate::data::query::QueryExecutionShape::BulkPacketized,
            entities,
            relations,
        }
    }

    pub fn get_entity(&self, entity_id: EntityId) -> Option<&EntityReadRecord> {
        self.entities
            .iter()
            .find(|record| record.entity_id == entity_id)
    }

    pub fn get_relation(&self, relation_id: RelationId) -> Option<&RelationReadRecord> {
        self.relations
            .iter()
            .find(|record| record.relation_id == relation_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketResult {
    pub execution_shape: crate::data::query::QueryExecutionShape,
    pub entities: Vec<EntityReadRecord>,
    pub relations: Vec<RelationReadRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexedReadOutcome {
    pub result: PacketResult,
    pub used_index_generation: Option<crate::data::index::DerivedIndexGenerationId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageStats {
    pub entity_slots: usize,
    pub entity_chunks: usize,
    pub live_entities: usize,
    pub deleted_entities: usize,
    pub reusable_entity_slots: usize,
    pub relation_slots: usize,
    pub relation_chunks: usize,
    pub live_relations: usize,
    pub deleted_relations: usize,
    pub reusable_relation_slots: usize,
    pub snapshot_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionPassOutcome {
    pub entity_reclaimable: usize,
    pub entity_reclaimed: usize,
    pub entity_chunks_scanned: usize,
    pub relation_reclaimable: usize,
    pub relation_reclaimed: usize,
    pub relation_chunks_scanned: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkVisibilitySummary {
    pub chunk_index: usize,
    pub slot_start: usize,
    pub slot_len: usize,
    pub visible_records: usize,
    pub retained_records: usize,
    pub reclaimable_records: usize,
    pub earliest_created_at: Option<VersionId>,
    pub latest_retired_at: Option<VersionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkedStorageSummary {
    pub entity_chunks: Vec<ChunkVisibilitySummary>,
    pub relation_chunks: Vec<ChunkVisibilitySummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkDiagnostics {
    pub version_id: VersionId,
    pub entity_chunks_total: usize,
    pub entity_chunks_with_visible_records: usize,
    pub entity_chunks_with_retained_records: usize,
    pub relation_chunks_total: usize,
    pub relation_chunks_with_visible_records: usize,
    pub relation_chunks_with_retained_records: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryOutcome {
    pub recovered_commits: usize,
    pub latest_commit: Option<crate::data::history::CommitReference>,
    pub restored_branches: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalRuntimeConfig {
    pub profile: RelationalRuntimeProfile,
    pub runtime_name: String,
    pub execution_model: RelationalExecutionModel,
    pub planning: PlanningContract,
    pub commit_authority: CommitAuthorityContract,
    pub diagnostics: crate::data::diagnostics::RelationalDiagnosticsProfile,
    pub version_graph_policy: VersionGraphPolicy,
    pub history_retention: HistoryRetentionClass,
    pub main_branch: BranchId,
    pub schema_registry: RelationalSchemaRegistry,
    pub invariant_catalog: InvariantCatalog,
    pub mvcc: MvccConfig,
    pub storage_layout: StorageLayoutConfig,
    pub publication: PublicationConfig,
    pub durability_mode: DurabilityMode,
    pub config_override: RelationalConfigOverride,
    pub config_provenance: ConfigProvenance,
    pub initial_entity_capacity: usize,
    pub initial_relation_capacity: usize,
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
                diagnostics: crate::data::diagnostics::RelationalDiagnosticsProfile {
                    detailed_traces_enabled: true,
                    max_entries_per_artifact: 512,
                    ..crate::data::diagnostics::RelationalDiagnosticsProfile::default()
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
                },
                storage_layout: StorageLayoutConfig {
                    entity_chunk_size: 1024,
                    relation_chunk_size: 1024,
                    scan_packet_size: 512,
                },
                publication: PublicationConfig {
                    coherent_publication_required: true,
                    max_patch_records_per_commit: 4096,
                },
                durability_mode: DurabilityMode::InMemoryCanonical,
                config_override: RelationalConfigOverride::default(),
                config_provenance: ConfigProvenance {
                    profile,
                    entries: Default::default(),
                },
                initial_entity_capacity: 64,
                initial_relation_capacity: 64,
            },
            RelationalRuntimeProfile::GeometryKernel => Self {
                profile,
                runtime_name: "forge-relational-geometry".to_string(),
                execution_model: RelationalExecutionModel::SerialAuthority,
                planning: PlanningContract::default(),
                commit_authority: CommitAuthorityContract::default(),
                diagnostics: crate::data::diagnostics::RelationalDiagnosticsProfile {
                    detailed_traces_enabled: true,
                    max_entries_per_artifact: 768,
                    ..crate::data::diagnostics::RelationalDiagnosticsProfile::default()
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
                },
                storage_layout: StorageLayoutConfig {
                    entity_chunk_size: 2048,
                    relation_chunk_size: 2048,
                    scan_packet_size: 1024,
                },
                publication: PublicationConfig {
                    coherent_publication_required: true,
                    max_patch_records_per_commit: 8192,
                },
                durability_mode: DurabilityMode::InMemoryCanonical,
                config_override: RelationalConfigOverride::default(),
                config_provenance: ConfigProvenance {
                    profile,
                    entries: Default::default(),
                },
                initial_entity_capacity: 256,
                initial_relation_capacity: 256,
            },
            RelationalRuntimeProfile::ChipSimulation => Self {
                profile,
                runtime_name: "forge-relational-chip".to_string(),
                execution_model: RelationalExecutionModel::SerialAuthority,
                planning: PlanningContract::default(),
                commit_authority: CommitAuthorityContract::default(),
                diagnostics: crate::data::diagnostics::RelationalDiagnosticsProfile {
                    detailed_traces_enabled: false,
                    max_entries_per_artifact: 384,
                    ..crate::data::diagnostics::RelationalDiagnosticsProfile::default()
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
                },
                storage_layout: StorageLayoutConfig {
                    entity_chunk_size: 4096,
                    relation_chunk_size: 4096,
                    scan_packet_size: 2048,
                },
                publication: PublicationConfig {
                    coherent_publication_required: true,
                    max_patch_records_per_commit: 16384,
                },
                durability_mode: DurabilityMode::InMemoryCanonical,
                config_override: RelationalConfigOverride::default(),
                config_provenance: ConfigProvenance {
                    profile,
                    entries: Default::default(),
                },
                initial_entity_capacity: 512,
                initial_relation_capacity: 512,
            },
            RelationalRuntimeProfile::AiWorkflow => Self {
                profile,
                runtime_name: "forge-relational-ai".to_string(),
                execution_model: RelationalExecutionModel::SerialAuthority,
                planning: PlanningContract::default(),
                commit_authority: CommitAuthorityContract::default(),
                diagnostics: crate::data::diagnostics::RelationalDiagnosticsProfile {
                    detailed_traces_enabled: false,
                    max_entries_per_artifact: 256,
                    ..crate::data::diagnostics::RelationalDiagnosticsProfile::default()
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
                },
                storage_layout: StorageLayoutConfig {
                    entity_chunk_size: 2048,
                    relation_chunk_size: 1024,
                    scan_packet_size: 1024,
                },
                publication: PublicationConfig {
                    coherent_publication_required: true,
                    max_patch_records_per_commit: 8192,
                },
                durability_mode: DurabilityMode::InMemoryCanonical,
                config_override: RelationalConfigOverride::default(),
                config_provenance: ConfigProvenance {
                    profile,
                    entries: Default::default(),
                },
                initial_entity_capacity: 128,
                initial_relation_capacity: 128,
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
        }
        if let Some(storage_layout) = &config_override.storage_layout {
            config.storage_layout = storage_layout.clone();
        }
        if let Some(publication) = &config_override.publication {
            config.publication = publication.clone();
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
