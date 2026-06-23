use crate::capability::SurfaceId;
use crate::runtime::{
    WorthUiAuthoredSurfacePropValue, WorthUiPrimitiveSourceSpan, WorthUiRuntimeHost,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct AuthoredAppearanceStateProp {
    pub(super) key: String,
    pub(super) value: String,
    pub(super) source_span: Option<WorthUiPrimitiveSourceSpan>,
}

pub(super) fn appearance_state_authored_props(
    host: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
) -> Vec<AuthoredAppearanceStateProp> {
    host.inspect_active_authored_surface_props(surface_id)
        .filter(|entry| entry.key().starts_with("appearance_"))
        .map(|entry| AuthoredAppearanceStateProp {
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
