use forge_relational::facade::commit_strategies::{
    CommitStrategyId, CommitStrategyRegistration, IntentReconciliationStrategy,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use forge_relational::facade::schema::{
    AspectBinding, AspectComparator, AspectKey, AspectPrecision, DeclaredAspect,
    EntityKindRegistration, KindAspectDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use forge_relational::facade::transactions::{
    CreateIntent, EntityMutationIntent, EntitySpec, MutationIntent, RecordRef, TransactionOptions,
    UpdateEntityIntent, WorkerIntentBatch,
};
use forge_relational::facade::{identity::KindId, identity::PartitionId, symbols::InternedString};
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    CoarseRoutingMode, CommittedPatchSource, InvalidationSink, MappingSelector,
    RawCommittedPatchEnvelope, RelationalBridgeSourceError, RelationalCommittedPatchRequest,
    RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthSnapshotIdentity, TruthSnapshotReader, TruthWritebackAuthority,
    TruthWritebackAuthorityError, TruthWritebackReceipt, TruthWritebackRequest,
};

pub(crate) fn relational_runtime_with_intent_strategy() -> RelationalRuntime {
    let descriptor = IntentReconciliationStrategy::descriptor(CommitStrategyId(211));
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .commit_strategy(
            CommitStrategyRegistration::new(descriptor.clone()).expect("strategy registration"),
        )
        .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
            &descriptor,
        ))
        .build()
}

pub(crate) fn create_entity(
    runtime: &mut RelationalRuntime,
    name: &str,
    branch: BranchId,
) -> forge_relational::facade::identity::EntityId {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new(format!("create-{name}")).push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw(name.to_string()),
                payload: RecordPayload::StructuredJson(serde_json::json!({ "name": name })),
            }),
        )),
    );
    let outcome = txn.commit().expect("seed commit should succeed");
    outcome
        .changed_records
        .iter()
        .find_map(|record| match record {
            RecordRef::Entity(entity_id) => Some(*entity_id),
            RecordRef::Relation(_) => None,
        })
        .expect("seed commit should touch one entity")
}

pub(crate) fn update_entity_name(
    runtime: &mut RelationalRuntime,
    entity_id: forge_relational::facade::identity::EntityId,
    name: &str,
    branch: BranchId,
) -> forge_relational::facade::history::CommitId {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new(format!("update-{name}")).push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id,
                payload: RecordPayload::StructuredJson(serde_json::json!({ "name": name })),
            }),
        )),
    );
    txn.commit()
        .expect("intervening update should succeed")
        .outcome
        .commit
        .commit_id
}

pub(crate) fn test_bridge() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(TestBridgeSource)
        .with_signal_sink(TestBridgeSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("external-test"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::any(),
                MappingSelector::any(),
            ),
            SignalInvalidationScope::new("external-test"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("test bridge should build")
}

pub(crate) fn test_bridge_with_writeback_authority() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(TestBridgeSource)
        .with_signal_sink(TestBridgeSink)
        .with_writeback_authority(StaticWritebackAuthority)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("external-test"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::any(),
                MappingSelector::any(),
            ),
            SignalInvalidationScope::new("external-test"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("test bridge with writeback authority should build")
}

pub(crate) fn runtime_snapshot_token(runtime: &RelationalRuntime) -> String {
    let version_id = runtime
        .history()
        .latest_commit()
        .map(|commit| commit.version_id.0)
        .unwrap_or(0);
    format!("snapshot-{version_id}")
}

pub(crate) fn branch_snapshot_token(runtime: &RelationalRuntime, branch: &str) -> String {
    let version_id = runtime
        .history()
        .branch_head(&BranchId(branch.to_string()))
        .map(|commit| commit.version_id.0)
        .unwrap_or(0);
    format!("snapshot-{version_id}")
}

fn test_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![
                DeclaredAspect {
                    key: AspectKey(InternedString::Raw("name".to_string())),
                    binding: AspectBinding::EntityPayloadField {
                        field: InternedString::Raw("name".to_string()),
                    },
                    comparator: AspectComparator::JsonScalarEquality,
                    precision: AspectPrecision::Structured,
                },
                DeclaredAspect {
                    key: AspectKey(InternedString::Raw("lifecycle".to_string())),
                    binding: AspectBinding::LifecycleTransition,
                    comparator: AspectComparator::LifecycleTransitionEquality,
                    precision: AspectPrecision::Structured,
                },
            ]),
        })
        .expect("test entity kind should register")
}

#[derive(Clone, Debug)]
struct TestBridgeSource;

impl CommittedPatchSource for TestBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
            TruthSnapshotIdentity::new("external-snapshot"),
            TruthBranchIdentity::new("main"),
            vec![BridgeCommittedPatchItem::new("entity", "aspect", "field")],
        ))
    }
}

impl SnapshotReadSource for TestBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(TestSnapshotReader {
            identity: identity.clone(),
        }))
    }
}

#[derive(Clone, Debug)]
struct TestSnapshotReader {
    identity: TruthSnapshotIdentity,
}

impl TruthSnapshotReader for TestSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        Ok(SnapshotReadPacketResult::new(
            self.identity.clone(),
            request
                .reads()
                .iter()
                .map(|read| SnapshotReadRecord::new(read.request_key(), Vec::new()))
                .collect(),
        ))
    }
}

#[derive(Clone, Debug)]
struct TestBridgeSink;

impl InvalidationSink for TestBridgeSink {
    fn deliver_invalidation(
        &self,
        delivery: forge_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

#[derive(Clone, Debug)]
struct StaticWritebackAuthority;

impl TruthWritebackAuthority for StaticWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        Ok(TruthWritebackReceipt::new(
            forge_runtime_bridge::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            format!("authoritative-artifact:{}", request.digest()),
            &request,
        ))
    }
}
