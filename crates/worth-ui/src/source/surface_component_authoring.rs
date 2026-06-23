mod classification;
mod item_adapters;
mod parsing;
mod spanned_parsing;
mod types;

use crate::source::{WorthUiArtifactInputBodyAtom, WorthUiSourceToken};

pub(crate) use types::{
    WorthUiSpannedSurfaceAuthoring, WorthUiSpannedSurfaceAuthoringProperty,
    WorthUiSurfaceAuthoring, WorthUiSurfaceAuthoringParseFailure, WorthUiSurfaceAuthoringProperty,
    WorthUiSurfaceAuthoringValue, WorthUiSurfaceComponentSelection,
};

pub(crate) fn classify_surface_component_selection_tokens(
    tokens: &[WorthUiSourceToken],
) -> WorthUiSurfaceComponentSelection<'_> {
    classification::classify_surface_component_selection_tokens(tokens)
}

pub(crate) fn classify_surface_component_selection_body_atoms(
    body_atoms: &[WorthUiArtifactInputBodyAtom],
) -> WorthUiSurfaceComponentSelection<'_> {
    classification::classify_surface_component_selection_body_atoms(body_atoms)
}

pub(crate) fn parse_surface_authoring_tokens(
    tokens: &[WorthUiSourceToken],
) -> Result<WorthUiSurfaceAuthoring<'_>, WorthUiSurfaceAuthoringParseFailure> {
    parsing::parse_surface_authoring_tokens(tokens)
}

pub(crate) fn parse_surface_authoring_tokens_with_spans(
    tokens: &[WorthUiSourceToken],
) -> Result<WorthUiSpannedSurfaceAuthoring<'_>, WorthUiSurfaceAuthoringParseFailure> {
    spanned_parsing::parse_surface_authoring_tokens_with_spans(tokens)
}

pub(crate) fn parse_surface_authoring_body_atoms(
    body_atoms: &[WorthUiArtifactInputBodyAtom],
) -> Result<WorthUiSurfaceAuthoring<'_>, WorthUiSurfaceAuthoringParseFailure> {
    parsing::parse_surface_authoring_body_atoms(body_atoms)
}
