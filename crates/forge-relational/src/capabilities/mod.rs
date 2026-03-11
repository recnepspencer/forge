use crate::config::data::{RelationalRuntimeConfig, RelationalRuntimeProfile};
use crate::diagnostics::data::{DiagnosticCode, DiagnosticsScope};
use crate::durability::data::{DurableCheckpoint, DurableStore};
use crate::durability::data::DurabilityError;
use crate::history::data::{BranchId, CommitId, CommitReference};
use crate::identity::data::{PartitionId, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::logic::runtime::RuntimeInstrumentation;
use crate::publication::data::PublicationBundle;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::replay::data::RelationalReplayRecord;
use crate::schema::data::RelationalSchemaRegistry;
use crate::schema::data::SchemaVersionId;
use crate::snapshots::data::{SnapshotId, SnapshotReadPolicy};
use crate::storage::logic::state::PartitionAccess;
use crate::storage::data::RelationalReadView;
use crate::storage::overlay::{PartitionState, RelationalDraft};
use crate::{indexes::data::DerivedIndexGeneration, lineage::data::LineageEventRecord};

pub(crate) trait StorageRead: PartitionAccess {}

impl<T: PartitionAccess + ?Sized> StorageRead for T {}

#[allow(dead_code)]
pub(crate) trait StorageWrite: StorageRead {
    fn get_partition_mut(&mut self, partition_id: PartitionId) -> &mut PartitionState;
}

impl StorageWrite for RelationalDraft {
    fn get_partition_mut(&mut self, partition_id: PartitionId) -> &mut PartitionState {
        RelationalDraft::get_partition_mut(self, partition_id)
    }
}

#[allow(dead_code)]
pub(crate) trait VersionSource {
    fn current_version_id(&self) -> VersionId;
}

impl VersionSource for RelationalRuntime {
    fn current_version_id(&self) -> VersionId {
        RelationalRuntime::current_version_id(self)
    }
}

pub(crate) trait RuntimeConfigSource {
    fn runtime_config(&self) -> &RelationalRuntimeConfig;
}

impl RuntimeConfigSource for RelationalRuntime {
    fn runtime_config(&self) -> &RelationalRuntimeConfig {
        &self.config
    }
}

pub(crate) trait InstrumentationSource {
    fn runtime_instrumentation(&self) -> &RuntimeInstrumentation;
}

impl InstrumentationSource for RelationalRuntime {
    fn runtime_instrumentation(&self) -> &RuntimeInstrumentation {
        &self.instrumentation
    }
}

#[allow(dead_code)]
pub(crate) trait SchemaSource {
    fn schema_registry(&self) -> &RelationalSchemaRegistry;
}

impl SchemaSource for RelationalRuntime {
    fn schema_registry(&self) -> &RelationalSchemaRegistry {
        &self.config.schema_registry
    }
}

impl SchemaSource for RelationalSchemaRegistry {
    fn schema_registry(&self) -> &RelationalSchemaRegistry {
        self
    }
}

impl SchemaSource for RelationalRuntimeConfig {
    fn schema_registry(&self) -> &RelationalSchemaRegistry {
        &self.schema_registry
    }
}

#[allow(dead_code)]
pub(crate) trait DiagnosticsSink {
    fn emit_diagnostic_entry(
        &mut self,
        scope: DiagnosticsScope,
        code: DiagnosticCode,
        message: impl Into<String>,
        fields: serde_json::Value,
    );
}

impl DiagnosticsSink for RelationalRuntime {
    fn emit_diagnostic_entry(
        &mut self,
        scope: DiagnosticsScope,
        code: DiagnosticCode,
        message: impl Into<String>,
        fields: serde_json::Value,
    ) {
        self.diagnostic(scope).failure().emit_entry(code, message, fields);
    }
}

#[allow(dead_code)]
pub(crate) trait PublicationSink {
    fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>>;
}

impl PublicationSink for RelationalRuntime {
    fn latest_publication_bundle(&self) -> Option<&PublicationBundle<RelationalReplayRecord>> {
        self.publication.latest_bundle.as_ref()
    }
}

#[allow(dead_code)]
pub(crate) trait LineageSource {
    fn lineage_event_count(&self) -> usize;
}

impl LineageSource for RelationalRuntime {
    fn lineage_event_count(&self) -> usize {
        self.lineage.events.len()
    }
}

#[allow(dead_code)]
pub(crate) trait IndexSource {
    fn unique_entity_field_index(
        &self,
    ) -> &std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<
            String,
            std::collections::BTreeSet<crate::identity::data::EntityId>,
        >,
    >;
}

impl IndexSource for RelationalRuntime {
    fn unique_entity_field_index(
        &self,
    ) -> &std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<
            String,
            std::collections::BTreeSet<crate::identity::data::EntityId>,
        >,
    > {
        &self.indexes.entity_unique_field_index
    }
}

pub(crate) trait RuntimeIdentitySource {
    fn runtime_name(&self) -> &str;
    fn runtime_profile(&self) -> RelationalRuntimeProfile;
}

impl RuntimeIdentitySource for RelationalRuntime {
    fn runtime_name(&self) -> &str {
        &self.config.runtime_name
    }

    fn runtime_profile(&self) -> RelationalRuntimeProfile {
        self.config.profile
    }
}

pub(crate) trait CommitEnvelopeSource {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope>;
}

impl CommitEnvelopeSource for RelationalRuntime {
    fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope> {
        self.history.commit_envelopes.get(&commit_id)
    }
}

pub(crate) trait HistorySource: CommitEnvelopeSource {
    fn branch_head_ref(&self, branch_id: &BranchId) -> Option<&CommitReference>;
    fn next_commit_id(&self) -> CommitId;
}

impl HistorySource for RelationalRuntime {
    fn branch_head_ref(&self, branch_id: &BranchId) -> Option<&CommitReference> {
        self.history
            .branch_heads
            .get(branch_id)
            .and_then(|head| head.as_ref())
    }

    fn next_commit_id(&self) -> CommitId {
        CommitId(self.history.next_commit_id)
    }
}

pub(crate) trait DurabilityRead {
    fn durable_checkpoints(&self) -> &[DurableCheckpoint];
    fn durable_store(&self) -> Option<&DurableStore>;
    fn durable_log(&self) -> &[CanonicalCommitEnvelope];
}

impl DurabilityRead for RelationalRuntime {
    fn durable_checkpoints(&self) -> &[DurableCheckpoint] {
        &self.durability.checkpoints
    }

    fn durable_store(&self) -> Option<&DurableStore> {
        self.durability.store.as_ref()
    }

    fn durable_log(&self) -> &[CanonicalCommitEnvelope] {
        &self.durability.log
    }
}

pub(crate) trait ReplayRead {
    fn read_view_at_version(&self, version_id: VersionId) -> RelationalReadView;
    fn index_generations_at_version(&self, version_id: VersionId) -> Vec<DerivedIndexGeneration>;
}

impl ReplayRead for RelationalRuntime {
    fn read_view_at_version(&self, version_id: VersionId) -> RelationalReadView {
        self.read_version(version_id)
    }

    fn index_generations_at_version(&self, version_id: VersionId) -> Vec<DerivedIndexGeneration> {
        self.index_generations_for_version(version_id)
    }
}

pub(crate) trait LineageRead {
    fn lineage_events(&self) -> &[LineageEventRecord];
}

impl LineageRead for RelationalRuntime {
    fn lineage_events(&self) -> &[LineageEventRecord] {
        &self.lineage.events
    }
}

pub(crate) trait SnapshotSource {
    fn active_snapshot_binding(&self, snapshot_id: SnapshotId) -> Option<(VersionId, SnapshotReadPolicy)>;
    fn published_snapshot_version(&self, snapshot_id: SnapshotId) -> Option<VersionId>;
}

impl SnapshotSource for RelationalRuntime {
    fn active_snapshot_binding(
        &self,
        snapshot_id: SnapshotId,
    ) -> Option<(VersionId, SnapshotReadPolicy)> {
        self.snapshots
            .active
            .get(&snapshot_id)
            .map(|binding| (binding.version_id, binding.read_policy))
    }

    fn published_snapshot_version(&self, snapshot_id: SnapshotId) -> Option<VersionId> {
        self.snapshots.published_handles.get(&snapshot_id).copied()
    }
}

pub(crate) trait VisibilityPolicySource {
    fn visibility_cache_enabled(&self) -> bool;
    fn recent_visibility_window(&self) -> usize;
    fn protect_active_snapshots(&self) -> bool;
    fn protect_branch_heads(&self) -> bool;
}

impl VisibilityPolicySource for RelationalRuntime {
    fn visibility_cache_enabled(&self) -> bool {
        self.config.visibility_cache_policy.enabled
    }

    fn recent_visibility_window(&self) -> usize {
        self.config.visibility_cache_policy.recent_version_window
    }

    fn protect_active_snapshots(&self) -> bool {
        self.config.visibility_cache_policy.protect_active_snapshots
    }

    fn protect_branch_heads(&self) -> bool {
        self.config.visibility_cache_policy.protect_branch_heads
    }
}

pub(crate) trait PublicationPolicySource {
    fn max_patch_records_per_commit(&self) -> usize;
}

impl PublicationPolicySource for RelationalRuntime {
    fn max_patch_records_per_commit(&self) -> usize {
        self.config.publication.max_patch_records_per_commit
    }
}

pub(crate) trait SchemaVersionSource {
    fn primary_schema_version_id(&self) -> SchemaVersionId;
}

impl SchemaVersionSource for RelationalRuntime {
    fn primary_schema_version_id(&self) -> SchemaVersionId {
        self.primary_schema_version()
    }
}

pub(crate) trait DurabilityWrite {
    fn append_durable_envelope(
        &mut self,
        envelope: CanonicalCommitEnvelope,
    ) -> Result<(), DurabilityError>;
}

impl DurabilityWrite for RelationalRuntime {
    fn append_durable_envelope(
        &mut self,
        envelope: CanonicalCommitEnvelope,
    ) -> Result<(), DurabilityError> {
        self.append_durable_commit(envelope)
    }
}
