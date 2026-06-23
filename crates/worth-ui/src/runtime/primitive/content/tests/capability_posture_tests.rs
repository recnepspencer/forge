use crate::runtime::{
    WorthUiPrimitiveContentIconRenderPosture, WorthUiPrimitiveContentValueDenialCode,
};

use super::support::{content_source, runtime_for_source, surface_id};

#[test]
fn icon_render_posture_comes_from_registered_capability_support() {
    let runtime = runtime_for_source(content_source(&[
        ("content_order", "\"icon,text\""),
        ("content_icon", "worth.icon.action.fallback"),
        ("content_text", "\"Fallback\""),
    ]));

    let receipt = runtime
        .resolve_primitive_content_admission_report(&surface_id())
        .status()
        .accepted_receipt()
        .expect("fallback-capable icon id admits")
        .resolved_receipt(&runtime);
    let icon = receipt.items()[0].as_icon().expect("icon item resolves");

    assert_eq!(
        icon.render_posture(),
        WorthUiPrimitiveContentIconRenderPosture::SymbolFallback
    );
}

#[test]
fn raw_icon_paths_reject_through_content_icon_schema() {
    let runtime = runtime_for_source(content_source(&[
        ("content_order", "\"icon,text\""),
        ("content_icon", "\"assets/icons/plus.svg\""),
        ("content_text", "\"Submit\""),
    ]));

    let report = runtime.resolve_primitive_content_admission_report(&surface_id());
    let denial_set = report
        .status()
        .denial_set()
        .expect("raw icon paths are not icon ids");
    let denial = denial_set
        .denials()
        .iter()
        .find(|denial| denial.prop_key() == "content_icon")
        .expect("content_icon denial is emitted");

    assert_eq!(
        denial.denial_code(),
        WorthUiPrimitiveContentValueDenialCode::InvalidIconId
    );
    assert_eq!(
        denial.schema_id(),
        "worth.primitive.content.prop.content_icon"
    );
}
