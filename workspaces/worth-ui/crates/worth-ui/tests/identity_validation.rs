use std::collections::{BTreeSet, HashSet};
use std::str::FromStr;

use worth_ui::facade::{
    declaration::{
        CommandId, CommandProjectionId, ComponentId, IconId, MosaicPlacementPolicyId,
        MosaicRegionKindId, MosaicSizingContractId, MosaicStateOwnerScopeId, MosaicStateSlotId,
        NativeCapabilityId, PluginSlotId, RuntimeOutcomeProjectionId, SettingId, SurfaceId,
        TaskPresentationId, ThemeTokenId, ViewBindingId,
    },
    support::CapabilityIdError,
};

#[test]
fn valid_id_text_preserves_canonical_text() {
    let command_id = CommandId::new("app.command.save").expect("valid command id");

    assert_eq!(command_id.as_str(), "app.command.save");
    assert_eq!(command_id.to_string(), "app.command.save");
    assert_eq!(format!("{command_id:?}"), "CommandId(\"app.command.save\")");
}

#[test]
fn valid_id_text_accepts_digits_and_underscores_after_segment_start() {
    let command_id =
        CommandId::new("app.command.save_2").expect("valid command id with segment suffixes");

    assert_eq!(command_id.as_str(), "app.command.save_2");
}

#[test]
fn every_registry_family_accepts_valid_identity_text() {
    assert_eq!(
        CommandId::new("app.command.save")
            .expect("valid command id")
            .as_str(),
        "app.command.save"
    );
    assert_eq!(
        ComponentId::new("app.component.editor")
            .expect("valid component id")
            .as_str(),
        "app.component.editor"
    );
    assert_eq!(
        SurfaceId::new("app.surface.main")
            .expect("valid surface id")
            .as_str(),
        "app.surface.main"
    );
    assert_eq!(
        MosaicRegionKindId::new("platform.mosaic_region.primary")
            .expect("valid mosaic region kind id")
            .as_str(),
        "platform.mosaic_region.primary"
    );
    assert_eq!(
        MosaicPlacementPolicyId::new("platform.mosaic_placement.docked")
            .expect("valid mosaic placement policy id")
            .as_str(),
        "platform.mosaic_placement.docked"
    );
    assert_eq!(
        MosaicSizingContractId::new("platform.mosaic_sizing.flex")
            .expect("valid mosaic sizing contract id")
            .as_str(),
        "platform.mosaic_sizing.flex"
    );
    assert_eq!(
        MosaicStateSlotId::new("app.mosaic_state.editor_tabs")
            .expect("valid mosaic state slot id")
            .as_str(),
        "app.mosaic_state.editor_tabs"
    );
    assert_eq!(
        MosaicStateOwnerScopeId::new("app.mosaic_state_owner.workspace")
            .expect("valid mosaic state owner scope id")
            .as_str(),
        "app.mosaic_state_owner.workspace"
    );
    assert_eq!(
        ViewBindingId::new("app.view_binding.tasks")
            .expect("valid view binding id")
            .as_str(),
        "app.view_binding.tasks"
    );
    assert_eq!(
        RuntimeOutcomeProjectionId::new("app.runtime_outcome.build")
            .expect("valid runtime outcome projection id")
            .as_str(),
        "app.runtime_outcome.build"
    );
    assert_eq!(
        SettingId::new("app.setting.theme")
            .expect("valid setting id")
            .as_str(),
        "app.setting.theme"
    );
    assert_eq!(
        TaskPresentationId::new("app.task_presentation.default")
            .expect("valid task presentation id")
            .as_str(),
        "app.task_presentation.default"
    );
    assert_eq!(
        ThemeTokenId::new("app.theme_token.accent")
            .expect("valid theme token id")
            .as_str(),
        "app.theme_token.accent"
    );
    assert_eq!(
        IconId::new("app.icon.save")
            .expect("valid icon id")
            .as_str(),
        "app.icon.save"
    );
    assert_eq!(
        CommandProjectionId::new("app.command_projection.toolbar")
            .expect("valid command projection id")
            .as_str(),
        "app.command_projection.toolbar"
    );
    assert_eq!(
        PluginSlotId::new("app.plugin_slot.theme")
            .expect("valid plugin slot id")
            .as_str(),
        "app.plugin_slot.theme"
    );
    assert_eq!(
        NativeCapabilityId::new("platform.native.clipboard")
            .expect("valid native capability id")
            .as_str(),
        "platform.native.clipboard"
    );
}

#[test]
fn equivalent_id_text_produces_equivalent_ids() {
    let first = CommandId::new("app.command.save").expect("valid command id");
    let second = CommandId::new("app.command.save").expect("valid command id");

    assert_eq!(first, second);
    assert_eq!(first.cmp(&second), core::cmp::Ordering::Equal);
}

#[test]
fn same_text_can_exist_in_distinct_id_families_without_unifying_identity() {
    let command_id = CommandId::new("app.shared.action").expect("valid command id");
    let component_id = ComponentId::new("app.shared.action").expect("valid component id");

    assert_eq!(command_id.as_str(), component_id.as_str());
    assert_eq!(command_id.to_string(), component_id.to_string());
}

#[test]
fn from_str_uses_the_same_validation_boundary_as_new() {
    let parsed = CommandId::from_str("app.command.save").expect("valid command id");

    assert_eq!(
        parsed,
        CommandId::new("app.command.save").expect("valid command id")
    );
    assert_eq!(
        CommandId::from_str("app.Command.save").unwrap_err(),
        CapabilityIdError::InvalidSegmentStart {
            byte_index: 4,
            found: 'C',
        }
    );
}

#[test]
fn different_valid_text_produces_different_ids() {
    let save = CommandId::new("app.command.save").expect("valid command id");
    let open = CommandId::new("app.command.open").expect("valid command id");

    assert_ne!(save, open);
    assert!(open < save);
}

#[test]
fn ids_are_stable_hash_and_order_keys() {
    let save = CommandId::new("app.command.save").expect("valid command id");
    let equivalent_save = CommandId::new("app.command.save").expect("valid command id");
    let open = CommandId::new("app.command.open").expect("valid command id");

    let mut hash_keys = HashSet::new();
    hash_keys.insert(save.clone());
    hash_keys.insert(equivalent_save);
    hash_keys.insert(open.clone());

    assert_eq!(hash_keys.len(), 2);
    assert!(hash_keys.contains(&save));

    let ordered_keys = BTreeSet::from([save, open]);
    assert_eq!(
        ordered_keys
            .iter()
            .map(CommandId::as_str)
            .collect::<Vec<_>>(),
        vec!["app.command.open", "app.command.save"]
    );
}

#[test]
fn invalid_id_text_rejected_before_descriptor_construction() {
    assert_eq!(CommandId::new("").unwrap_err(), CapabilityIdError::Empty);
    assert_eq!(
        CommandId::new("app..save").unwrap_err(),
        CapabilityIdError::EmptySegment { byte_index: 4 }
    );
    assert_eq!(
        CommandId::new(".app.save").unwrap_err(),
        CapabilityIdError::EmptySegment { byte_index: 0 }
    );
    assert_eq!(
        CommandId::new("app.save.").unwrap_err(),
        CapabilityIdError::EmptySegment { byte_index: 9 }
    );
    assert_eq!(
        CommandId::new("App.command.save").unwrap_err(),
        CapabilityIdError::InvalidSegmentStart {
            byte_index: 0,
            found: 'A',
        }
    );
    assert_eq!(
        CommandId::new("app.1command.save").unwrap_err(),
        CapabilityIdError::InvalidSegmentStart {
            byte_index: 4,
            found: '1',
        }
    );
    assert_eq!(
        CommandId::new("app._command.save").unwrap_err(),
        CapabilityIdError::InvalidSegmentStart {
            byte_index: 4,
            found: '_',
        }
    );
    assert_eq!(
        CommandId::new("app.écommand.save").unwrap_err(),
        CapabilityIdError::InvalidSegmentStart {
            byte_index: 4,
            found: 'é',
        }
    );
    assert_eq!(
        CommandId::new("app.command-save").unwrap_err(),
        CapabilityIdError::InvalidSegmentCharacter {
            byte_index: 11,
            found: '-',
        }
    );
    assert_eq!(
        CommandId::new("app.command save").unwrap_err(),
        CapabilityIdError::InvalidSegmentCharacter {
            byte_index: 11,
            found: ' ',
        }
    );
    assert_eq!(
        CommandId::new("app.command/save").unwrap_err(),
        CapabilityIdError::InvalidSegmentCharacter {
            byte_index: 11,
            found: '/',
        }
    );
}
