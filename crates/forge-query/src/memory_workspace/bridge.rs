use super::*;
use forge_relational::facade::identity::PartitionId;
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::transactions::{
    CreateIntent, DeleteEntityIntent, EntityMutationIntent, EntitySpec, MutationIntent,
    TransactionOptions, UpdateEntityIntent, WorkerIntentBatch,
};
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    CoarseRoutingMode, InvalidationSink, MappingSelector, RawCommittedPatchEnvelope,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity,
    TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity,
    TruthSnapshotReader, TruthWritebackAuthority, TruthWritebackAuthorityError,
    TruthWritebackReceipt, TruthWritebackRequest,
};

pub(super) fn build_query_memory_bridge(
    authority_state: Arc<Mutex<ForgeQueryAuthorityState>>,
) -> Result<RuntimeBridge, ForgeQueryWorkspaceError> {
    RuntimeBridgeBuilder::new()
        .with_policy(forge_runtime_bridge::facade::BridgeRuntimePolicy::development())
        .with_relational_source(ForgeQueryBridgeSource)
        .with_signal_sink(ForgeQueryBridgeSink)
        .with_writeback_authority(ForgeQueryWritebackAuthority {
            state: authority_state,
        })
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("forge-query-memory"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::any(),
                MappingSelector::any(),
            ),
            SignalInvalidationScope::new("forge-query-memory"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .map_err(|error| ForgeQueryWorkspaceError::new(format!("{error:?}")))
}

impl forge_runtime_bridge::facade::CommittedPatchSource for ForgeQueryBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
            TruthSnapshotIdentity::new("snapshot:forge-query-memory"),
            TruthBranchIdentity::new("main"),
            vec![BridgeCommittedPatchItem::new(
                "entity:forge-query-memory",
                "mutation",
                "value",
            )],
        ))
    }
}

impl SnapshotReadSource for ForgeQueryBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(ForgeQueryBridgeSnapshotReader {
            identity: identity.clone(),
        }))
    }
}

impl TruthSnapshotReader for ForgeQueryBridgeSnapshotReader {
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

impl InvalidationSink for ForgeQueryBridgeSink {
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

impl TruthWritebackAuthority for ForgeQueryWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| TruthWritebackAuthorityError::new("query memory authority poisoned"))?;
        let pending = state
            .pending
            .remove(request.proposed_effect_digest())
            .ok_or_else(|| {
                TruthWritebackAuthorityError::new(format!(
                    "no pending query writeback for `{}`",
                    request.proposed_effect_digest()
                ))
            })?;
        let mut txn = state
            .runtime
            .begin_transaction(TransactionOptions::default());
        let batch = match pending.operation {
            ForgeQueryPendingOperation::Insert {
                kind_id,
                client_key,
                payload,
            } => WorkerIntentBatch::new("query-memory-authority-insert").push(
                MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id,
                    client_key,
                    payload: RecordPayload::StructuredJson(payload),
                })),
            ),
            ForgeQueryPendingOperation::Update {
                entity_id, payload, ..
            } => WorkerIntentBatch::new("query-memory-authority-update").push(
                MutationIntent::Entity(EntityMutationIntent::Update(UpdateEntityIntent {
                    entity_id,
                    payload: RecordPayload::StructuredJson(payload),
                })),
            ),
            ForgeQueryPendingOperation::Delete { entity_id, .. } => WorkerIntentBatch::new(
                "query-memory-authority-delete",
            )
            .push(MutationIntent::Entity(EntityMutationIntent::Delete(
                DeleteEntityIntent { entity_id },
            ))),
        };
        txn.push_batch(batch);
        let result = txn
            .commit()
            .map_err(|error| TruthWritebackAuthorityError::new(format!("{error:?}")))?;
        let receipt = super::helpers::receipt_from_runtime_commit(
            &state.runtime,
            result,
            pending.collection,
            pending.kind,
            pending.aspect_paths,
        );
        let artifact_digest = format!("forge-query-authoritative:{}", receipt.commit_identity);
        state.completed.insert(artifact_digest.clone(), receipt);
        Ok(TruthWritebackReceipt::new(
            forge_runtime_bridge::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            artifact_digest,
            &request,
        ))
    }
}
