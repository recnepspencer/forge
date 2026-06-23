mod primitive_content_hot_reload_support;
mod validation_app_reload_fixture;

use primitive_content_hot_reload_support::{
    activate_content_edits, assert_content_projection_rebound,
    stable_source_text_with_content_edits, PRIMITIVE_SURFACE,
};
use worth_ui::facade::{
    WorthUiPrimitiveContentIconPaintCommand, WorthUiPrimitiveContentItem,
    WorthUiPrimitiveProofDenial, WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

#[test]
fn content_icon_swap_size_and_order_hot_reload_through_content_facts() {
    let projection = activate_content_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "content_icon",
            "worth.icon.action.check",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "content_icon_size",
            "validation.density.primitive.content.icon.large",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "content_order",
            "\"text,icon\"",
        ),
    ]);

    assert_content_projection_rebound(&projection);
    let content = projection.primitive_receipt().content();
    assert!(matches!(
        content.items()[0],
        WorthUiPrimitiveContentItem::Text(_)
    ));
    let icon = content.items()[1].as_icon().expect("icon is second item");
    assert_eq!(
        icon.paint_command(),
        WorthUiPrimitiveContentIconPaintCommand::Check
    );
    assert_eq!(icon.size_points(), 32.0);
}

#[test]
fn flow_gap_edit_preserves_content_digest_and_rebinds_flow_only() {
    let projection = activate_content_edits(&[ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "flow_gap",
        "validation.density.primitive.flow.gap.compact",
    )]);

    assert!(projection.changed_rows().iter().any(
        |row| row.changed_facts()[0].family() == WorthUiRuntimeFactFamily::PrimitiveFlowLayout
    ));
    assert!(!projection
        .changed_rows()
        .iter()
        .any(|row| row.changed_facts()[0].family() == WorthUiRuntimeFactFamily::PrimitiveContent));
}

#[test]
fn observed_source_reload_updates_text_only_to_icon_text_content() {
    let fixture = validation_app_reload_fixture::ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    fixture.write_source(&stable_source_text_with_content_edits(&[
        ("content_order", "\"text\""),
        ("content_icon", "worth.icon.action.plus"),
    ]));
    app.run_one_reload_observation_cycle();
    let text_only = app
        .centered_primitive_proof()
        .expect("observed text-only source reload resolves");
    assert_eq!(text_only.content().items().len(), 1);

    fixture.write_source(&stable_source_text_with_content_edits(&[
        ("content_order", "\"icon,text\""),
        ("content_icon", "worth.icon.action.plus"),
    ]));
    app.run_one_reload_observation_cycle();
    let icon_text = app
        .centered_primitive_proof()
        .expect("observed icon/text source reload resolves");

    assert_eq!(icon_text.content().items().len(), 2);
    assert!(matches!(
        icon_text.content().items()[0],
        WorthUiPrimitiveContentItem::Icon(_)
    ));
}

#[test]
fn observed_invalid_content_batch_surfaces_one_typed_content_report() {
    let fixture = validation_app_reload_fixture::ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    fixture.write_source(&stable_source_text_with_content_edits(&[]));
    app.run_one_reload_observation_cycle();
    let prior = app
        .centered_primitive_proof()
        .expect("prior valid primitive proof resolves")
        .receipt_digest();

    fixture.write_source(&stable_source_text_with_content_edits(&[
        ("content_icon", "worth.icon.action.missing"),
        ("content_icon_size", "\"32px\""),
    ]));
    app.run_one_reload_observation_cycle();
    let denial = app
        .centered_primitive_proof()
        .expect_err("invalid content reload surfaces primitive content denial");
    let WorthUiPrimitiveProofDenial::InvalidContentValues { report } = denial else {
        panic!("expected content denial after prior digest {prior}");
    };
    let denial_set = report
        .status()
        .denial_set()
        .expect("content denial report carries denial set");

    assert_eq!(denial_set.denials().len(), 2);
    assert_eq!(report.counters().denials_emitted(), 2);
    assert_eq!(denial_set.denials()[0].prop_key(), "content_icon");
    assert_eq!(denial_set.denials()[1].prop_key(), "content_icon_size");
}
