use forge_runtime_bridge::facade::{
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    RuntimeBridgeBuilder, SignalInvalidationScope, SnapshotReadContract, TruthPatchScope,
};

fn registration() -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new("profile-name"),
        TruthPatchScope::for_entity_field(MappingSelector::exact("user"), forge_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"), forge_foundational::facade::FieldKey::new("name".to_owned()).expect("valid native field key")),
        SnapshotReadContract::scalar(
            forge_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            forge_foundational::facade::ScalarAspectType::String,
        ),
        SignalInvalidationScope::new("signal.profile"),
        CoarseRoutingMode::Direct,
    )
}

fn main() {
    let builder = RuntimeBridgeBuilder::new().register_mapping(registration());
    let _ = builder.build();
}
