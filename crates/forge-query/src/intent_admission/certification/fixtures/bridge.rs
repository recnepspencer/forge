use forge_runtime_bridge::facade::{
    BridgeCommittedPatchItem, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    CoarseRoutingMode, InvalidationSink, MappingSelector, RawCommittedPatchEnvelope,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity,
    TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity,
    TruthSnapshotReader,
};

pub(crate) fn certification_bridge() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(CertificationBridgeSource)
        .with_signal_sink(CertificationBridgeSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("certification-external"),
            TruthPatchScope::new(
                MappingSelector::any(),
                MappingSelector::any(),
                MappingSelector::any(),
            ),
            SignalInvalidationScope::new("certification-external"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("certification bridge should build")
}

#[derive(Clone, Debug)]
struct CertificationBridgeSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for CertificationBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch:{}", request.commit_identity())),
            TruthSnapshotIdentity::new("certification-external-snapshot"),
            TruthBranchIdentity::new("main"),
            vec![BridgeCommittedPatchItem::new(
                "certification-entity",
                forge_foundational::facade::AspectKey::new("certification-aspect")
                    .expect("valid bridge patch aspect key"),
                "value",
            )],
        ))
    }
}

impl SnapshotReadSource for CertificationBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Ok(Box::new(CertificationSnapshotReader {
            identity: identity.clone(),
        }))
    }
}

struct CertificationSnapshotReader {
    identity: TruthSnapshotIdentity,
}

impl TruthSnapshotReader for CertificationSnapshotReader {
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

struct CertificationBridgeSink;

impl InvalidationSink for CertificationBridgeSink {
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
