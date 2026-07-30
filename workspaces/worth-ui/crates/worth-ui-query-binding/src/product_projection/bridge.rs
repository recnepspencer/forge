use worth_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode,
    CommittedPatchSource, InvalidationSink, MappingSelector, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SnapshotReadContract, SnapshotReadSource, TruthPatchScope,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

pub(crate) fn platform_pulse_bridge() -> Result<RuntimeBridge, String> {
    RuntimeBridgeBuilder::new()
        .with_relational_source(ExternalScalarTruthSource)
        .with_signal_sink(ExternalScalarSignalSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::from_stable_name("worth-ui-external-scalar"),
            TruthPatchScope::for_entity_field(
                MappingSelector::any(),
                worth_foundational::facade::AspectKey::new("query_text")
                    .expect("static aspect must admit"),
                worth_foundational::facade::FieldKey::new("status")
                    .expect("static field must admit"),
            ),
            SnapshotReadContract::scalar(
                worth_foundational::facade::AspectKey::new("query_text")
                    .expect("static aspect must admit"),
                worth_foundational::facade::ScalarAspectType::String,
            ),
            SignalInvalidationScope::from_stable_name("worth-ui-external-scalar"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .map_err(|error| error.to_string())
}

#[derive(Clone, Copy)]
struct ExternalScalarTruthSource;

impl CommittedPatchSource for ExternalScalarTruthSource {
    fn load_committed_patch(
        &self,
        _request: RelationalCommittedPatchRequest,
    ) -> Result<
        worth_runtime_bridge::facade::BridgeCommittedPatchEnvelope,
        RelationalBridgeSourceError,
    > {
        Err(RelationalBridgeSourceError::new(
            "external scalar source does not expose relational patch IO",
        ))
    }
}

impl SnapshotReadSource for ExternalScalarTruthSource {
    fn open_snapshot(
        &self,
        _identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        Err(RelationalBridgeSourceError::new(
            "external scalar source does not expose snapshot IO",
        ))
    }
}

#[derive(Clone, Copy)]
struct ExternalScalarSignalSink;

impl InvalidationSink for ExternalScalarSignalSink {
    fn deliver_invalidation(
        &self,
        delivery: worth_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}
