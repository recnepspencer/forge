use crate::mapping::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingId,
    BridgeMappingRegistration, CoarseRoutingMode, MappingSelector, SignalInvalidationScope,
    SliceWideningPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
};

pub(in crate::builder::tests) fn exact_registration(mapping_id: &str) -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new(mapping_id),
        TruthPatchScope::for_entity_field(
            MappingSelector::exact("user"),
            forge_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            forge_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native field key"),
        ),
        crate::snapshot::SnapshotReadContract::scalar(
            forge_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            forge_foundational::facade::ScalarAspectType::String,
        ),
        SignalInvalidationScope::new("signal.user.profile"),
        CoarseRoutingMode::Direct,
    )
}

pub(in crate::builder::tests) fn exact_aspect_registration(
    registration_id: &str,
) -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::new(registration_id),
        TruthPatchScope::for_entity_field(
            MappingSelector::exact("user"),
            forge_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            forge_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native field key"),
        ),
        crate::snapshot::SnapshotReadContract::scalar(
            forge_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            forge_foundational::facade::ScalarAspectType::String,
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceWideningPolicy::Disallow,
    )
}
