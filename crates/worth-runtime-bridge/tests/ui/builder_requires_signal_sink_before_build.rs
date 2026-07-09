use worth_runtime_bridge::facade::{
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    BridgeCommittedPatchEnvelope, CommittedPatchSource, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridgeBuilder, SignalInvalidationScope,
    SnapshotReadContract, SnapshotReadSource, TruthPatchScope, TruthSnapshotIdentity,
    TruthSnapshotReader,
};

fn registration() -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new("profile-name"),
        TruthPatchScope::for_entity_field(MappingSelector::exact("user"), worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"), worth_foundational::facade::FieldKey::new("name".to_owned()).expect("valid native field key")),
        SnapshotReadContract::scalar(
            worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            worth_foundational::facade::ScalarAspectType::String,
        ),
        SignalInvalidationScope::new("signal.profile"),
        CoarseRoutingMode::Direct,
    )
}

struct Source;

impl CommittedPatchSource for Source {
    fn load_committed_patch(
        &self,
        _request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        unreachable!()
    }

}

impl SnapshotReadSource for Source {
    fn open_snapshot(
        &self,
        _identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        unreachable!()
    }
}

fn main() {
    let builder = RuntimeBridgeBuilder::new()
        .with_relational_source(Source)
        .register_mapping(registration());
    let _ = builder.build();
}
