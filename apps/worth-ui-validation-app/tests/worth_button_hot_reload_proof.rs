use worth_ui::facade::{
    SurfaceId, WorthUiComponentInteractionKind, WorthUiInlineContentItem,
    WorthUiRuntimeChangeFamily, WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

#[test]
fn authored_button_surface_resolves_frame_without_local_interaction_payload() {
    let prepared = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::sample())
        .expect("validation workbench should prepare");
    let surface_id = button_surface_id();

    let frame = prepared
        .runtime()
        .resolve_button_frame(&surface_id)
        .expect("button frame should resolve from authored source");

    assert_eq!(frame.surface_id(), "worth.surface.preview.button.proof");
    assert_eq!(frame.component_id(), "worth.component.button");
    assert_eq!(frame.label(), "Submit");
    assert_eq!(
        frame.icon().map(|icon| icon.icon_id()),
        Some("worth.icon.action.plus")
    );
    assert_eq!(frame.icon().map(|icon| icon.source_key()), Some("plus"));
    assert_eq!(frame.fill(), "#2f7de1");
    assert_eq!(frame.pressed_style().background_color(), "#ffffff");
    assert_eq!(frame.pressed_style().foreground_color(), "#2f7de1");
    assert_eq!(frame.pressed_style().border_color(), "#2f7de1");
    assert_eq!(frame.pressed_style().border_width_points(), 2.0);
    assert_eq!(frame.style().icon_color(), "#f7f1e8");
    assert_eq!(frame.style().border_width_points(), 0.0);
    assert_eq!(frame.style().border_radius_points(), 14.0);
    assert_eq!(frame.width_points(), 280.0);
    assert_eq!(frame.content().gap_points(), 4.0);
    assert!(frame.content().items().iter().any(|item| matches!(
        item,
        WorthUiInlineContentItem::Icon(icon)
            if icon.size_points() == 32.0
                && icon.stroke_width_points() == 1.6
                && icon.pressed_style().color() == "#2f7de1"
    )));
    assert!(frame.content().items().iter().any(|item| matches!(
        item,
        WorthUiInlineContentItem::Text(text) if text.size_points() == 15.0
    )));
}

#[test]
fn button_submit_enters_generic_component_interaction_lane() {
    let mut workbench = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::sample())
        .expect("validation workbench should prepare")
        .into_runtime_workbench();
    let surface_id = button_surface_id();

    let receipt = workbench
        .submit_component_interaction(&surface_id, WorthUiComponentInteractionKind::Submit)
        .expect("button submit should enter the runtime interaction lane");
    assert_eq!(receipt.interaction_id(), "worth.interaction.button.submit");
    assert_eq!(
        receipt
            .payload()
            .field("payload")
            .expect("button receipt carries authored payload")
            .as_text(),
        "authored submit payload"
    );
    let admitted = workbench
        .runtime()
        .admit_component_interaction_runtime_change(&receipt)
        .expect("component interaction should admit as runtime evidence");

    let row = admitted
        .family_rows()
        .first()
        .expect("interaction evidence should have a row");
    assert_eq!(row.family(), WorthUiRuntimeChangeFamily::InteractionState);
    assert!(row
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::ComponentInteractionState));
}

fn button_surface_id() -> SurfaceId {
    SurfaceId::new("worth.surface.preview.button.proof").expect("valid button surface id")
}
