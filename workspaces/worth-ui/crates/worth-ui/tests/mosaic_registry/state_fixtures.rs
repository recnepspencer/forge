use worth_ui::facade::registry::{
    MosaicRegionKindId, MosaicStateOwnerIdentity, MosaicStateOwnerScopeId,
    MosaicStatePersistencePolicy, MosaicStateReplacementRule, MosaicStateSlotDescriptor,
    MosaicStateSlotId, MosaicStateSlotKind, MosaicStateTruthPosture, SurfaceId,
};

pub(crate) fn splitter_position_slot(id: &str) -> MosaicStateSlotDescriptor {
    complete_state_slot(id, MosaicStateSlotKind::splitter_position())
}

pub(crate) fn focused_region_slot(id: &str) -> MosaicStateSlotDescriptor {
    complete_state_slot(id, MosaicStateSlotKind::focused_region()).with_owner_identity(
        MosaicStateOwnerIdentity::runtime_scope(owner_scope_id("workspace.focus")),
    )
}

pub(crate) fn draft_input_slot(id: &str) -> MosaicStateSlotDescriptor {
    complete_state_slot(id, MosaicStateSlotKind::draft_input_state())
        .with_owner_identity(MosaicStateOwnerIdentity::surface(surface_id(
            "workspace.surface.editor",
        )))
        .with_replacement_rule(MosaicStateReplacementRule::discard_when_owner_changes())
}

pub(crate) fn complete_state_slot(
    id: &str,
    kind: MosaicStateSlotKind,
) -> MosaicStateSlotDescriptor {
    MosaicStateSlotDescriptor::new(state_slot_id(id), kind)
        .with_owner_identity(MosaicStateOwnerIdentity::mosaic_region_kind(
            mosaic_region_id("workspace.region.sidebar"),
        ))
        .with_persistence_policy(MosaicStatePersistencePolicy::restore_across_hot_reload())
        .with_replacement_rule(MosaicStateReplacementRule::preserve_when_owner_matches())
        .with_truth_posture(MosaicStateTruthPosture::ui_runtime_state())
}

pub(crate) fn state_slot_id(raw_text: &str) -> MosaicStateSlotId {
    MosaicStateSlotId::new(raw_text).expect("valid mosaic state slot id")
}

fn mosaic_region_id(raw_text: &str) -> MosaicRegionKindId {
    MosaicRegionKindId::new(raw_text).expect("valid mosaic region kind id")
}

fn surface_id(raw_text: &str) -> SurfaceId {
    SurfaceId::new(raw_text).expect("valid surface id")
}

fn owner_scope_id(raw_text: &str) -> MosaicStateOwnerScopeId {
    MosaicStateOwnerScopeId::new(raw_text).expect("valid mosaic state owner scope id")
}
