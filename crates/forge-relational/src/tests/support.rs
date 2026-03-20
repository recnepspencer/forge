pub(super) use forge_harness::facade::{
    DiagnosticsHarnessAdapter, ExecutionProfile, ExecutionRequest, HarnessAdapter, MutationBatch,
    ReplayHarnessAdapter, ReplayRequest, ScenarioPlan,
};
pub(super) use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEST_STORE_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(super) use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
pub(super) use crate::config::data::{DurableLogPolicy, DurableLogRetentionMode};
pub(super) use crate::config::data::{PatchSurfacePolicy, PublicationConfig};
pub(super) use crate::facade::config::{
    RelationalRuntimeProfile, StorageLayoutConfig, VisibilityCachePolicy,
};
pub(super) use crate::facade::diagnostics::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
};
pub(super) use crate::facade::durability::{DurabilityMode, DurableStoreLayout};
pub(super) use crate::facade::harness::RelationalHarnessAdapter;
pub(super) use crate::facade::history::{
    AspectFilter, AspectFilterMode, AspectHistoryCommitSpan, AspectHistoryEntry,
    AspectResolutionContext, BranchId, HistoryAspectQueryTarget, RequestedAspectSet,
};
pub(super) use crate::facade::identity::{KindId, LineageId, PartitionId, RelationId};
pub(super) use crate::facade::publication::{
    PatchStreamPosition, PatchStreamRequest, PublicationStage, PublicationStatus,
    SubscriberResumeRequest, SubscriberStreamFailureClass,
};
pub(super) use crate::facade::query::QueryWorkPacket;
pub(super) use crate::facade::runtime::{
    EntityReadRecord, InvariantCatalog, InvariantClass, InvariantRegistration, InvariantRule,
    RelationalRuntime, RelationalRuntimeApi,
};
pub(super) use crate::facade::schema::{
    AspectBinding, AspectComparator, AspectKey, AspectPrecision, DeclaredAspect,
    EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration,
    RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};
pub(super) use crate::facade::transactions::{
    BulkEntityCreateIntent, CommitResult, CreateIntent, DeleteEntityIntent, DeleteRelationIntent,
    EntityMutationIntent, MutationIntent, PatchVsTruthDeltaReport, RecordRef,
    RelationMutationIntent, ReplaceEntityIntent, TransactionCommitError, TransactionOptions,
    UpdateEntityIntent, WorkerIntentBatch,
};
pub(super) use crate::payloads::data::RecordPayload;
pub(super) use crate::publication::cdc::planning::checkpoint_for_schema_version;
pub(super) use crate::publication::data::diff::{
    CanonicalAspectSet, PatchCompatibilityClass, PatchDetail, RecordStructuralChange,
};
pub(super) use crate::schema::data::RelationPayloadClass;
pub(super) use crate::symbols::data::{InternedString, SymbolPolicy};
use crate::tests::harness::model::truth_model::VisibleTruthSummary;

pub(super) fn test_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
            })
        })
        .unwrap()
}

pub(super) fn runtime_with_test_schema() -> RelationalRuntime {
    runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore)
}

pub(super) fn aspect_key(name: &str) -> AspectKey {
    AspectKey(InternedString::Raw(name.to_string()))
}

pub(super) fn entity_payload_aspect(name: &str, field: &str) -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key(name),
        binding: AspectBinding::EntityPayloadField {
            field: InternedString::Raw(field.to_string()),
        },
        comparator: AspectComparator::JsonScalarEquality,
        precision: AspectPrecision::Structured,
    }
}

pub(super) fn relation_payload_aspect(name: &str, field: &str) -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key(name),
        binding: AspectBinding::RelationPayloadField {
            field: InternedString::Raw(field.to_string()),
        },
        comparator: AspectComparator::JsonScalarEquality,
        precision: AspectPrecision::Structured,
    }
}

pub(super) fn lifecycle_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key("lifecycle"),
        binding: AspectBinding::LifecycleTransition,
        comparator: AspectComparator::LifecycleTransitionEquality,
        precision: AspectPrecision::Structured,
    }
}

pub(super) fn relation_source_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key("source"),
        binding: AspectBinding::RelationSourceEndpoint,
        comparator: AspectComparator::EndpointIdentityEquality,
        precision: AspectPrecision::Structured,
    }
}

pub(super) fn relation_target_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key("target"),
        binding: AspectBinding::RelationTargetEndpoint,
        comparator: AspectComparator::EndpointIdentityEquality,
        precision: AspectPrecision::Structured,
    }
}

#[derive(Debug, Clone)]
pub(super) struct AspectSchemaFixture {
    pub entity_kind_id: KindId,
    pub relation_kind_id: KindId,
    pub entity_kind_name: String,
    pub relation_kind_name: String,
    pub schema_id: SchemaId,
    pub schema_version_id: SchemaVersionId,
    pub relation_payload_class: RelationPayloadClass,
    pub cross_context_policy: CrossContextPolicy,
    pub cascade_delete_policy: CascadeDeletePolicy,
    pub entity_aspects: Vec<DeclaredAspect>,
    pub relation_aspects: Vec<DeclaredAspect>,
}

impl Default for AspectSchemaFixture {
    fn default() -> Self {
        Self {
            entity_kind_id: KindId(1),
            relation_kind_id: KindId(2),
            entity_kind_name: "test.entity".to_string(),
            relation_kind_name: "test.relation".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            relation_payload_class: RelationPayloadClass::PayloadBearingRelation,
            cross_context_policy: CrossContextPolicy::AllowExplicit,
            cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            entity_aspects: Vec::new(),
            relation_aspects: Vec::new(),
        }
    }
}

impl AspectSchemaFixture {
    pub(super) fn with_default_declared_aspects(
        cascade_delete_policy: CascadeDeletePolicy,
    ) -> Self {
        Self {
            cascade_delete_policy,
            entity_aspects: vec![entity_payload_aspect("name", "name"), lifecycle_aspect()],
            relation_aspects: vec![
                relation_payload_aspect("label", "label"),
                lifecycle_aspect(),
                relation_source_aspect(),
                relation_target_aspect(),
            ],
            ..Self::default()
        }
    }

    pub(super) fn build_registry(&self) -> RelationalSchemaRegistry {
        RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: self.entity_kind_id,
                kind_name: self.entity_kind_name.clone(),
                schema_id: self.schema_id.clone(),
                schema_version_id: self.schema_version_id,
                aspect_declarations: KindAspectDeclarations::new(self.entity_aspects.clone()),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: self.relation_kind_id,
                    kind_name: self.relation_kind_name.clone(),
                    schema_id: self.schema_id.clone(),
                    schema_version_id: self.schema_version_id,
                    payload_class: self.relation_payload_class,
                    cross_context_policy: self.cross_context_policy,
                    cascade_delete_policy: self.cascade_delete_policy,
                    aspect_declarations: KindAspectDeclarations::new(self.relation_aspects.clone()),
                })
            })
            .unwrap()
    }

    pub(super) fn build_runtime(&self) -> RelationalRuntime {
        RelationalRuntimeApi::builder()
            .schema_registry(self.build_registry())
            .build()
    }
}

pub(super) fn runtime_with_declared_aspect_schema(
    cascade_delete_policy: CascadeDeletePolicy,
) -> RelationalRuntime {
    AspectSchemaFixture::with_default_declared_aspects(cascade_delete_policy).build_runtime()
}

pub(super) fn runtime_with_declared_aspect_schema_profile(
    profile: RelationalRuntimeProfile,
    cascade_delete_policy: CascadeDeletePolicy,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(profile)
        .schema_registry(declared_aspect_schema_registry(cascade_delete_policy))
        .build()
}

pub(super) fn declared_aspect_schema_registry(
    cascade_delete_policy: CascadeDeletePolicy,
) -> RelationalSchemaRegistry {
    AspectSchemaFixture::with_default_declared_aspects(cascade_delete_policy).build_registry()
}

pub(super) fn runtime_with_test_schema_execution_model(
    execution_model: crate::facade::runtime::RelationalExecutionModel,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .execution_model(execution_model)
        .build()
}

pub(super) fn runtime_with_test_schema_profile(
    profile: RelationalRuntimeProfile,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(profile)
        .schema_registry(test_schema_registry())
        .build()
}

pub(super) fn persisted_runtime_with_test_schema() -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: unique_test_store_path("forge-relational-persisted"),
            segment_commit_capacity: 2,
        })
        .build()
}

pub(super) fn persisted_runtime_with_declared_aspect_schema(
    cascade_delete_policy: CascadeDeletePolicy,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(cascade_delete_policy))
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: unique_test_store_path("forge-relational-persisted-aspects"),
            segment_commit_capacity: 2,
        })
        .build()
}

pub(super) fn unique_test_store_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = TEST_STORE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("{prefix}-{nanos}-{counter}"));
    let _ = fs::remove_dir_all(&path);
    path
}

pub(super) fn runtime_with_test_schema_and_chunks(
    entity_chunk_size: usize,
    relation_chunk_size: usize,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .storage_layout(StorageLayoutConfig {
            entity_chunk_size,
            relation_chunk_size,
            scan_packet_size: 64,
        })
        .build()
}

pub(super) fn runtime_with_test_schema_and_invariants(
    invariant_catalog: InvariantCatalog,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .invariant_catalog(invariant_catalog)
        .build()
}

pub(super) fn batch_create(name: &str) -> WorkerIntentBatch {
    WorkerIntentBatch::new(format!("batch-{name}")).push(MutationIntent::Create(
        CreateIntent::Entity(crate::transactions::data::EntitySpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: InternedString::Raw(name.to_string()),
            payload: RecordPayload::StructuredJson(json!({ "name": name })),
        }),
    ))
}

pub(super) fn create_entity(
    runtime: &mut RelationalRuntime,
    name: &str,
) -> crate::facade::identity::EntityId {
    changed_entities(&create_entity_outcome(runtime, name))[0]
}

pub(super) fn create_entity_in_partition(
    runtime: &mut RelationalRuntime,
    name: &str,
    partition_id: PartitionId,
) -> crate::facade::identity::EntityId {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new(format!("batch-{name}")).push(MutationIntent::Create(
            CreateIntent::Entity(crate::transactions::data::EntitySpec {
                partition_id,
                kind_id: KindId(1),
                client_key: InternedString::Raw(name.to_string()),
                payload: RecordPayload::StructuredJson(json!({ "name": name })),
            }),
        )),
    );
    changed_entities(&txn.commit().unwrap())[0]
}

pub(super) fn create_entity_outcome(runtime: &mut RelationalRuntime, name: &str) -> CommitResult {
    create_entity_outcome_on_branch(runtime, name, BranchId("main".to_string()))
}

pub(super) fn create_entity_outcome_on_branch(
    runtime: &mut RelationalRuntime,
    name: &str,
    branch_id: BranchId,
) -> CommitResult {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(batch_create(name));
    txn.commit().unwrap()
}

pub(super) fn delete_entity(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::identity::EntityId,
) -> CommitResult {
    delete_entity_on_branch(runtime, entity_id, BranchId("main".to_string()))
}

pub(super) fn delete_entity_on_branch(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::identity::EntityId,
    branch_id: BranchId,
) -> CommitResult {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("delete").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id }),
        )),
    );
    txn.commit().unwrap()
}

pub(super) fn update_entity(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::identity::EntityId,
    name: &str,
) -> CommitResult {
    update_entity_on_branch(runtime, entity_id, name, BranchId("main".to_string()))
}

pub(super) fn update_entity_on_branch(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::identity::EntityId,
    name: &str,
    branch_id: BranchId,
) -> CommitResult {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id,
                payload: RecordPayload::StructuredJson(json!({ "name": name })),
            }),
        )),
    );
    txn.commit().unwrap()
}

pub(super) fn create_relation(
    runtime: &mut RelationalRuntime,
    source: crate::facade::identity::EntityId,
    target: crate::facade::identity::EntityId,
    client_key: &str,
) -> RelationId {
    create_relation_in_partition(runtime, source, target, client_key, PartitionId::main())
}

pub(super) fn create_relation_in_partition(
    runtime: &mut RelationalRuntime,
    source: crate::facade::identity::EntityId,
    target: crate::facade::identity::EntityId,
    client_key: &str,
    partition_id: PartitionId,
) -> RelationId {
    create_relation_in_partition_on_branch(
        runtime,
        source,
        target,
        client_key,
        partition_id,
        BranchId("main".to_string()),
    )
}

pub(super) fn create_relation_in_partition_on_branch(
    runtime: &mut RelationalRuntime,
    source: crate::facade::identity::EntityId,
    target: crate::facade::identity::EntityId,
    client_key: &str,
    partition_id: PartitionId,
    branch_id: BranchId,
) -> RelationId {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id,
                kind_id: KindId(2),
                client_key: InternedString::Raw(client_key.to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
            },
        ))),
    );
    let outcome = txn.commit().unwrap();
    changed_relations(&outcome)[0]
}

pub(super) fn create_relation_outcome(
    runtime: &mut RelationalRuntime,
    source: crate::facade::identity::EntityId,
    target: crate::facade::identity::EntityId,
    client_key: &str,
) -> CommitResult {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw(client_key.to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
            },
        ))),
    );
    txn.commit().unwrap()
}

pub(super) fn changed_entities(outcome: &CommitResult) -> Vec<crate::facade::identity::EntityId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            RecordRef::Entity(entity_id) => Some(*entity_id),
            RecordRef::Relation(_) => None,
        })
        .collect()
}

pub(super) fn changed_relations(outcome: &CommitResult) -> Vec<RelationId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            RecordRef::Relation(relation_id) => Some(*relation_id),
            RecordRef::Entity(_) => None,
        })
        .collect()
}

pub(super) fn apply_batches(batches: Vec<WorkerIntentBatch>) -> RelationalRuntime {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    for batch in batches {
        txn.push_batch(batch);
    }
    txn.commit().unwrap();
    runtime
}

pub(super) fn merge_commit_from_branches(
    runtime: &mut RelationalRuntime,
    target_branch: BranchId,
    merge_parent_branches: Vec<BranchId>,
) -> CommitResult {
    let txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(target_branch),
        merge_parent_branches,
        ..TransactionOptions::default()
    });
    txn.commit().unwrap()
}

pub(super) fn read_entity_name(record: &EntityReadRecord) -> Option<&str> {
    record
        .payload
        .as_json()
        .and_then(|value| value.get("name"))
        .and_then(|value| value.as_str())
}

pub(super) fn all_aspect_filter(names: impl IntoIterator<Item = &'static str>) -> AspectFilter {
    AspectFilter {
        mode: AspectFilterMode::All,
        aspects: RequestedAspectSet::new(names.into_iter().map(aspect_key)),
    }
}

pub(super) fn any_aspect_filter(names: impl IntoIterator<Item = &'static str>) -> AspectFilter {
    AspectFilter {
        mode: AspectFilterMode::Any,
        aspects: RequestedAspectSet::new(names.into_iter().map(aspect_key)),
    }
}

pub(super) fn entity_aspect_history_digest(
    runtime: &RelationalRuntime,
    entity_id: crate::facade::identity::EntityId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    entity_aspect_history_digest_on_branch(runtime, &BranchId("main".to_string()), entity_id, filter)
}

pub(super) fn entity_aspect_history_digest_on_branch(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    entity_id: crate::facade::identity::EntityId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    runtime
        .history_access()
        .entity_aspect_history_with_trace(branch_id, entity_id, filter)
        .aspect_history_digest()
}

pub(super) fn relation_aspect_history_digest(
    runtime: &RelationalRuntime,
    relation_id: RelationId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    relation_aspect_history_digest_on_branch(
        runtime,
        &BranchId("main".to_string()),
        relation_id,
        filter,
    )
}

pub(super) fn relation_aspect_history_digest_on_branch(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    relation_id: RelationId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::AspectHistoryDigest {
    runtime
        .history_access()
        .relation_aspect_history_with_trace(branch_id, relation_id, filter)
        .aspect_history_digest()
}

pub(super) fn lineage_aspect_history_digest(
    runtime: &RelationalRuntime,
    lineage_id: LineageId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::LineageAspectResolutionDigest {
    lineage_aspect_history_digest_on_branch(
        runtime,
        &BranchId("main".to_string()),
        lineage_id,
        filter,
    )
}

pub(super) fn lineage_aspect_history_digest_on_branch(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    lineage_id: LineageId,
    filter: Option<&AspectFilter>,
) -> crate::facade::history::LineageAspectResolutionDigest {
    runtime
        .lineage_access()
        .entity_aspect_history_with_trace(branch_id, lineage_id, filter)
        .lineage_aspect_resolution_digest()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AspectTruthBundle {
    pub visible_truth: VisibleTruthSummary,
    pub latest_patch: Option<crate::facade::publication::RelationalPatchRecord>,
    pub latest_replay: Option<crate::facade::runtime::RelationalReplayRecord>,
    pub diagnostics: crate::facade::diagnostics::RelationalDiagnosticsFacade,
    pub entity_history_digests:
        Vec<(crate::facade::identity::EntityId, crate::facade::history::AspectHistoryDigest)>,
    pub relation_history_digests:
        Vec<(RelationId, crate::facade::history::AspectHistoryDigest)>,
    pub lineage_history_digests:
        Vec<(LineageId, crate::facade::history::LineageAspectResolutionDigest)>,
}

pub(super) fn capture_aspect_truth_bundle(
    runtime: &mut RelationalRuntime,
    entity_ids: &[crate::facade::identity::EntityId],
    relation_ids: &[RelationId],
    lineage_ids: &[LineageId],
) -> AspectTruthBundle {
    AspectTruthBundle {
        visible_truth: VisibleTruthSummary::capture(runtime),
        latest_patch: runtime.publication_access().latest_patch().cloned(),
        latest_replay: runtime.publication_access().latest_replay().cloned(),
        diagnostics: runtime.publication_access().diagnostics().clone(),
        entity_history_digests: entity_ids
            .iter()
            .map(|entity_id| (*entity_id, entity_aspect_history_digest(runtime, *entity_id, None)))
            .collect(),
        relation_history_digests: relation_ids
            .iter()
            .map(|relation_id| {
                (*relation_id, relation_aspect_history_digest(runtime, *relation_id, None))
            })
            .collect(),
        lineage_history_digests: lineage_ids
            .iter()
            .map(|lineage_id| (*lineage_id, lineage_aspect_history_digest(runtime, *lineage_id, None)))
            .collect(),
    }
}

pub(super) fn assert_stable_aspect_truth_bundle_eq(
    expected: &AspectTruthBundle,
    actual: &AspectTruthBundle,
) {
    assert_eq!(expected.visible_truth, actual.visible_truth);
    assert_eq!(expected.entity_history_digests, actual.entity_history_digests);
    assert_eq!(expected.relation_history_digests, actual.relation_history_digests);
    assert_eq!(expected.lineage_history_digests, actual.lineage_history_digests);
}

pub(super) fn assert_recovered_commit_truth_matches(
    original_runtime: &mut RelationalRuntime,
    recovered_runtime: &mut RelationalRuntime,
    commit_id: crate::facade::history::CommitId,
    entity_ids: &[crate::facade::identity::EntityId],
    relation_ids: &[RelationId],
    lineage_ids: &[LineageId],
) {
    let original_envelope = original_runtime
        .replay_access()
        .canonical_commit_envelope(commit_id)
        .cloned()
        .unwrap();
    let recovered_envelope = recovered_runtime
        .replay_access()
        .canonical_commit_envelope(commit_id)
        .cloned()
        .unwrap();
    let original_bundle =
        capture_aspect_truth_bundle(original_runtime, entity_ids, relation_ids, lineage_ids);
    let recovered_bundle =
        capture_aspect_truth_bundle(recovered_runtime, entity_ids, relation_ids, lineage_ids);

    assert_stable_aspect_truth_bundle_eq(&original_bundle, &recovered_bundle);
    assert_eq!(original_envelope, recovered_envelope);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InspectionTruthBundle {
    pub graph_summary: crate::facade::inspection::GraphInspectionSummary,
    pub kind_summary: crate::facade::inspection::KindInspectionSummary,
    pub connectivity_summary: crate::facade::inspection::ConnectivityInspectionSummary,
    pub historical_record: crate::facade::inspection::HistoricalRecordInspection,
    pub retention_summary: crate::facade::inspection::RetentionInspectionSummary,
    pub record_retention: crate::facade::inspection::RecordRetentionInspection,
    pub branch_head: Option<crate::facade::inspection::CommitInspection>,
    pub latest_commit: crate::facade::inspection::CommitInspection,
    pub recent_commits: crate::facade::inspection::RecentCommitInspectionWindow,
}

pub(super) fn capture_inspection_truth_bundle(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
    entity_id: crate::facade::identity::EntityId,
    historical_version: crate::facade::identity::VersionId,
) -> InspectionTruthBundle {
    let inspection = runtime.inspection_access();
    let latest_commit_id = runtime
        .history_access()
        .latest_commit()
        .map(|commit| commit.commit_id)
        .expect("latest commit");
    InspectionTruthBundle {
        graph_summary: inspection.graph_summary(&crate::facade::inspection::GraphInspectionRequest {
            scope: crate::facade::inspection::InspectionScope::Current,
            partition_scope: None,
            relation_kind_scope: None,
            summary_only: true,
        }),
        kind_summary: inspection.kind_summary(&crate::facade::inspection::KindInspectionRequest {
            scope: crate::facade::inspection::InspectionScope::Current,
            partition_scope: None,
            kind_id: KindId(1),
            record_class: crate::facade::inspection::InspectionRecordClass::Entity,
        }),
        connectivity_summary: inspection.connectivity_summary(
            &crate::facade::inspection::ConnectivityInspectionRequest {
                scope: crate::facade::inspection::InspectionScope::Current,
                partition_scope: None,
                relation_kind_scope: None,
                include_members: false,
            },
        ),
        historical_record: inspection.inspect_historical_record(
            branch_id,
            historical_version,
            RecordRef::Entity(entity_id),
            crate::facade::inspection::HistoricalInspectionMode::AllowCanonicalReconstruction,
        ),
        retention_summary: inspection.retention_summary(),
        record_retention: inspection
            .inspect_record_retention(RecordRef::Entity(entity_id))
            .expect("record retention"),
        branch_head: inspection.inspect_branch_head(branch_id),
        latest_commit: inspection
            .inspect_commit(latest_commit_id)
            .expect("latest commit inspection"),
        recent_commits: inspection.inspect_recent_commits(
            &crate::facade::inspection::RecentCommitInspectionRequest {
                branch_id: Some(branch_id.clone()),
                limit: 8,
            },
        ),
    }
}

pub(super) fn assert_patch_truth_invariants(result: &CommitResult) -> PatchVsTruthDeltaReport {
    let patch_vs_truth = result.patch_vs_truth_delta_report();
    let tag_accuracy = result.aspect_tag_accuracy_report();

    assert!(
        patch_vs_truth.exact_match,
        "patch surface diverged from canonical aspect truth: {:?}",
        patch_vs_truth
    );
    assert_eq!(patch_vs_truth.records_checked, result.patch().len());
    assert_eq!(tag_accuracy.records_checked, result.patch().len());
    assert_eq!(tag_accuracy.correctly_tagged_records, result.patch().len());

    patch_vs_truth
}

pub(super) fn assert_direct_history_origin_invariants(
    entries: &[AspectHistoryEntry],
    target: RecordRef,
) {
    assert!(
        !entries.is_empty(),
        "expected direct aspect history entries for {:?}",
        target
    );
    assert!(entries.iter().all(|entry| entry.origin.target == target));
    assert!(entries.iter().all(|entry| matches!(
        entry.resolution,
        AspectResolutionContext::DirectRecordHistory
    )));
}

pub(super) fn assert_lineage_history_origin_invariants(
    entries: &[AspectHistoryEntry],
    start_lineage_id: LineageId,
) {
    assert!(
        !entries.is_empty(),
        "expected lineage-aware aspect history entries for {:?}",
        start_lineage_id
    );
    assert!(entries.iter().all(|entry| matches!(
        entry.resolution,
        AspectResolutionContext::ResolvedViaLineage {
            start_lineage_id: resolved_start,
            ..
        } if resolved_start == start_lineage_id
    )));
}
