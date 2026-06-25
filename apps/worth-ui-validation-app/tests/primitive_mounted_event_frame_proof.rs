mod validation_app_reload_fixture;

use validation_app_reload_fixture::ValidationAppReloadFixture;
use worth_ui::facade::{
    WorthUiPrimitiveEventDispatchOutcome, WorthUiPrimitiveEventHitTestPoint,
    WorthUiPrimitivePointerCaptureHostSupport, WorthUiPrimitivePointerCaptureState,
    WorthUiPrimitivePointerFrameInput, WorthUiPrimitivePointerPhase,
    WorthUiPrimitiveResolvedCursorPosture,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;
use worth_ui_validation_app::ValidationMountedPrimitiveEventViewport;

const OUTER_SURFACE: &str = "worth.surface.preview.primitive.proof";
const INNER_SURFACE: &str = "worth.surface.preview.primitive.inner";

#[test]
fn mounted_pointer_frame_reports_cursor_and_inner_click_through_app_boundary() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    enable_inner_surface(&mut app);
    let viewport = ValidationMountedPrimitiveEventViewport::new(900.0, 600.0);
    let inner_point = mounted_inner_center(&app, viewport);
    let hover = app
        .mounted_primitive_event_frame_for_proof(
            viewport,
            WorthUiPrimitivePointerFrameInput::hover(inner_point),
        )
        .expect("mounted hover frame resolves");
    let release = app
        .mounted_primitive_event_frame_for_proof(
            viewport,
            pointer_input(inner_point, WorthUiPrimitivePointerPhase::Release),
        )
        .expect("mounted release frame resolves");

    assert_eq!(
        hover.pointer_frame().dispatch().cursor(),
        WorthUiPrimitiveResolvedCursorPosture::Pointer
    );
    assert_eq!(
        release.pointer_frame().dispatch().primary_surface_id(),
        Some(INNER_SURFACE)
    );
    assert_eq!(
        release.pointer_frame().dispatch().emitted_surface_ids(),
        &[INNER_SURFACE.to_owned()]
    );
}

#[test]
fn mounted_bubble_and_disabled_paths_are_receipt_distinct() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    enable_inner_surface(&mut app);
    let viewport = ValidationMountedPrimitiveEventViewport::new(900.0, 600.0);
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        INNER_SURFACE,
        "event_containment",
        "bubble",
    ))
    .expect("inner containment edit applies");
    let inner_point = mounted_inner_center(&app, viewport);
    let bubbled = app
        .mounted_primitive_event_frame_for_proof(
            viewport,
            pointer_input(inner_point, WorthUiPrimitivePointerPhase::Release),
        )
        .expect("mounted bubble frame resolves");

    assert!(matches!(
        bubbled.pointer_frame().dispatch().outcome(),
        WorthUiPrimitiveEventDispatchOutcome::Bubbled(_)
    ));
    assert_eq!(
        bubbled.pointer_frame().dispatch().emitted_surface_ids(),
        &[INNER_SURFACE.to_owned(), OUTER_SURFACE.to_owned()]
    );

    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        INNER_SURFACE,
        "interaction_readiness",
        "disabled",
    ))
    .expect("disabled readiness edit applies");
    let disabled = app
        .mounted_primitive_event_frame_for_proof(
            viewport,
            pointer_input(inner_point, WorthUiPrimitivePointerPhase::Release),
        )
        .expect("mounted disabled frame resolves");

    assert!(matches!(
        disabled.pointer_frame().dispatch().outcome(),
        WorthUiPrimitiveEventDispatchOutcome::DisabledHit(_)
    ));
    assert_eq!(
        disabled
            .pointer_frame()
            .dispatch()
            .query_graph_execution()
            .selected_obligation_count(),
        6
    );
    assert!(disabled
        .pointer_frame()
        .dispatch()
        .emitted_surface_ids()
        .is_empty());
}

#[test]
fn mounted_press_drag_capture_routes_drag_to_captured_surface() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    enable_inner_surface(&mut app);
    let viewport = ValidationMountedPrimitiveEventViewport::new(900.0, 600.0);
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        INNER_SURFACE,
        "event_capture",
        "press_drag",
    ))
    .expect("press drag capture edit applies");
    let inner_point = mounted_inner_center(&app, viewport);
    let press = app
        .mounted_primitive_event_frame_for_proof(
            viewport,
            WorthUiPrimitivePointerFrameInput::new(
                inner_point,
                WorthUiPrimitivePointerPhase::Press,
                WorthUiPrimitivePointerCaptureState::Uncaptured,
                WorthUiPrimitivePointerCaptureHostSupport::Certified,
            ),
        )
        .expect("mounted press frame resolves");
    let prior_capture = press.pointer_frame().capture_state().clone();
    let drag = app
        .mounted_primitive_event_frame_for_proof(
            viewport,
            WorthUiPrimitivePointerFrameInput::new(
                WorthUiPrimitiveEventHitTestPoint::new(1.0, 1.0),
                WorthUiPrimitivePointerPhase::Drag,
                prior_capture,
                WorthUiPrimitivePointerCaptureHostSupport::Certified,
            ),
        )
        .expect("mounted drag frame resolves");

    assert_eq!(
        press.pointer_frame().capture_state().captured_surface_id(),
        Some(INNER_SURFACE)
    );
    assert_eq!(
        drag.pointer_frame().dispatch().primary_surface_id(),
        Some(INNER_SURFACE)
    );
    assert_eq!(
        drag.pointer_frame().dispatch().emitted_surface_ids(),
        &[INNER_SURFACE.to_owned()]
    );
}

fn mounted_inner_center(
    app: &worth_ui_validation_app::ValidationWorkbenchApp,
    viewport: ValidationMountedPrimitiveEventViewport,
) -> WorthUiPrimitiveEventHitTestPoint {
    let seed = app
        .mounted_primitive_event_frame_for_proof(
            viewport,
            WorthUiPrimitivePointerFrameInput::hover(WorthUiPrimitiveEventHitTestPoint::new(
                0.0, 0.0,
            )),
        )
        .expect("mounted event plan resolves");
    let inner = seed
        .event_plan()
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner mounted event region exists");
    WorthUiPrimitiveEventHitTestPoint::new(
        inner.hit_frame().x() + inner.hit_frame().width() * 0.5,
        inner.hit_frame().y() + inner.hit_frame().height() * 0.5,
    )
}

fn pointer_input(
    point: WorthUiPrimitiveEventHitTestPoint,
    phase: WorthUiPrimitivePointerPhase,
) -> WorthUiPrimitivePointerFrameInput {
    WorthUiPrimitivePointerFrameInput::new(
        point,
        phase,
        WorthUiPrimitivePointerCaptureState::Uncaptured,
        WorthUiPrimitivePointerCaptureHostSupport::Certified,
    )
}

fn enable_inner_surface(app: &mut worth_ui_validation_app::ValidationWorkbenchApp) {
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        INNER_SURFACE,
        "primitive_disabled",
        "false",
    ))
    .expect("inner primitive disabled flag edit applies");
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        INNER_SURFACE,
        "interaction_readiness",
        "enabled",
    ))
    .expect("inner interaction readiness edit applies");
}
