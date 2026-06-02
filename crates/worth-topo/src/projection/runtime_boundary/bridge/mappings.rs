use forge_runtime_bridge::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingId,
    BridgeMappingRegistration, CoarseRoutingMode, MappingSelector, SignalInvalidationScope,
    SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
};
use schema::facade::platform::authority::{
    milestone_two_invalidation_declarations, DerivedTruthSurfaceKind,
};

pub(crate) fn milestone_one_bridge_mapping_registrations() -> Vec<BridgeMappingRegistration> {
    milestone_two_invalidation_declarations()
        .into_iter()
        .map(|declaration| {
            BridgeMappingRegistration::new(
                BridgeMappingId::new(format!(":m2:{}", declaration.declaration_id)),
                TruthPatchScope::new(
                    MappingSelector::any(),
                    MappingSelector::exact(declaration.truth_patch_field),
                    MappingSelector::any(),
                ),
                SignalInvalidationScope::new(declaration.target.bridge_scope()),
                CoarseRoutingMode::Direct,
            )
        })
        .collect()
}

pub(crate) fn milestone_one_bridge_aspect_registrations() -> Vec<BridgeAspectRegistration> {
    milestone_two_invalidation_declarations()
        .into_iter()
        .map(|declaration| {
            BridgeAspectRegistration::new(
                BridgeAspectRegistrationId::new(format!(
                    ":m2:aspect:{}",
                    declaration.declaration_id
                )),
                TruthPatchScope::new(
                    MappingSelector::any(),
                    MappingSelector::exact(declaration.truth_patch_field),
                    MappingSelector::any(),
                ),
                match declaration.truth_surface_kind {
                    DerivedTruthSurfaceKind::EntityField => TruthDeltaSurfaceKind::EntityField,
                    DerivedTruthSurfaceKind::EntityRelationEndpoint => {
                        TruthDeltaSurfaceKind::EntityRelationEndpoint
                    }
                },
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            )
        })
        .collect()
}
