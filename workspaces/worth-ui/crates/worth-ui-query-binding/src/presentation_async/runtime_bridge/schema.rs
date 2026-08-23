use worth_foundational::facade::ScalarAspectType;
use worth_query::facade::{foundation, read};

use super::super::WorthUiPresentationAsyncDeclaration;

pub(super) fn presentation_view_name(declaration: &WorthUiPresentationAsyncDeclaration) -> String {
    format!(
        "worth-ui.presentation.{}",
        declaration.request_identity().canonical_identity()
    )
}

pub(super) fn presentation_live_request() -> foundation::DeclarativeLiveQueryRequest {
    let mut request = foundation::DeclarativeLiveQueryRequest::new(
        "WorthUiPresentation",
        foundation::DeclarativeLiveViewShape::table(),
    );
    for (aspect, field, _) in PRESENTATION_FIELDS {
        request = request.project(
            foundation::DeclarativeProjectionField::new(
                foundation::AspectFieldKey::from_authoring_parts(aspect, field)
                    .expect("static presentation projection field must admit"),
            )
            .delivered_as(format!("{aspect}.{field}")),
        );
    }
    request
}

pub(super) fn presentation_schema_view() -> read::QuerySchemaView {
    read::QuerySchemaView::new(
        "worth-ui-presentation-async",
        PRESENTATION_FIELDS.map(|(aspect, field, family)| {
            read::SchemaFieldView::new(
                read::AspectName::new(aspect).expect("static presentation aspect must admit"),
                read::FieldName::new(field).expect("static presentation field must admit"),
                family,
            )
        }),
        [],
    )
}

const PRESENTATION_FIELDS: [(&str, &str, ScalarAspectType); 9] = [
    ("identity", "id", ScalarAspectType::String),
    ("presentation-content", "content", ScalarAspectType::UInt64),
    ("presentation-width", "width", ScalarAspectType::UInt64),
    (
        "presentation-paint-value",
        "paint_value",
        ScalarAspectType::UInt64,
    ),
    (
        "presentation-paint-boundary",
        "paint_boundary",
        ScalarAspectType::UInt64,
    ),
    ("presentation-dpi", "dpi", ScalarAspectType::UInt64),
    (
        "presentation-upload",
        "upload_completion",
        ScalarAspectType::UInt64,
    ),
    (
        "presentation-pin-release",
        "pin_release",
        ScalarAspectType::UInt64,
    ),
    (
        "presentation-currentness",
        "currentness",
        ScalarAspectType::UInt64,
    ),
];
