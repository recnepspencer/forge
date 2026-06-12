use forge_foundational::facade::{
    AspectKey, AspectLocator, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
    ScalarAspectType,
};
use forge_runtime_bridge::facade::{
    AspectKeySelector, BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity,
    BridgeCommittedPatchItem, BridgeCommittedPatchTarget, BridgeDeliveryReceipt, BridgeMappingId,
    BridgeMappingRegistration, CoarseRoutingMode, InvalidationSink, MappingSelector,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadContract,
    SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource,
    TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity, TruthPatchScope,
    TruthPatchTargetSelector, TruthSnapshotIdentity, TruthSnapshotReader,
};

pub(crate) fn certification_bridge() -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_relational_source(CertificationBridgeSource)
        .with_signal_sink(CertificationBridgeSink)
        .with_writeback_authority(CertificationWritebackAuthority)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::from_stable_name("certification-external"),
            TruthPatchScope::new(
                MappingSelector::any(),
                AspectKeySelector::exact(aspect_key("certification-aspect")),
                TruthPatchTargetSelector::any(),
            ),
            SnapshotReadContract::scalar(
                aspect_key("certification-aspect"),
                ScalarAspectType::String,
            ),
            SignalInvalidationScope::from_stable_name("certification-external"),
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
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        Ok(native_patch_envelope(
            request.commit_identity().clone(),
            "certification-external-snapshot",
            "main",
            "certification-entity",
            "certification-aspect",
            "value",
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
                .map(|read| SnapshotReadRecord::for_request(read, AspectValue::Null))
                .collect(),
        ))
    }
}

fn native_patch_envelope(
    commit_identity: TruthCommitIdentity,
    snapshot_identity: &str,
    branch_identity: &str,
    entity_identity: &str,
    aspect: &str,
    field: &str,
) -> BridgeCommittedPatchEnvelope {
    let patch_identity = TruthPatchIdentity::from_bridge_harness_label("patch:certification");
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new(
            commit_identity,
            patch_identity,
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
    .expect("intent certification fixture must build a native patch envelope")
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid intent certification bridge aspect key")
}

fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid intent certification bridge field key")
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

#[derive(Clone, Debug)]
struct CertificationWritebackAuthority;

impl forge_runtime_bridge::facade::TruthWritebackAuthority for CertificationWritebackAuthority {
    fn execute_writeback(
        &self,
        request: forge_runtime_bridge::facade::TruthWritebackRequest,
    ) -> Result<
        forge_runtime_bridge::facade::TruthWritebackReceipt,
        forge_runtime_bridge::facade::TruthWritebackAuthorityError,
    > {
        Ok(forge_runtime_bridge::facade::TruthWritebackReceipt::new(
            forge_runtime_bridge::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            &request,
        ))
    }
}
