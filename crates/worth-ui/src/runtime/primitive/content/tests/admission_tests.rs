use crate::runtime::{
    WorthUiPrimitiveContentIconPaintCommand, WorthUiPrimitiveContentIconRenderPosture,
    WorthUiPrimitiveContentItem, WorthUiPrimitiveContentItemKind, WorthUiPrimitiveContentKind,
    WorthUiPrimitiveContentValueDenialCode,
};

use super::support::{content_source, runtime_for_source, surface_id};

#[test]
fn token_backed_text_icon_content_lowers_to_sealed_receipt() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        (
            "content_text_size",
            "validation.density.primitive.content.text.default",
        ),
        (
            "content_icon_size",
            "validation.density.primitive.content.icon.large",
        ),
        (
            "content_icon_stroke",
            "validation.density.primitive.content.icon.stroke.thin",
        ),
        ("content_accessibility_name", "\"Submit form\""),
    ]));

    let report = runtime.resolve_primitive_content_admission_report(&surface_id());
    let receipt = report
        .status()
        .accepted_receipt()
        .expect("content values should admit");
    let prop_set = receipt.prop_set();
    assert_eq!(
        prop_set.icon_size_token(),
        "validation.density.primitive.content.icon.large"
    );
    assert_eq!(prop_set.icon_size_points(), 32.0);
    assert_eq!(
        prop_set.icon_stroke_token(),
        "validation.density.primitive.content.icon.stroke.thin"
    );

    let content = receipt.resolved_receipt(&runtime);
    assert_eq!(content.kind(), WorthUiPrimitiveContentKind::Inline);
    assert_eq!(content.accessibility_name(), Some("Submit form"));
    assert_eq!(
        content
            .items()
            .iter()
            .map(WorthUiPrimitiveContentItem::kind)
            .collect::<Vec<_>>(),
        vec![
            WorthUiPrimitiveContentItemKind::Icon,
            WorthUiPrimitiveContentItemKind::Text,
        ]
    );
    let icon = content.items()[0].as_icon().expect("first item is icon");
    assert_eq!(icon.icon_id(), "worth.icon.action.plus");
    assert_eq!(
        icon.paint_command(),
        WorthUiPrimitiveContentIconPaintCommand::Plus
    );
    assert_eq!(
        icon.render_posture(),
        WorthUiPrimitiveContentIconRenderPosture::NativeVector
    );
    assert_eq!(icon.size_points(), 32.0);
    assert_eq!(icon.stroke_width_points(), 1.0);
    assert_eq!(content.items()[1].as_text().unwrap().size_points(), 15.0);
    assert!(content.receipt_digest() != 0);
}

#[test]
fn invalid_content_values_batch_into_schema_ordered_denial_set() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.missing"),
        ("content_text_size", "15"),
        ("content_icon_size", "\"32px\""),
        (
            "content_icon_stroke",
            "validation.density.primitive.content.unknown",
        ),
        ("content_image", "assets/icon.svg"),
        ("content_slot", "footer"),
        ("content_margin", "fat"),
    ]));

    let report = runtime.resolve_primitive_content_admission_report(&surface_id());
    let denial_set = report
        .status()
        .denial_set()
        .expect("invalid content values reject");
    let denials = denial_set.denials();
    assert_eq!(report.counters().denials_emitted(), 7);
    assert_eq!(
        denials
            .iter()
            .map(|denial| denial.prop_key())
            .collect::<Vec<_>>(),
        vec![
            "content_icon",
            "content_text_size",
            "content_icon_size",
            "content_icon_stroke",
            "content_image",
            "content_slot",
            "content_margin",
        ]
    );
    assert!(denials.iter().all(|denial| {
        denial.fact_family() == crate::runtime::WorthUiRuntimeFactFamily::PrimitiveContent
            && denial.semantic_slice() == crate::runtime::WorthUiSemanticSliceId::PrimitiveContent
    }));
    assert_eq!(
        denials[0].denial_code(),
        WorthUiPrimitiveContentValueDenialCode::InvalidIconId
    );
    assert_eq!(
        denials[4].denial_code(),
        WorthUiPrimitiveContentValueDenialCode::UnsupportedImageReference
    );
    assert_eq!(
        denials[5].denial_code(),
        WorthUiPrimitiveContentValueDenialCode::UnsupportedSlotDeclaration
    );
    assert!(denial_set.denial_set_digest() != 0);
}
