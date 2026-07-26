use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        MosaicPlacementAction, MosaicPlacementEligibility, MosaicPlacementSource,
        MosaicPlacementSupport, MosaicPlacementTarget, MosaicRegionRole, SurfacePlacementClass,
    },
};

use super::mosaic_placement_registry_fixtures::{complete_policy, placement_id};

#[test]
fn overlay_policy_with_explicit_runtime_support_is_admitted() {
    let app = WorthUi::app()
        .register_mosaic_placement_policy(
            complete_policy(
                "workspace.placement.overlay",
                MosaicPlacementAction::overlay(),
            )
            .with_source(MosaicPlacementSource::surface_class(
                SurfacePlacementClass::overlay_layer(),
            ))
            .with_target(MosaicPlacementTarget::region_role(
                MosaicRegionRole::overlay(),
            ))
            .with_support(MosaicPlacementSupport::supported())
            .with_label("Overlay placement"),
        )
        .freeze()
        .expect("application preparation should succeed");

    let descriptor = app
        .capabilities()
        .mosaic_placement_policies()
        .get(&placement_id("workspace.placement.overlay"))
        .expect("registered mosaic placement policy");
    assert_eq!(descriptor.action(), &MosaicPlacementAction::overlay());
    assert_eq!(
        descriptor.eligibility(),
        Some(MosaicPlacementEligibility::new(
            MosaicPlacementSource::surface_class(SurfacePlacementClass::overlay_layer()),
            MosaicPlacementTarget::region_role(MosaicRegionRole::overlay())
        ))
    );
    assert_eq!(descriptor.label(), Some("Overlay placement"));
}
