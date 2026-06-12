use forge_foundational::facade::{
    AspectKey, AspectLocator, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
    ScalarAspectType,
};
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    BridgeCommittedPatchTarget, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    BridgeWritebackOutcomeClass, CoarseRoutingMode, InvalidationSink, MappingSelector,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadContract,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthSnapshotIdentity, TruthSnapshotReader, TruthWritebackAuthority,
    TruthWritebackAuthorityError, TruthWritebackReceipt, TruthWritebackRequest,
};

#[derive(Clone, Debug)]
struct PublicBridgeSource;

impl forge_runtime_bridge::facade::CommittedPatchSource for PublicBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(native_patch_envelope(
            request.commit_identity().clone(),
            "public-bridge-snapshot",
            "main",
            "entity",
            "aspect",
            "value",
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
                .map(|read| SnapshotReadRecord::for_request(read, AspectValue::Null))
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

#[derive(Clone, Debug)]
struct PublicBridgeWritebackAuthority;

impl TruthWritebackAuthority for PublicBridgeWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        Ok(TruthWritebackReceipt::new(
            BridgeWritebackOutcomeClass::AuthoritativeCommit,
            &request,
        ))
    }
}

pub(super) fn public_bridge() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(PublicBridgeSource)
        .with_signal_sink(PublicBridgeSink)
        .with_writeback_authority(PublicBridgeWritebackAuthority)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::from_stable_name("public-graph"),
            TruthPatchScope::for_entity_field(
                MappingSelector::any(),
                aspect_key("aspect"),
                field_key("field"),
            ),
            SnapshotReadContract::scalar(aspect_key("aspect"), ScalarAspectType::String),
            SignalInvalidationScope::from_stable_name("public-graph"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("public bridge should build")
}

fn native_patch_envelope(
    commit_identity: TruthCommitIdentity,
    snapshot_identity: &str,
    branch_identity: &str,
    entity_identity: &str,
    aspect: &str,
    field: &str,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new(
            commit_identity.clone(),
            TruthPatchIdentity::from_bridge_harness_label(format!(
                "patch:{}",
                commit_identity.evidence_identity().as_str()
            )),
            TruthSnapshotIdentity::from_bridge_harness_label(snapshot_identity),
            TruthBranchIdentity::from_bridge_harness_label(branch_identity),
        ),
        vec![BridgeCommittedPatchItem::with_target(
            entity_identity,
            BridgeCommittedPatchTarget::entity_field_path(
                AspectLocator::new(LocatorAuthority::Authoritative, aspect_key(aspect)),
                CanonicalFieldPath::single(field_key(field)),
            ),
        )],
    )
    .expect("public bridge fixture must build a native patch envelope")
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid bridge fixture aspect key")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid bridge fixture field key")
}
