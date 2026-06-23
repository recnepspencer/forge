mod primitive_appearance_state_basis_support;
mod primitive_appearance_state_reload_support;

use primitive_appearance_state_reload_support::{
    activate_appearance_state_edits, assert_exact_appearance_state_projection_rows,
    changed_fact_mapping, launch_stable_workbench, prepare_reload_for_edits, PRIMITIVE_SURFACE,
};
use worth_ui::facade::{
    WorthUiAppearanceStateName, WorthUiPrimitiveObservedPostureReceipt,
    WorthUiPrimitiveProjectionRebindStatus, WorthUiRuntimeFactFamily, WorthUiSemanticSliceId,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

#[test]
fn appearance_state_color_edit_rebinds_only_state_projection_consumers() {
    let edit = ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "appearance_rest_background",
        "\"#b3261e\"",
    );
    let projection = activate_appearance_state_edits(&[edit]);
    let active = projection
        .primitive_receipt()
        .appearance_state()
        .resolve_active(worth_ui::facade::WorthUiAppearanceStatePosture::rest());

    assert_eq!(
        projection.rebind_status(),
        WorthUiPrimitiveProjectionRebindStatus::Rebound
    );
    assert_exact_families(
        projection.rebind_plan().rebuilt_facts(),
        &[
            WorthUiRuntimeFactFamily::AuthoredSurfaceProps,
            WorthUiRuntimeFactFamily::PrimitiveAppearanceState,
        ],
    );
    assert_exact_families(
        projection.rebind_plan().preserved_facts(),
        &[
            WorthUiRuntimeFactFamily::PrimitiveContent,
            WorthUiRuntimeFactFamily::PrimitiveContainer,
            WorthUiRuntimeFactFamily::PrimitiveMeasurement,
            WorthUiRuntimeFactFamily::PrimitiveAppearance,
            WorthUiRuntimeFactFamily::PrimitiveInteraction,
            WorthUiRuntimeFactFamily::PrimitiveEventGeometry,
            WorthUiRuntimeFactFamily::PrimitiveMotion,
            WorthUiRuntimeFactFamily::PrimitiveFlowLayout,
        ],
    );
    assert_eq!(active.background_color().hex_triplet(), "#b3261e");
    assert_exact_appearance_state_projection_rows(projection.changed_rows());
}

#[test]
fn paint_plan_consumes_resolved_active_appearance_receipt() {
    let projection = activate_appearance_state_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_pressed_background",
            "\"#ffffff\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_pressed_text_color",
            "\"#b3261e\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_focus_border_color",
            "\"#444444\"",
        ),
    ]);
    let paint_plan = projection.primitive_receipt().paint_plan(
        1000.0,
        600.0,
        WorthUiPrimitiveObservedPostureReceipt::from_renderer_observation(
            true, true, true, false, false,
        ),
    );
    let active = paint_plan.active_appearance();

    assert_eq!(active.background_color().hex_triplet(), "#ffffff");
    assert_eq!(active.text_color().hex_triplet(), "#b3261e");
    assert_eq!(active.border_color().hex_triplet(), "#444444");
    assert_eq!(
        active.active_states(),
        &[
            WorthUiAppearanceStateName::Rest,
            WorthUiAppearanceStateName::Hover,
            WorthUiAppearanceStateName::Pressed,
            WorthUiAppearanceStateName::Focus,
        ]
    );
    assert_eq!(paint_plan.draw_plan().item_frames().len(), 2);
}

#[test]
fn token_backed_colors_and_typography_resolve_into_renderer_ready_receipt() {
    let projection = activate_appearance_state_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_rest_background",
            "validation.theme.header.menu.hover",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_rest_typography",
            "validation.appearance.header.font_size",
        ),
    ]);
    let active = projection.primitive_receipt().paint_plan(
        1000.0,
        600.0,
        WorthUiPrimitiveObservedPostureReceipt::rest(),
    );

    assert_eq!(
        active.active_appearance().background_color().hex_triplet(),
        "#3e3e42"
    );
    assert_eq!(
        active.active_appearance().typography_token(),
        "validation.appearance.header.font_size"
    );
    assert_eq!(active.active_appearance().text_size_points(), 13.0);
}

#[test]
fn removing_state_override_falls_back_to_rest_recipe() {
    let projection = activate_appearance_state_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_pressed_text_color",
            "\"#b3261e\"",
        ),
        ValidationAuthoredReloadEdit::remove_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_pressed_text_color",
        ),
    ]);
    let active = projection.primitive_receipt().paint_plan(
        1000.0,
        600.0,
        WorthUiPrimitiveObservedPostureReceipt::from_renderer_observation(
            false, true, false, false, false,
        ),
    );

    assert_eq!(
        active.active_appearance().text_color().hex_triplet(),
        "#cccccc"
    );
}

#[test]
fn all_state_posture_combinations_resolve_with_declared_precedence() {
    let projection = activate_appearance_state_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_hover_background",
            "\"#333333\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_pressed_background",
            "\"#222222\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_focus_border_color",
            "\"#444444\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_disabled_opacity",
            "\"0.25\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_selected_background",
            "\"#555555\"",
        ),
    ]);

    let hover_focus = active_for(&projection, true, false, true, false, false);
    let pressed_hover = active_for(&projection, true, true, false, false, false);
    let disabled_hover = active_for(&projection, true, false, false, true, false);
    let selected_focus = active_for(&projection, false, false, true, false, true);

    assert_eq!(hover_focus.background_color().hex_triplet(), "#333333");
    assert_eq!(hover_focus.border_color().hex_triplet(), "#444444");
    assert_eq!(pressed_hover.background_color().hex_triplet(), "#222222");
    assert_eq!(disabled_hover.background_color().hex_triplet(), "#252a31");
    assert_eq!(disabled_hover.opacity(), 0.25);
    assert_eq!(
        disabled_hover.active_states(),
        &[
            WorthUiAppearanceStateName::Rest,
            WorthUiAppearanceStateName::Disabled
        ]
    );
    assert_eq!(selected_focus.background_color().hex_triplet(), "#555555");
    assert_eq!(selected_focus.border_color().hex_triplet(), "#444444");
}

#[test]
fn disabled_appearance_suppresses_pressed_outline_without_redeclaring_every_field() {
    let projection = activate_appearance_state_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_pressed_border_color",
            "\"#ff00ff\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_pressed_border_width",
            "validation.density.primitive.border.default",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_disabled_background",
            "\"#41464f\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_disabled_opacity",
            "\"0.45\"",
        ),
    ]);

    let active = active_for(&projection, true, true, false, true, false);

    assert_eq!(active.background_color().hex_triplet(), "#41464f");
    assert_eq!(active.border_width_points(), 0.0);
    assert_eq!(active.opacity(), 0.45);
    assert_eq!(
        active.active_states(),
        &[
            WorthUiAppearanceStateName::Rest,
            WorthUiAppearanceStateName::Disabled
        ]
    );
}

#[test]
fn disabled_observation_normalizes_hover_press_and_focus_before_paint_resolution() {
    let observed = WorthUiPrimitiveObservedPostureReceipt::from_renderer_observation(
        true, true, true, true, false,
    );
    let posture = observed.posture();

    assert!(!posture.hovered());
    assert!(!posture.pressed());
    assert!(!posture.focused());
    assert!(posture.disabled());
}

#[test]
fn prepared_reload_exposes_mapping_before_activation() {
    let workbench = launch_stable_workbench();
    let prepared = prepare_reload_for_edits(
        &workbench,
        &[ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "appearance_rest_background",
            "\"#b3261e\"",
        )],
    );
    let mapping = changed_fact_mapping(&prepared);

    assert_eq!(mapping.rows().len(), 2);
    assert!(mapping.rows().iter().any(
        |row| row.semantic_row().slice_id() == WorthUiSemanticSliceId::PrimitiveAppearanceState
    ));
}

fn active_for(
    projection: &worth_ui::facade::WorthUiPrimitiveProjectionReceipt,
    hovered: bool,
    pressed: bool,
    focused: bool,
    disabled: bool,
    selected: bool,
) -> worth_ui::facade::WorthUiResolvedAppearanceStateReceipt {
    projection
        .primitive_receipt()
        .paint_plan(
            1000.0,
            600.0,
            WorthUiPrimitiveObservedPostureReceipt::from_renderer_observation(
                hovered, pressed, focused, disabled, selected,
            ),
        )
        .active_appearance()
        .clone()
}

fn assert_exact_families(
    facts: &[worth_ui::facade::WorthUiRuntimeFactId],
    expected: &[WorthUiRuntimeFactFamily],
) {
    assert_eq!(facts.len(), expected.len());
    for expected_family in expected {
        assert!(
            facts.iter().any(|fact| fact.family() == *expected_family),
            "missing fact family {}",
            expected_family.token()
        );
    }
}
