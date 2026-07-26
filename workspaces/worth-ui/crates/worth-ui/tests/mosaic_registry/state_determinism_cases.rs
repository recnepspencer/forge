use worth_ui::facade::{
    app::WorthUi,
    declaration::{MosaicStateSlotKind, MosaicStateTruthPosture},
};

use super::state_assertions::{
    assert_reconciliation_keys, assert_registered_mosaic_state_slot_ids,
};
use super::state_fixtures::{
    complete_state_slot, draft_input_slot, focused_region_slot, splitter_position_slot,
};

#[test]
fn equivalent_state_slots_produce_equivalent_reconciliation_keys() {
    let first = WorthUi::app()
        .register_mosaic_state_slot(splitter_position_slot("workspace.state.sidebar_width"))
        .register_mosaic_state_slot(focused_region_slot("workspace.state.focused_region"))
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .register_mosaic_state_slot(focused_region_slot("workspace.state.focused_region"))
        .register_mosaic_state_slot(splitter_position_slot("workspace.state.sidebar_width"))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        first.capabilities().mosaic_state_slots(),
        second.capabilities().mosaic_state_slots()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_reconciliation_keys(
        first.capabilities().mosaic_state_slots(),
        &[
            "workspace.state.focused_region|focused_region|runtime_scope:workspace.focus|restore_across_hot_reload|preserve_when_owner_matches|ui_runtime_state",
            "workspace.state.sidebar_width|splitter_position|mosaic_region_kind:workspace.region.sidebar|restore_across_hot_reload|preserve_when_owner_matches|ui_runtime_state",
        ],
    );
}

#[test]
fn truth_posture_participates_in_reconciliation_key_equivalence() {
    let ui_runtime = WorthUi::app()
        .register_mosaic_state_slot(splitter_position_slot("workspace.state.sidebar_width"))
        .freeze()
        .expect("application preparation should succeed");
    let derived_runtime = WorthUi::app()
        .register_mosaic_state_slot(
            splitter_position_slot("workspace.state.sidebar_width").with_truth_posture(
                MosaicStateTruthPosture::derived_from_authoritative_runtime_truth(),
            ),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        ui_runtime.capabilities().mosaic_state_slots().entries(),
        derived_runtime
            .capabilities()
            .mosaic_state_slots()
            .entries()
    );
}

#[test]
fn different_state_slot_meaning_changes_snapshot_digest() {
    let splitter = WorthUi::app()
        .register_mosaic_state_slot(splitter_position_slot("workspace.state.sidebar"))
        .freeze()
        .expect("application preparation should succeed");
    let scroll = WorthUi::app()
        .register_mosaic_state_slot(complete_state_slot(
            "workspace.state.sidebar",
            MosaicStateSlotKind::scroll_position(),
        ))
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        splitter.capabilities().mosaic_state_slots(),
        scroll.capabilities().mosaic_state_slots()
    );
    assert_ne!(
        splitter.capabilities().digest(),
        scroll.capabilities().digest()
    );
}

#[test]
fn accepted_state_slots_remain_inspectable_after_freeze() {
    let app = WorthUi::app()
        .register_mosaic_state_slot(draft_input_slot("workspace.state.editor_draft"))
        .freeze()
        .expect("application preparation should succeed");
    let descriptor = app.capabilities().mosaic_state_slots().entries()[0].descriptor();

    assert_eq!(descriptor.kind(), &MosaicStateSlotKind::draft_input_state());
    assert_registered_mosaic_state_slot_ids(
        app.capabilities().mosaic_state_slots(),
        &["workspace.state.editor_draft"],
    );
}
