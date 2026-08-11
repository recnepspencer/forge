use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::history::data::{BranchId, CommitId};
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId};
use crate::schema::data::{ProposedSchemaTransition, SchemaReconciliationPolicy};
use crate::symbols::data::ClientKey;

use super::intents::MutationIntent;
use super::AspectFieldPatch;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TransactionId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SavepointId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorityMode {
    SerializedCommit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitAuthority {
    pub mode: AuthorityMode,
    pub label: String,
}

impl Default for CommitAuthority {
    fn default() -> Self {
        Self {
            mode: AuthorityMode::SerializedCommit,
            label: "single-writer deterministic commit".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionOptions {
    pub allow_nested_savepoints: bool,
    pub diagnostics_required: bool,
    pub deterministic_merge_required: bool,
    pub target_branch: Option<BranchId>,
    /// Optional owner-enforced compare-and-commit precondition for the exact
    /// current target-branch head.
    pub expected_branch_head: Option<ExpectedBranchHead>,
    pub merge_parent_branches: Vec<BranchId>,
    pub proposed_schema_transition: Option<ProposedSchemaTransition>,
    pub schema_reconciliation_policy: Option<SchemaReconciliationPolicy>,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            allow_nested_savepoints: true,
            diagnostics_required: true,
            deterministic_merge_required: true,
            target_branch: None,
            expected_branch_head: None,
            merge_parent_branches: Vec::new(),
            proposed_schema_transition: None,
            schema_reconciliation_policy: None,
        }
    }
}

impl TransactionOptions {
    pub fn for_branch(target_branch: BranchId) -> Self {
        Self {
            target_branch: Some(target_branch),
            ..Self::default()
        }
    }

    pub fn merge_from_branches(mut self, branches: Vec<BranchId>) -> Self {
        self.merge_parent_branches = branches;
        self
    }

    pub fn expect_branch_head(mut self, expected: ExpectedBranchHead) -> Self {
        self.expected_branch_head = Some(expected);
        self
    }

    pub fn with_schema_transition(
        mut self,
        proposed_schema_transition: ProposedSchemaTransition,
        schema_reconciliation_policy: Option<SchemaReconciliationPolicy>,
    ) -> Self {
        self.proposed_schema_transition = Some(proposed_schema_transition);
        self.schema_reconciliation_policy = schema_reconciliation_policy;
        self
    }
}

/// Exact target-branch head required for one Relational commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpectedBranchHead {
    Empty,
    Commit(CommitId),
}

impl ExpectedBranchHead {
    pub const fn observed_commit(self) -> Option<CommitId> {
        match self {
            Self::Empty => None,
            Self::Commit(commit) => Some(commit),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerIntentBatch {
    pub name: String,
    pub partition_key: Option<String>,
    pub worker_local_only: bool,
    pub intents: Vec<MutationIntent>,
}

impl WorkerIntentBatch {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            partition_key: None,
            worker_local_only: true,
            intents: Vec::new(),
        }
    }

    pub fn with_partition_key(mut self, partition_key: impl Into<String>) -> Self {
        self.partition_key = Some(partition_key.into());
        self
    }

    pub fn push(mut self, intent: MutationIntent) -> Self {
        self.intents.push(intent);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecordRef {
    Entity(EntityId),
    Relation(RelationId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExistingRecordTarget {
    Entity(EntityId),
    Relation(RelationId),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RelationIdentity {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub source: EntityReference,
    pub target: EntityReference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationScope {
    SamePartition,
    CrossPartition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrossContextEndpointClass {
    SamePartitionEndpoints,
    CrossPartitionEndpoints,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitySpec {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_key: ClientKey,
    pub fields: AspectFieldPatch,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CreatedEntityRef {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_key: ClientKey,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EntityReference {
    Existing(EntityId),
    Created(CreatedEntityRef),
}

impl EntityReference {
    pub fn partition_id(&self) -> PartitionId {
        match self {
            Self::Existing(entity_id) => entity_id.partition_id,
            Self::Created(created) => created.partition_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationSpec {
    pub partition_id: PartitionId,
    pub kind_id: KindId,
    pub client_key: ClientKey,
    pub source: EntityReference,
    pub target: EntityReference,
    pub fields: AspectFieldPatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BulkMutationScope {
    BulkEntityCreate,
    BulkRelationCreate,
    BulkMixedMutation,
    TopologyRegionRewrite,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkMutationLocalityFootprint {
    pub touched_partitions: Arc<[PartitionId]>,
    pub cross_partition_relation_count: usize,
    pub entity_target_count: usize,
    pub relation_target_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkMutationNamingPlan {
    pub normalized_client_keys: Arc<[ClientKey]>,
    pub naming_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlannedLineageTransition {
    CreateEntity {
        partition_id: PartitionId,
        kind_id: KindId,
        client_key: ClientKey,
    },
    ReplaceEntity {
        entity_id: EntityId,
        replacement_partition_id: PartitionId,
        replacement_kind_id: KindId,
        replacement_client_key: ClientKey,
    },
    DeleteEntity {
        entity_id: EntityId,
    },
    CreateRelation {
        partition_id: PartitionId,
        kind_id: KindId,
        source: EntityReference,
        target: EntityReference,
        client_key: ClientKey,
    },
    UpdateRelationEndpoints {
        relation_id: RelationId,
        source: EntityReference,
        target: EntityReference,
    },
    DeleteRelation {
        relation_id: RelationId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkMutationLineagePlan {
    pub transitions: Arc<[PlannedLineageTransition]>,
    pub lineage_scope_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkMutationProvenancePlan {
    pub batch_name: String,
    pub target_branch: Option<BranchId>,
    pub worker_batch_names: Arc<[String]>,
    pub worker_partition_keys: Arc<[Option<String>]>,
    pub worker_local_only_flags: Arc<[bool]>,
    pub provenance_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedBulkMutationBatch {
    pub transaction_id: TransactionId,
    pub scope: BulkMutationScope,
    pub locality: BulkMutationLocalityFootprint,
    pub naming: BulkMutationNamingPlan,
    pub lineage: BulkMutationLineagePlan,
    pub provenance: BulkMutationProvenancePlan,
    pub intents: Arc<[MutationIntent]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamingStableBulkMutationBatch {
    planned: PlannedBulkMutationBatch,
    proof_token: BulkMutationAdmissionToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineageSafeBulkMutationBatch {
    naming_stable: NamingStableBulkMutationBatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceCompleteBulkMutationBatch {
    lineage_safe: LineageSafeBulkMutationBatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct BulkMutationAdmissionToken;

impl NamingStableBulkMutationBatch {
    pub fn planned(&self) -> &PlannedBulkMutationBatch {
        &self.planned
    }
}

impl LineageSafeBulkMutationBatch {
    pub fn naming_stable(&self) -> &NamingStableBulkMutationBatch {
        &self.naming_stable
    }

    pub fn planned(&self) -> &PlannedBulkMutationBatch {
        self.naming_stable.planned()
    }
}

impl ProvenanceCompleteBulkMutationBatch {
    pub fn lineage_safe(&self) -> &LineageSafeBulkMutationBatch {
        &self.lineage_safe
    }

    pub fn planned(&self) -> &PlannedBulkMutationBatch {
        self.lineage_safe.planned()
    }
}

pub(crate) fn naming_stable_bulk_mutation_batch(
    planned: PlannedBulkMutationBatch,
) -> NamingStableBulkMutationBatch {
    NamingStableBulkMutationBatch {
        planned,
        proof_token: BulkMutationAdmissionToken,
    }
}

pub(crate) fn lineage_safe_bulk_mutation_batch(
    naming_stable: NamingStableBulkMutationBatch,
) -> LineageSafeBulkMutationBatch {
    LineageSafeBulkMutationBatch { naming_stable }
}

pub(crate) fn provenance_complete_bulk_mutation_batch(
    lineage_safe: LineageSafeBulkMutationBatch,
) -> ProvenanceCompleteBulkMutationBatch {
    ProvenanceCompleteBulkMutationBatch { lineage_safe }
}
