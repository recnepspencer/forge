use forge_runtime_bridge::facade::{
    BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    CoarseRoutingMode, InvalidationSink, MappingSelector, RawCommittedPatchEnvelope,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity,
    TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity,
    TruthSnapshotReader,
};

#[derive(Clone, Debug)]
struct PublicBridgeSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for PublicBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
            TruthSnapshotIdentity::new("public-bridge-snapshot"),
            TruthBranchIdentity::new("main"),
            vec![BridgeCommittedPatchItem::new(
                "entity",
                forge_foundational::facade::AspectKey::new("aspect")
                    .expect("valid bridge patch aspect key"),
                "value",
            )],
        ))
    }
}

impl SnapshotReadSource for PublicBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(PublicBridgeSnapshotReader {
            identity: identity.clone(),
        }))
    }
}

struct PublicBridgeSnapshotReader {
    identity: TruthSnapshotIdentity,
}

impl TruthSnapshotReader for PublicBridgeSnapshotReader {
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

struct PublicBridgeSink;

impl InvalidationSink for PublicBridgeSink {
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

pub(super) fn public_bridge() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(PublicBridgeSource)
        .with_signal_sink(PublicBridgeSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("public-graph"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::any(),
                MappingSelector::any(),
            ),
            SignalInvalidationScope::new("public-graph"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("public bridge should build")
}
