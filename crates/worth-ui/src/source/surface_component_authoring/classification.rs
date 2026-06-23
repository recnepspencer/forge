use crate::source::{WorthUiArtifactInputBodyAtom, WorthUiSourceToken};

use super::item_adapters::{
    identifier_from_body_atom, identifier_from_source_token_kind, value_from_body_atom,
    value_from_source_token, SurfaceAuthoringItem,
};
use super::parsing::parse_surface_authoring;
use super::types::{WorthUiSurfaceAuthoringValue, WorthUiSurfaceComponentSelection};

pub(super) fn classify_surface_component_selection_tokens(
    tokens: &[WorthUiSourceToken],
) -> WorthUiSurfaceComponentSelection<'_> {
    classify_surface_component_selection(
        tokens.iter().map(WorthUiSourceToken::kind),
        identifier_from_source_token_kind,
        value_from_source_token,
    )
}

pub(super) fn classify_surface_component_selection_body_atoms(
    body_atoms: &[WorthUiArtifactInputBodyAtom],
) -> WorthUiSurfaceComponentSelection<'_> {
    classify_surface_component_selection(
        body_atoms.iter(),
        identifier_from_body_atom,
        value_from_body_atom,
    )
}

fn classify_surface_component_selection<'a, T>(
    items: impl Iterator<Item = &'a T>,
    identifier_text: impl Fn(&'a T) -> Option<&'a str>,
    value_from_item: impl Fn(&'a T) -> Option<WorthUiSurfaceAuthoringValue<'a>>,
) -> WorthUiSurfaceComponentSelection<'a>
where
    T: SurfaceAuthoringItem + 'a,
{
    let items = items.collect::<Vec<_>>();
    let Some(first) = items.first().copied() else {
        return WorthUiSurfaceComponentSelection::Absent;
    };
    if !first.is_component_keyword() {
        return WorthUiSurfaceComponentSelection::Absent;
    }
    let Some(second) = items.get(1).copied() else {
        return WorthUiSurfaceComponentSelection::Malformed;
    };
    let Some(component_id) = identifier_text(second) else {
        return WorthUiSurfaceComponentSelection::Malformed;
    };
    if items.len() == 2 {
        return WorthUiSurfaceComponentSelection::Selected(component_id);
    }
    match parse_surface_authoring(items.into_iter(), value_from_item) {
        Ok(authoring) => authoring.component_id().map_or(
            WorthUiSurfaceComponentSelection::Absent,
            WorthUiSurfaceComponentSelection::Selected,
        ),
        Err(_) => WorthUiSurfaceComponentSelection::Malformed,
    }
}
