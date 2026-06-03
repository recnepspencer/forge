use forge_foundational::facade::{AspectKey, ScalarAspectType};
use forge_runtime_bridge::facade::{
    AspectKeySelector, BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingId,
    BridgeMappingRegistration, CoarseRoutingMode, MappingSelector, SignalInvalidationScope,
    SliceWideningPolicy, SnapshotReadContract, SubscriptionSliceKind, TruthDeltaSurfaceKind,
    TruthPatchScope, TruthPatchTargetSelector,
};
use schema::facade::platform::authority::{
    milestone_two_invalidation_declarations, DerivedTruthSurfaceKind,
};

pub(crate) fn milestone_one_bridge_mapping_registrations() -> Vec<BridgeMappingRegistration> {
    milestone_two_invalidation_declarations()
        .into_iter()
        .map(|declaration| {
            let aspect_key = native_aspect_key(declaration.truth_patch_field);
            BridgeMappingRegistration::new(
                BridgeMappingId::new(format!(":m2:{}", declaration.declaration_id)),
                native_truth_patch_scope(aspect_key.clone(), declaration.truth_surface_kind),
                native_snapshot_read_contract(declaration.truth_patch_field, aspect_key),
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
            let aspect_key = native_aspect_key(declaration.truth_patch_field);
            BridgeAspectRegistration::new(
                BridgeAspectRegistrationId::new(format!(
                    ":m2:aspect:{}",
                    declaration.declaration_id
                )),
                native_truth_patch_scope(aspect_key.clone(), declaration.truth_surface_kind),
                native_snapshot_read_contract(declaration.truth_patch_field, aspect_key),
                match declaration.truth_surface_kind {
                    DerivedTruthSurfaceKind::EntityField => TruthDeltaSurfaceKind::EntityField,
                    DerivedTruthSurfaceKind::EntityRelationEndpoint => {
                        TruthDeltaSurfaceKind::EntityRelationEndpoint
                    }
                },
                SubscriptionSliceKind::SignalField,
                SliceWideningPolicy::Disallow,
            )
        })
        .collect()
}

fn native_aspect_key(aspect: &str) -> AspectKey {
    AspectKey::new(aspect).expect("worth schema declarations use native aspect keys")
}

fn native_snapshot_read_contract(aspect: &str, aspect_key: AspectKey) -> SnapshotReadContract {
    SnapshotReadContract::scalar(aspect_key, native_scalar_type(aspect))
}

fn native_scalar_type(aspect: &str) -> ScalarAspectType {
    match aspect {
        "source"
        | "target"
        | "naming.source_identity"
        | "naming.target_identity"
        | "topology.ownership"
        | "topology.boundary"
        | "topology.radial" => ScalarAspectType::EntityRef,
        _ => ScalarAspectType::String,
    }
}

fn native_truth_patch_scope(
    aspect_key: AspectKey,
    truth_surface_kind: DerivedTruthSurfaceKind,
) -> TruthPatchScope {
    TruthPatchScope::new(
        MappingSelector::any(),
        AspectKeySelector::exact(aspect_key),
        native_truth_patch_target_selector(truth_surface_kind),
    )
}

fn native_truth_patch_target_selector(
    truth_surface_kind: DerivedTruthSurfaceKind,
) -> TruthPatchTargetSelector {
    match truth_surface_kind {
        DerivedTruthSurfaceKind::EntityField => TruthPatchTargetSelector::region(),
        DerivedTruthSurfaceKind::EntityRelationEndpoint => {
            TruthPatchTargetSelector::relation_endpoint()
        }
    }
}
