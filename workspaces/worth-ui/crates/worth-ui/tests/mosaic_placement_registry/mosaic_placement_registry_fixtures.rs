use worth_ui::facade::{
    MosaicPlacementAction, MosaicPlacementConflictBehavior, MosaicPlacementPersistence,
    MosaicPlacementPolicyDescriptor, MosaicPlacementPolicyId, MosaicPlacementReloadReconciliation,
    MosaicPlacementSource, MosaicPlacementTarget, MosaicRegionRole, MosaicStableIdentityBehavior,
    SurfacePlacementClass,
};

pub(crate) fn primary_dock_policy(id: &str) -> MosaicPlacementPolicyDescriptor {
    complete_policy(id, MosaicPlacementAction::dock())
        .with_source(MosaicPlacementSource::surface_class(
            SurfacePlacementClass::primary_region(),
        ))
        .with_target(MosaicPlacementTarget::region_role(
            MosaicRegionRole::primary(),
        ))
}

pub(crate) fn auxiliary_dock_policy(id: &str) -> MosaicPlacementPolicyDescriptor {
    complete_policy(id, MosaicPlacementAction::dock())
        .with_source(MosaicPlacementSource::surface_class(
            SurfacePlacementClass::auxiliary_region(),
        ))
        .with_target(MosaicPlacementTarget::region_role(MosaicRegionRole::side()))
}

pub(crate) fn complete_policy(
    id: &str,
    action: MosaicPlacementAction,
) -> MosaicPlacementPolicyDescriptor {
    MosaicPlacementPolicyDescriptor::new(placement_id(id), action)
        .with_source(MosaicPlacementSource::surface_class(
            SurfacePlacementClass::primary_region(),
        ))
        .with_target(MosaicPlacementTarget::region_role(
            MosaicRegionRole::primary(),
        ))
        .with_persistence(MosaicPlacementPersistence::restorable())
        .with_stable_identity_behavior(MosaicStableIdentityBehavior::preserve_surface_identity())
        .with_conflict_behavior(MosaicPlacementConflictBehavior::reject_conflict())
        .with_reload_reconciliation(MosaicPlacementReloadReconciliation::restore_when_possible())
}

pub(crate) fn placement_id(raw_text: &str) -> MosaicPlacementPolicyId {
    MosaicPlacementPolicyId::new(raw_text).expect("valid mosaic placement policy id")
}
