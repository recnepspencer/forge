use crate::source::{WorthUiArtifactInputBodyAtom, WorthUiSourceToken};

use super::item_adapters::{value_from_body_atom, value_from_source_token, SurfaceAuthoringItem};
use super::types::{
    WorthUiSurfaceAuthoring, WorthUiSurfaceAuthoringParseFailure, WorthUiSurfaceAuthoringProperty,
    WorthUiSurfaceAuthoringValue,
};

pub(super) fn parse_surface_authoring_tokens(
    tokens: &[WorthUiSourceToken],
) -> Result<WorthUiSurfaceAuthoring<'_>, WorthUiSurfaceAuthoringParseFailure> {
    parse_surface_authoring(
        tokens.iter().map(WorthUiSourceToken::kind),
        value_from_source_token,
    )
}

pub(super) fn parse_surface_authoring_body_atoms(
    body_atoms: &[WorthUiArtifactInputBodyAtom],
) -> Result<WorthUiSurfaceAuthoring<'_>, WorthUiSurfaceAuthoringParseFailure> {
    parse_surface_authoring(body_atoms.iter(), value_from_body_atom)
}

pub(super) fn parse_surface_authoring<'a, T>(
    items: impl Iterator<Item = &'a T>,
    value_from_item: impl Fn(&'a T) -> Option<WorthUiSurfaceAuthoringValue<'a>>,
) -> Result<WorthUiSurfaceAuthoring<'a>, WorthUiSurfaceAuthoringParseFailure>
where
    T: SurfaceAuthoringItem + 'a,
{
    let items = items.collect::<Vec<_>>();
    let mut index = 0;
    let mut component_id = None;
    let mut properties = Vec::new();

    if let Some(item) = items.first() {
        if item.is_component_keyword() {
            let Some(component_item) = items.get(1) else {
                return Err(WorthUiSurfaceAuthoringParseFailure::Malformed);
            };
            let Some(component_name) = component_item.identifier_text() else {
                return Err(WorthUiSurfaceAuthoringParseFailure::Malformed);
            };
            component_id = Some(component_name);
            index = 2;
        }
    }

    while index < items.len() {
        let key = items[index]
            .identifier_text()
            .ok_or(WorthUiSurfaceAuthoringParseFailure::Malformed)?;
        let value = items
            .get(index + 1)
            .and_then(|item| value_from_item(item))
            .ok_or(WorthUiSurfaceAuthoringParseFailure::Malformed)?;
        properties.push(WorthUiSurfaceAuthoringProperty { key, value });
        index += 2;
    }

    Ok(WorthUiSurfaceAuthoring {
        component_id,
        properties,
    })
}
