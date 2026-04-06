use forge_runtime_bridge::facade::{
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    CommittedPatchSource, RawCommittedPatchEnvelope, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridgeBuilder, SignalInvalidationScope,
    SnapshotReadSource, TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
};

fn registration() -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new("profile-name"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
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
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
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
