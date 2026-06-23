use crate::source::{WorthUiArtifactInputBodyAtom, WorthUiSourceTokenKind};

use super::types::WorthUiSurfaceAuthoringValue;

pub(super) trait SurfaceComponentKeyword {
    fn is_component_keyword(&self) -> bool;
}

pub(super) trait SurfaceAuthoringItem: SurfaceComponentKeyword {
    fn identifier_text(&self) -> Option<&str>;
}

pub(super) fn identifier_from_source_token_kind(
    token_kind: &WorthUiSourceTokenKind,
) -> Option<&str> {
    match token_kind {
        WorthUiSourceTokenKind::Identifier(component_id) => Some(component_id.as_str()),
        _ => None,
    }
}

pub(super) fn identifier_from_body_atom(body_atom: &WorthUiArtifactInputBodyAtom) -> Option<&str> {
    match body_atom {
        WorthUiArtifactInputBodyAtom::Identifier(component_id) => Some(component_id.as_str()),
        _ => None,
    }
}

pub(super) fn value_from_source_token(
    token_kind: &WorthUiSourceTokenKind,
) -> Option<WorthUiSurfaceAuthoringValue<'_>> {
    match token_kind {
        WorthUiSourceTokenKind::Identifier(value) => {
            Some(WorthUiSurfaceAuthoringValue::Identifier(value.as_str()))
        }
        WorthUiSourceTokenKind::NumberLiteral(value) => {
            Some(WorthUiSurfaceAuthoringValue::NumberLiteral(*value))
        }
        WorthUiSourceTokenKind::StringLiteral(value) => {
            Some(WorthUiSurfaceAuthoringValue::StringLiteral(value.as_str()))
        }
        _ => None,
    }
}

pub(super) fn value_from_body_atom(
    body_atom: &WorthUiArtifactInputBodyAtom,
) -> Option<WorthUiSurfaceAuthoringValue<'_>> {
    match body_atom {
        WorthUiArtifactInputBodyAtom::Identifier(value) => {
            Some(WorthUiSurfaceAuthoringValue::Identifier(value.as_str()))
        }
        WorthUiArtifactInputBodyAtom::NumberLiteral(value) => {
            Some(WorthUiSurfaceAuthoringValue::NumberLiteral(*value))
        }
        WorthUiArtifactInputBodyAtom::StringLiteral(value) => {
            Some(WorthUiSurfaceAuthoringValue::StringLiteral(value.as_str()))
        }
        _ => None,
    }
}

impl SurfaceComponentKeyword for WorthUiSourceTokenKind {
    fn is_component_keyword(&self) -> bool {
        matches!(self, WorthUiSourceTokenKind::KeywordComponent)
    }
}

impl SurfaceAuthoringItem for WorthUiSourceTokenKind {
    fn identifier_text(&self) -> Option<&str> {
        identifier_from_source_token_kind(self)
    }
}

impl SurfaceComponentKeyword for WorthUiArtifactInputBodyAtom {
    fn is_component_keyword(&self) -> bool {
        matches!(self, WorthUiArtifactInputBodyAtom::KeywordComponent)
    }
}

impl SurfaceAuthoringItem for WorthUiArtifactInputBodyAtom {
    fn identifier_text(&self) -> Option<&str> {
        identifier_from_body_atom(self)
    }
}
