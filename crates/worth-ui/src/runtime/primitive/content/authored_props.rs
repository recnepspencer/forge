use crate::capability::SurfaceId;
use crate::runtime::{WorthUiAuthoredSurfacePropValue, WorthUiRuntimeHost};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct AuthoredPrimitiveContentProp {
    pub(super) key: String,
    pub(super) value: String,
    pub(super) source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
}

pub(super) fn primitive_content_authored_props(
    host: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
) -> Vec<AuthoredPrimitiveContentProp> {
    host.inspect_active_authored_surface_props(surface_id)
        .map(|entry| AuthoredPrimitiveContentProp {
            key: entry.key().to_owned(),
            value: match entry.value() {
                WorthUiAuthoredSurfacePropValue::Identifier(value)
                | WorthUiAuthoredSurfacePropValue::StringLiteral(value) => value.clone(),
                WorthUiAuthoredSurfacePropValue::NumberLiteral(value) => value.to_string(),
            },
            source_span: entry.source_span(),
        })
        .collect()
}
