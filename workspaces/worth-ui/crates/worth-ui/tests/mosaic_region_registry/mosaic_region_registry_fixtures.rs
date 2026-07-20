use worth_ui::facade::registry::{
    MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture,
    MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole,
    MosaicScrollOwnership, MosaicSizingBehavior, SurfacePlacementClass,
};

pub(crate) fn mosaic_region_descriptor(id: &str) -> MosaicRegionKindDescriptor {
    mosaic_region_descriptor_with_role(id, MosaicRegionRole::primary())
}

pub(crate) fn mosaic_region_descriptor_with_role(
    id: &str,
    role: MosaicRegionRole,
) -> MosaicRegionKindDescriptor {
    complete_mosaic_region_descriptor(id, role, MosaicChildRule::accepts_surfaces())
        .with_allowed_surface_class(SurfacePlacementClass::primary_region())
}

pub(crate) fn complete_mosaic_region_descriptor(
    id: &str,
    role: MosaicRegionRole,
    child_rule: MosaicChildRule,
) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(mosaic_region_id(id), role)
        .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
        .with_scroll_ownership(MosaicScrollOwnership::region_owned())
        .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
        .with_child_rule(child_rule)
        .with_persistence(MosaicRegionPersistence::restorable())
        .with_clipping(MosaicClippingPosture::clip_to_region())
        .with_hit_test(MosaicHitTestPosture::participates())
}

pub(crate) fn mosaic_region_id(raw_text: &str) -> MosaicRegionKindId {
    MosaicRegionKindId::new(raw_text).expect("valid mosaic region kind id")
}
