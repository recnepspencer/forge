use forge_relational::facade::commit_strategies::{
    CommitStrategyId, CommitStrategyRegistration, IntentReconciliationStrategy,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use forge_relational::facade::schema::{
    EntityKindRegistration, KindAspectContractDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use forge_relational::facade::transactions::{
    CreateIntent, EntityMutationIntent, EntitySpec, MutationIntent, RecordRef, TransactionOptions,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};
use forge_relational::facade::{identity::KindId, identity::PartitionId, symbols::ClientKey};
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId,
    BridgeMappingRegistration, CoarseRoutingMode, CommittedPatchSource, InvalidationSink,
    MappingSelector, RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity,
    TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
    TruthWritebackAuthority, TruthWritebackAuthorityError, TruthWritebackReceipt,
    TruthWritebackRequest,
};

use crate::aspect_field_authoring::{
    entity_string_field_aspect, lifecycle_string_aspect,
    single_aspect_field_patch_from_external_json,
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
                client_key: ClientKey::raw(name),
                fields: single_aspect_field_patch_from_external_json(
                    "name",
                    "name",
                    serde_json::json!(name),
                )
                .expect("seed name aspect patch"),
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
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id,
                fields: single_aspect_field_patch_from_external_json(
                    "name",
                    "name",
                    serde_json::json!(name),
                )
                .expect("update name aspect patch"),
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
                forge_runtime_bridge::facade::AspectKeySelector::any(),
                forge_runtime_bridge::facade::TruthPatchTargetSelector::any(),
            ),
            forge_runtime_bridge::facade::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("aspect")
                    .expect("valid bridge mapping aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
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
                forge_runtime_bridge::facade::AspectKeySelector::any(),
                forge_runtime_bridge::facade::TruthPatchTargetSelector::any(),
            ),
            forge_runtime_bridge::facade::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("aspect")
                    .expect("valid bridge mapping aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
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
            aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                entity_string_field_aspect("name", "name").expect("name aspect"),
                lifecycle_string_aspect("lifecycle").expect("lifecycle aspect"),
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
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(BridgeCommittedPatchEnvelope::new(
            forge_runtime_bridge::facade::BridgeCommittedPatchEnvelopeIdentity::new(
                request.commit_identity().clone(),
                TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
                TruthSnapshotIdentity::new("external-snapshot"),
                TruthBranchIdentity::new("main"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "entity",
                forge_runtime_bridge::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("aspect")
                            .expect("valid native bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("field".to_owned())
                            .expect("valid native bridge patch field key"),
                    ),
                ),
            )],
        )
        .expect("native bridge patch envelope fixture must construct"))
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
                .map(|read| {
                    SnapshotReadRecord::for_request(
                        read,
                        forge_foundational::facade::AspectValue::Null,
                    )
                })
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
            &request,
        ))
    }
}
