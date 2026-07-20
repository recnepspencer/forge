use worth_ui::facade::{
    diagnostics::{CapabilitySnapshot, SnapshotLookupReport},
    registry::{
        CommandId, ComponentId, IconId, MosaicPlacementPolicyId, MosaicRegionKindId,
        MosaicSizingContractId, MosaicStateSlotId, PluginSlotId, SurfaceId, ThemeTokenId,
    },
};

pub(crate) fn assert_minimal_app_snapshot_names_registered_capabilities(
    snapshot: &CapabilitySnapshot,
) {
    let index = snapshot.index();

    assert_index_lookup_found(index.commands().lookup(&command_id()));
    assert_minimal_app_snapshot_preserves_non_command_capabilities(snapshot);
}

pub(crate) fn assert_minimal_app_snapshot_preserves_non_command_capabilities(
    snapshot: &CapabilitySnapshot,
) {
    let index = snapshot.index();

    assert_index_lookup_found(index.components().lookup(&component_id()));
    assert_index_lookup_found(index.surfaces().lookup(&surface_id()));
    assert_index_lookup_found(index.mosaic_regions().lookup(&mosaic_region_id()));
    assert_index_lookup_found(
        index
            .mosaic_placement_policies()
            .lookup(&mosaic_placement_policy_id()),
    );
    assert_index_lookup_found(
        index
            .mosaic_sizing_contracts()
            .lookup(&mosaic_sizing_contract_id()),
    );
    assert_index_lookup_found(index.mosaic_state_slots().lookup(&mosaic_state_slot_id()));
    assert_index_lookup_found(index.theme_tokens().lookup(&theme_token_id()));
    assert_index_lookup_found(index.icons().lookup(&icon_id()));
    assert_index_lookup_found(index.plugin_slots().lookup(&plugin_slot_id()));
}

pub(crate) fn assert_minimal_app_snapshot_rejects_duplicate_command(snapshot: &CapabilitySnapshot) {
    assert_index_lookup_missing(snapshot.index().commands().lookup(&command_id()), 0);
}

pub(crate) fn assert_minimal_app_snapshot_does_not_name_raw_sizing_contract(
    snapshot: &CapabilitySnapshot,
) {
    assert_index_lookup_missing(
        snapshot
            .index()
            .mosaic_sizing_contracts()
            .lookup(&raw_sizing_contract_id()),
        1,
    );
}

pub(crate) fn assert_minimal_app_snapshot_does_not_name_illegal_placement_policy(
    snapshot: &CapabilitySnapshot,
) {
    assert_index_lookup_missing(
        snapshot
            .index()
            .mosaic_placement_policies()
            .lookup(&illegal_placement_policy_id()),
        1,
    );
}

fn assert_index_lookup_found<T>(lookup: SnapshotLookupReport<&T>) {
    assert!(lookup.is_found());
    assert_eq!(lookup.counters().family_width(), 1);
    assert_eq!(lookup.counters().families_scanned(), 0);
}

fn assert_index_lookup_missing<T>(lookup: SnapshotLookupReport<&T>, expected_family_width: usize) {
    assert!(!lookup.is_found());
    assert_eq!(lookup.counters().family_width(), expected_family_width);
    assert_eq!(lookup.counters().families_scanned(), 0);
}

fn command_id() -> CommandId {
    CommandId::new("minimal.command.save").expect("valid command id")
}

fn component_id() -> ComponentId {
    ComponentId::new("minimal.component.editor").expect("valid component id")
}

fn surface_id() -> SurfaceId {
    SurfaceId::new("minimal.surface.editor").expect("valid surface id")
}

fn mosaic_region_id() -> MosaicRegionKindId {
    MosaicRegionKindId::new("minimal.region.primary").expect("valid mosaic region id")
}

fn mosaic_placement_policy_id() -> MosaicPlacementPolicyId {
    MosaicPlacementPolicyId::new("minimal.placement.primary").expect("valid placement id")
}

fn illegal_placement_policy_id() -> MosaicPlacementPolicyId {
    MosaicPlacementPolicyId::new("minimal.placement.illegal").expect("valid placement id")
}

fn mosaic_sizing_contract_id() -> MosaicSizingContractId {
    MosaicSizingContractId::new("minimal.sizing.primary").expect("valid sizing id")
}

fn raw_sizing_contract_id() -> MosaicSizingContractId {
    MosaicSizingContractId::new("minimal.sizing.raw").expect("valid sizing id")
}

fn mosaic_state_slot_id() -> MosaicStateSlotId {
    MosaicStateSlotId::new("minimal.state.splitter").expect("valid state slot id")
}

fn theme_token_id() -> ThemeTokenId {
    ThemeTokenId::new("minimal.theme.text").expect("valid theme token id")
}

fn icon_id() -> IconId {
    IconId::new("minimal.icon.save").expect("valid icon id")
}

fn plugin_slot_id() -> PluginSlotId {
    PluginSlotId::new("minimal.plugin_slot.commands").expect("valid plugin slot id")
}
