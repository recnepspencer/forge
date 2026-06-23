use crate::runtime::{
    WorthUiRuntimeFactFamily, WorthUiRuntimeFactId, WorthUiRuntimeFactSet,
    WorthUiValidationReloadRequest,
};

use super::support::{content_source, runtime_for_source, SURFACE_ID};

#[test]
fn content_text_edit_emits_only_primitive_content_changed_fact() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
    ]));
    let prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module(
            "app/main.wui",
            content_source(&[
                ("content_kind", "inline"),
                ("content_order", "\"icon,text\""),
                ("content_text", "\"Confirm\""),
                ("content_icon", "worth.icon.action.plus"),
            ]),
        ),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("content edit should produce changed-fact proof");

    assert_only_content_changed(receipt.changed_facts());
}

#[test]
fn content_order_edit_emits_only_primitive_content_changed_fact() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
    ]));
    let prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module(
            "app/main.wui",
            content_source(&[
                ("content_kind", "inline"),
                ("content_order", "\"text,icon\""),
                ("content_text", "\"Submit\""),
                ("content_icon", "worth.icon.action.plus"),
            ]),
        ),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("content order edit should produce changed-fact proof");

    assert_only_content_changed(receipt.changed_facts());
}

#[test]
fn content_icon_size_edit_emits_only_primitive_content_changed_fact() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        (
            "content_icon_size",
            "validation.density.primitive.content.icon.default",
        ),
    ]));
    let prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module(
            "app/main.wui",
            content_source(&[
                ("content_kind", "inline"),
                ("content_order", "\"icon,text\""),
                ("content_text", "\"Submit\""),
                ("content_icon", "worth.icon.action.plus"),
                (
                    "content_icon_size",
                    "validation.density.primitive.content.icon.large",
                ),
            ]),
        ),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("content icon size edit should produce changed-fact proof");

    assert_only_content_changed(receipt.changed_facts());
}

#[test]
fn appearance_color_edits_do_not_dirty_content_facts() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        ("appearance_rest_text_color", "\"#f7f1e8\""),
        ("appearance_rest_icon_color", "\"#f7f1e8\""),
    ]));
    let prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module(
            "app/main.wui",
            content_source(&[
                ("content_kind", "inline"),
                ("content_order", "\"icon,text\""),
                ("content_text", "\"Submit\""),
                ("content_icon", "worth.icon.action.plus"),
                ("appearance_rest_text_color", "\"#2f7de1\""),
                ("appearance_rest_icon_color", "\"#2f7de1\""),
            ]),
        ),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("appearance color edit should produce changed-fact proof");

    assert!(receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::PrimitiveAppearanceState));
    assert!(!receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::PrimitiveContent));
}

#[test]
fn flow_gap_edit_does_not_dirty_content_facts() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        ("flow_gap", "validation.density.primitive.flow.gap.compact"),
    ]));
    let prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module(
            "app/main.wui",
            content_source(&[
                ("content_kind", "inline"),
                ("content_order", "\"icon,text\""),
                ("content_text", "\"Submit\""),
                ("content_icon", "worth.icon.action.plus"),
                ("flow_gap", "validation.density.primitive.flow.gap.default"),
            ]),
        ),
    );
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("flow gap edit should produce changed-fact proof");

    assert!(receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::PrimitiveFlowLayout));
    assert!(!receipt
        .changed_facts()
        .contains_family(WorthUiRuntimeFactFamily::PrimitiveContent));
}

fn assert_only_content_changed(changed_facts: &WorthUiRuntimeFactSet) {
    assert!(changed_facts.contains_exact(&WorthUiRuntimeFactId::primitive_content(SURFACE_ID)));
    assert!(changed_facts.contains_family(WorthUiRuntimeFactFamily::PrimitiveContent));
    assert!(!changed_facts.contains_family(WorthUiRuntimeFactFamily::PrimitiveFlowLayout));
    assert!(!changed_facts.contains_family(WorthUiRuntimeFactFamily::PrimitiveAppearance));
    assert!(!changed_facts.contains_family(WorthUiRuntimeFactFamily::PrimitiveAppearanceState));
    assert!(!changed_facts.contains_family(WorthUiRuntimeFactFamily::PrimitiveInteraction));
}
