use worth_ui::facade::{
    app::WorthUi,
    registry::{MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior, SurfacePlacementClass},
};

fn main() {
    let _app = WorthUi::app()
        .register_mosaic_region_kind(
            MosaicRegionKindDescriptor::new(
                MosaicRegionKindId::new("workspace.region.primary")
                    .expect("valid mosaic region kind id"),
                MosaicRegionRole::primary(),
            )
            .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
            .with_scroll_ownership(MosaicScrollOwnership::region_owned())
            .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
            .with_child_rule(MosaicChildRule::accepts_surfaces())
            .with_allowed_surface_class(SurfacePlacementClass::primary_region())
            .with_persistence(MosaicRegionPersistence::restorable())
            .with_clipping(MosaicClippingPosture::clip_to_region())
            .with_hit_test(MosaicHitTestPosture::participates()),
        )
        .freeze().expect("application preparation should succeed");
}
