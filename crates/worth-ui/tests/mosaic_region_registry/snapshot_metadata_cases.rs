use super::*;

#[test]
fn mosaic_region_metadata_survives_freeze() {
    let app = WorthUi::app()
        .register_mosaic_region_kind(
            MosaicRegionKindDescriptor::new(
                mosaic_region_id("workspace.region.overlay"),
                MosaicRegionRole::overlay(),
            )
            .with_sizing_behavior(MosaicSizingBehavior::overlay_anchored())
            .with_scroll_ownership(MosaicScrollOwnership::surface_owned())
            .with_focus_scope(MosaicFocusScopeKind::modal_trap_scope())
            .with_child_rule(MosaicChildRule::accepts_region_stack())
            .with_allowed_surface_class(SurfacePlacementClass::overlay_layer())
            .with_persistence(MosaicRegionPersistence::ephemeral())
            .with_clipping(MosaicClippingPosture::allow_overlay_escape())
            .with_hit_test(MosaicHitTestPosture::modal_capture())
            .with_label("Overlay"),
        )
        .freeze();

    let descriptor = app
        .capabilities()
        .mosaic_regions()
        .get(&mosaic_region_id("workspace.region.overlay"))
        .expect("registered mosaic region");
    assert_eq!(descriptor.role(), &MosaicRegionRole::overlay());
    assert_eq!(
        descriptor.allowed_surface_classes(),
        &[SurfacePlacementClass::overlay_layer()]
    );
    assert_eq!(descriptor.label(), Some("Overlay"));
}
