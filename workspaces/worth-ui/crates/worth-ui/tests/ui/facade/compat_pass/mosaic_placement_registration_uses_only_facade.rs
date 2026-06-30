use worth_ui::facade::{
    MosaicPlacementAction, MosaicPlacementConflictBehavior, MosaicPlacementPersistence,
    MosaicPlacementEligibility, MosaicPlacementPolicyDescriptor, MosaicPlacementPolicyId,
    MosaicPlacementReloadReconciliation, MosaicPlacementSource, MosaicPlacementSupport,
    MosaicPlacementTarget, MosaicRegionRole, MosaicStableIdentityBehavior, SurfacePlacementClass,
    WorthUi,
};

fn main() {
    let _app = WorthUi::app()
        .register_mosaic_placement_policy(
            MosaicPlacementPolicyDescriptor::new(
                MosaicPlacementPolicyId::new("workspace.placement.overlay")
                    .expect("valid mosaic placement policy id"),
            MosaicPlacementAction::overlay(),
            )
            .with_eligibility(MosaicPlacementEligibility::new(
                MosaicPlacementSource::surface_class(SurfacePlacementClass::overlay_layer()),
                MosaicPlacementTarget::region_role(MosaicRegionRole::overlay()),
            ))
            .with_persistence(MosaicPlacementPersistence::restorable())
            .with_stable_identity_behavior(
                MosaicStableIdentityBehavior::preserve_surface_identity(),
            )
            .with_conflict_behavior(MosaicPlacementConflictBehavior::reject_conflict())
            .with_reload_reconciliation(
                MosaicPlacementReloadReconciliation::restore_when_possible(),
            )
            .with_support(MosaicPlacementSupport::supported()),
        )
        .freeze();
}
