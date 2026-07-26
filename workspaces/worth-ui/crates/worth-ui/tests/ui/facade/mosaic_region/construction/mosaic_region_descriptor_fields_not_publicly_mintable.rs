use worth_ui::facade::{
    declaration::{MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior, SurfacePlacementClass},
};

fn main() {
    let _descriptor = MosaicRegionKindDescriptor {
        id: MosaicRegionKindId::new("workspace.region.primary")
            .expect("valid mosaic region kind id"),
        role: MosaicRegionRole::primary(),
        sizing_behavior: Some(MosaicSizingBehavior::fills_available_space()),
        scroll_ownership: Some(MosaicScrollOwnership::region_owned()),
        focus_scope: Some(MosaicFocusScopeKind::active_surface_scope()),
        child_rule: Some(MosaicChildRule::accepts_surfaces()),
        allowed_surface_classes: Vec::<SurfacePlacementClass>::new(),
        persistence: Some(MosaicRegionPersistence::restorable()),
        clipping: Some(MosaicClippingPosture::clip_to_region()),
        hit_test: Some(MosaicHitTestPosture::participates()),
        label: None,
    };
}
