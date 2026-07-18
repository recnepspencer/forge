use worth_ui::facade::{
    CommandProjectionId, MosaicPlacementPolicyId, MosaicRegionKindId, MosaicSizingContractId,
    MosaicStateSlotId, PluginSlotId, RuntimeOutcomeProjectionId, SettingId, SurfaceId,
    TaskPresentationId, ThemeTokenId, ViewBindingId, WorthUi,
};

use super::snapshot_fixtures::{command_icon, command_id, command_with_icon};

#[test]
fn snapshot_lookup_by_typed_id_is_index_backed() {
    let app = WorthUi::app()
        .register_icon(command_icon("icon.save"))
        .register_command(command_with_icon("command.save", "icon.save"))
        .register_command(command_with_icon("command.open", "icon.save"))
        .freeze()
        .expect("application preparation should succeed");

    let lookup = app
        .capabilities()
        .index()
        .commands()
        .lookup(&command_id("command.save"));

    assert!(lookup.is_found());
    assert_eq!(
        lookup.value().expect("command descriptor").id(),
        &command_id("command.save")
    );
    assert_eq!(lookup.counters().family_width(), 2);
    assert_eq!(lookup.counters().families_scanned(), 0);
}

#[test]
fn snapshot_lookup_index_covers_every_frozen_family() {
    let app = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let index = app.capabilities().index();

    assert_index_backed(
        index
            .commands()
            .lookup(&command_id("command.missing"))
            .counters(),
    );
    assert_index_backed(
        index
            .command_projections()
            .lookup(&CommandProjectionId::new("command_projection.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .components()
            .lookup(&super::snapshot_fixtures::component_id("component.missing"))
            .counters(),
    );
    assert_index_backed(
        index
            .icons()
            .lookup(&super::snapshot_fixtures::icon_id("icon.missing"))
            .counters(),
    );
    assert_index_backed(
        index
            .surfaces()
            .lookup(&SurfaceId::new("surface.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .mosaic_regions()
            .lookup(&MosaicRegionKindId::new("mosaic_region.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .mosaic_placement_policies()
            .lookup(&MosaicPlacementPolicyId::new("mosaic_placement.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .mosaic_sizing_contracts()
            .lookup(&MosaicSizingContractId::new("mosaic_sizing.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .mosaic_state_slots()
            .lookup(&MosaicStateSlotId::new("mosaic_state.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .native_capabilities()
            .lookup(&super::snapshot_fixtures::native_capability_id(
                "native.missing",
            ))
            .counters(),
    );
    assert_index_backed(
        index
            .plugin_slots()
            .lookup(&PluginSlotId::new("plugin_slot.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .view_bindings()
            .lookup(&ViewBindingId::new("view_binding.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .runtime_outcome_projections()
            .lookup(&RuntimeOutcomeProjectionId::new("runtime_outcome.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .settings()
            .lookup(&SettingId::new("setting.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .task_presentations()
            .lookup(&TaskPresentationId::new("task_presentation.missing").unwrap())
            .counters(),
    );
    assert_index_backed(
        index
            .theme_tokens()
            .lookup(&ThemeTokenId::new("theme.missing").unwrap())
            .counters(),
    );
}

fn assert_index_backed(counters: worth_ui::facade::SnapshotLookupCounters) {
    assert_eq!(counters.family_width(), 0);
    assert_eq!(counters.families_scanned(), 0);
}
