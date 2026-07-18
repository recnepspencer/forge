use worth_ui::facade::{
    MosaicRegionKindId, MosaicStateOwnerIdentity, MosaicStatePersistencePolicy,
    MosaicStateReplacementRule, MosaicStateSlotDescriptor, MosaicStateSlotId,
    MosaicStateSlotKind, MosaicStateTruthPosture, WorthUi,
};

fn main() {
    let _app = WorthUi::app()
        .register_mosaic_state_slot(
            MosaicStateSlotDescriptor::new(
                MosaicStateSlotId::new("workspace.state.sidebar_width").unwrap(),
                MosaicStateSlotKind::splitter_position(),
            )
            .with_owner_identity(MosaicStateOwnerIdentity::mosaic_region_kind(
                MosaicRegionKindId::new("workspace.region.sidebar").unwrap(),
            ))
            .with_persistence_policy(MosaicStatePersistencePolicy::restore_across_hot_reload())
            .with_replacement_rule(MosaicStateReplacementRule::preserve_when_owner_matches())
            .with_truth_posture(MosaicStateTruthPosture::ui_runtime_state()),
        )
        .freeze().expect("application preparation should succeed");
}
