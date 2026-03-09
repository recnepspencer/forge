use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::diagnostics::{
    DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticArtifact,
};
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
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationReadRecord {
    pub relation_id: RelationId,
    pub kind: KindResolution,
    pub lifecycle: RecordLifecycleState,
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
pub struct StorageStats {
    pub entity_slots: usize,
    pub live_entities: usize,
    pub reusable_entity_slots: usize,
    pub relation_slots: usize,
    pub live_relations: usize,
    pub reusable_relation_slots: usize,
    pub snapshot_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalRuntimeConfig {
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
    pub initial_entity_capacity: usize,
    pub initial_relation_capacity: usize,
}

impl Default for RelationalRuntimeConfig {
    fn default() -> Self {
        Self {
            runtime_name: "forge-relational".to_string(),
            execution_model: RelationalExecutionModel::SerialAuthority,
            planning: PlanningContract::default(),
            commit_authority: CommitAuthorityContract::default(),
            diagnostics: crate::data::diagnostics::RelationalDiagnosticsProfile::default(),
            version_graph_policy: VersionGraphPolicy::CanonicalSerializedPublication,
            history_retention: HistoryRetentionClass::Durable,
            main_branch: BranchId("main".to_string()),
            schema_registry: RelationalSchemaRegistry::default(),
            invariant_catalog: InvariantCatalog::default(),
            initial_entity_capacity: 64,
            initial_relation_capacity: 64,
        }
    }
}
