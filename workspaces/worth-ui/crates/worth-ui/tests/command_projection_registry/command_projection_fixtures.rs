use worth_ui::facade::registry::{
    CommandCategory, CommandDescriptor, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface,
    MosaicPlacementAction, MosaicPlacementConflictBehavior, MosaicPlacementEligibility,
    MosaicPlacementPersistence, MosaicPlacementPolicyDescriptor, MosaicPlacementPolicyId,
    MosaicPlacementReloadReconciliation, MosaicPlacementSource, MosaicPlacementSupport,
    MosaicPlacementTarget, MosaicRegionRole, MosaicStableIdentityBehavior, SurfacePlacementClass,
};

pub(crate) fn command_projection(id: &str) -> CommandProjectionDescriptor {
    CommandProjectionDescriptor::new(
        command_projection_id(id),
        CommandProjectionSurface::command_palette(),
    )
    .with_eligible_category(CommandCategory::Workspace)
}

pub(crate) fn command_projection_for_command(
    id: &str,
    command: &str,
) -> CommandProjectionDescriptor {
    CommandProjectionDescriptor::new(
        command_projection_id(id),
        CommandProjectionSurface::toolbar(),
    )
    .with_command_reference(CommandProjectionCommandReference::command(command_id(
        command,
    )))
}

pub(crate) fn command_descriptor(id: &str, label: &str) -> CommandDescriptor {
    CommandDescriptor::new(command_id(id), label).with_category(CommandCategory::Workspace)
}

pub(crate) fn command_id(raw_text: &str) -> CommandId {
    CommandId::new(raw_text).expect("valid command id")
}

pub(crate) fn command_projection_id(raw_text: &str) -> CommandProjectionId {
    CommandProjectionId::new(raw_text).expect("valid command projection id")
}

pub(crate) fn mosaic_placement_policy(id: &str) -> MosaicPlacementPolicyDescriptor {
    MosaicPlacementPolicyDescriptor::new(
        mosaic_placement_policy_id(id),
        MosaicPlacementAction::dock(),
    )
    .with_eligibility(MosaicPlacementEligibility::new(
        MosaicPlacementSource::surface_class(SurfacePlacementClass::primary_region()),
        MosaicPlacementTarget::region_role(MosaicRegionRole::primary()),
    ))
    .with_persistence(MosaicPlacementPersistence::restorable())
    .with_stable_identity_behavior(MosaicStableIdentityBehavior::preserve_surface_identity())
    .with_conflict_behavior(MosaicPlacementConflictBehavior::reject_conflict())
    .with_reload_reconciliation(MosaicPlacementReloadReconciliation::restore_when_possible())
    .with_support(MosaicPlacementSupport::supported())
}

pub(crate) fn mosaic_placement_policy_id(raw_text: &str) -> MosaicPlacementPolicyId {
    MosaicPlacementPolicyId::new(raw_text).expect("valid mosaic placement id")
}
