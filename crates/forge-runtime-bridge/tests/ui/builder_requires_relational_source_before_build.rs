use forge_runtime_bridge::facade::{
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    RuntimeBridgeBuilder, SignalInvalidationScope, TruthPatchScope,
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

fn main() {
    let builder = RuntimeBridgeBuilder::new().register_mapping(registration());
    let _ = builder.build();
}
