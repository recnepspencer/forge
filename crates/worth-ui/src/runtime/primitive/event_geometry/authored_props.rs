use crate::capability::SurfaceId;
use crate::runtime::{WorthUiAuthoredSurfacePropValue, WorthUiRuntimeHost};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AuthoredEventGeometryProp {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
}

impl AuthoredEventGeometryProp {
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

pub(super) fn event_geometry_authored_props(
    host: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
) -> Vec<AuthoredEventGeometryProp> {
    host.inspect_active_authored_surface_props(surface_id)
        .map(|entry| {
            let value = match entry.value() {
                WorthUiAuthoredSurfacePropValue::Identifier(value)
                | WorthUiAuthoredSurfacePropValue::StringLiteral(value) => value.clone(),
                WorthUiAuthoredSurfacePropValue::NumberLiteral(value) => value.to_string(),
            };
            AuthoredEventGeometryProp::new(entry.key(), value, entry.source_span())
        })
        .collect()
}
