use crate::capability::SurfaceId;
use crate::runtime::{WorthUiAuthoredSurfacePropValue, WorthUiRuntimeHost};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AuthoredPrimitiveContentProp {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
}

impl AuthoredPrimitiveContentProp {
    pub(crate) fn new(
        key: impl Into<String>,
        value: impl Into<String>,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            source_span,
        }
    }
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
