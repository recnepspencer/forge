use crate::runtime::WorthUiPrimitiveContentValueDenialCode;

use super::support::{content_source, runtime_for_source, surface_id};

#[test]
fn content_denial_presentation_is_derived_from_typed_receipt() {
    let runtime = runtime_for_source(content_source(&[
        ("content_icon", "worth.icon.action.missing"),
        ("content_text_size", "15"),
        ("content_on_click", "submit"),
    ]));

    let report = runtime.resolve_primitive_content_admission_report(&surface_id());
    let denial_set = report
        .status()
        .denial_set()
        .expect("invalid content rejects");
    assert!(report.schema_digest() != 0);
    assert!(report.admission_digest() != 0);
    assert_eq!(report.counters().denials_emitted(), 3);

    let icon_denial = denial_set.denials()[0].clone();
    assert_eq!(
        icon_denial.denial_code(),
        WorthUiPrimitiveContentValueDenialCode::InvalidIconId
    );
    let presentation = icon_denial.presentation();
    let rows = presentation.rows();

    assert_eq!(presentation.title(), "Primitive content value rejected");
    assert_row(rows, "schema", "worth.primitive.content.prop.content_icon");
    assert_row(rows, "code", "InvalidIconId");
    assert_row(rows, "value_kind", "IconId");
    assert_row(rows, "prop", "content_icon");
    assert_row(
        rows,
        "expected",
        "a registered icon id like `worth.icon.action.plus`",
    );
    assert_row(rows, "fact", "primitive_content");
    assert!(row_value(rows, "source_span") != "unavailable");
    assert!(row_value(rows, "digest").parse::<u64>().unwrap() != 0);
}

#[test]
fn image_and_slot_are_known_but_capability_gated_unsupported_content_refs() {
    let runtime = runtime_for_source(content_source(&[
        ("content_image", "worth.image.logo"),
        ("content_slot", "leading"),
    ]));

    let denials = runtime
        .resolve_primitive_content_admission_report(&surface_id())
        .status()
        .denial_set()
        .expect("unsupported refs reject")
        .denials()
        .to_vec();

    assert_eq!(denials.len(), 2);
    assert_eq!(
        denials[0].denial_code(),
        WorthUiPrimitiveContentValueDenialCode::UnsupportedImageReference
    );
    assert_eq!(
        denials[1].denial_code(),
        WorthUiPrimitiveContentValueDenialCode::UnsupportedSlotDeclaration
    );
}

fn assert_row(
    rows: &[crate::runtime::WorthUiPrimitiveContentDenialPresentationRow],
    label: &str,
    expected: &str,
) {
    assert_eq!(row_value(rows, label), expected);
}

fn row_value<'a>(
    rows: &'a [crate::runtime::WorthUiPrimitiveContentDenialPresentationRow],
    label: &str,
) -> &'a str {
    rows.iter()
        .find(|row| row.label() == label)
        .map(|row| row.value())
        .expect("presentation row exists")
}
