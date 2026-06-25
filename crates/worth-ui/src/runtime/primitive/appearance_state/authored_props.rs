use crate::capability::SurfaceId;
use crate::runtime::{
    WorthUiAuthoredSurfacePropValue, WorthUiPrimitiveSourceSpan, WorthUiRuntimeHost,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AuthoredAppearanceStateProp {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) source_span: Option<WorthUiPrimitiveSourceSpan>,
}

impl AuthoredAppearanceStateProp {
    pub(crate) fn new(
        key: impl Into<String>,
        value: impl Into<String>,
        source_span: Option<WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            source_span,
        }
    }
}

pub(super) fn appearance_state_authored_props(
    host: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
) -> Vec<AuthoredAppearanceStateProp> {
    host.inspect_active_authored_surface_props(surface_id)
        .filter(|entry| entry.key().starts_with("appearance_"))
        .map(|entry| {
            let value = match entry.value() {
                WorthUiAuthoredSurfacePropValue::Identifier(value)
                | WorthUiAuthoredSurfacePropValue::StringLiteral(value) => value.clone(),
                WorthUiAuthoredSurfacePropValue::NumberLiteral(value) => value.to_string(),
            };
            AuthoredAppearanceStateProp::new(entry.key(), value, entry.source_span())
        })
        .collect()
}
