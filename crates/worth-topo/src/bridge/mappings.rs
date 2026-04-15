use forge_runtime_bridge::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingId,
    BridgeMappingRegistration, CoarseRoutingMode, MappingSelector, SignalInvalidationScope,
    SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
};
use worth_schema::facade::{
    worth_milestone_two_invalidation_declarations, WorthDerivedTruthSurfaceKind,
};

pub fn worth_milestone_one_bridge_mapping_registrations() -> Vec<BridgeMappingRegistration> {
    worth_milestone_two_invalidation_declarations()
        .into_iter()
        .map(|declaration| {
            BridgeMappingRegistration::new(
                BridgeMappingId::new(format!("worth:m2:{}", declaration.declaration_id)),
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

pub fn worth_milestone_one_bridge_aspect_registrations() -> Vec<BridgeAspectRegistration> {
    worth_milestone_two_invalidation_declarations()
        .into_iter()
        .map(|declaration| {
            BridgeAspectRegistration::new(
                BridgeAspectRegistrationId::new(format!(
                    "worth:m2:aspect:{}",
                    declaration.declaration_id
                )),
                TruthPatchScope::new(
                    MappingSelector::any(),
                    MappingSelector::exact(declaration.truth_patch_field),
                    MappingSelector::any(),
                ),
                match declaration.truth_surface_kind {
                    WorthDerivedTruthSurfaceKind::EntityField => TruthDeltaSurfaceKind::EntityField,
                    WorthDerivedTruthSurfaceKind::EntityRelationEndpoint => {
                        TruthDeltaSurfaceKind::EntityRelationEndpoint
                    }
                },
                SubscriptionSliceKind::SignalField,
                SliceFallbackPolicy::Disallow,
            )
        })
        .collect()
}
