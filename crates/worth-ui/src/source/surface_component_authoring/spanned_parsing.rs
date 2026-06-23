use crate::source::{WorthUiSourceToken, WorthUiSourceTokenKind};

use super::item_adapters::value_from_source_token;
use super::types::{
    WorthUiSpannedSurfaceAuthoring, WorthUiSpannedSurfaceAuthoringProperty,
    WorthUiSurfaceAuthoringParseFailure,
};

pub(super) fn parse_surface_authoring_tokens_with_spans(
    tokens: &[WorthUiSourceToken],
) -> Result<WorthUiSpannedSurfaceAuthoring<'_>, WorthUiSurfaceAuthoringParseFailure> {
    let mut index = if tokens
        .first()
        .is_some_and(|token| token.kind().is_component_keyword())
    {
        2
    } else {
        0
    };
    let mut properties = Vec::new();

    while index < tokens.len() {
        let key = tokens[index]
            .kind()
            .identifier_text()
            .ok_or(WorthUiSurfaceAuthoringParseFailure::Malformed)?;
        let value_token = tokens
            .get(index + 1)
            .ok_or(WorthUiSurfaceAuthoringParseFailure::Malformed)?;
        if value_from_source_token(value_token.kind()).is_none() {
            return Err(WorthUiSurfaceAuthoringParseFailure::Malformed);
        }
        properties.push(WorthUiSpannedSurfaceAuthoringProperty {
            key,
            source_span: value_token.span(),
        });
        index += 2;
    }

    Ok(WorthUiSpannedSurfaceAuthoring { properties })
}

trait SpannedSurfaceTokenKind {
    fn is_component_keyword(&self) -> bool;
    fn identifier_text(&self) -> Option<&str>;
}

impl SpannedSurfaceTokenKind for WorthUiSourceTokenKind {
    fn is_component_keyword(&self) -> bool {
        matches!(self, WorthUiSourceTokenKind::KeywordComponent)
    }

    fn identifier_text(&self) -> Option<&str> {
        match self {
            WorthUiSourceTokenKind::Identifier(text) => Some(text.as_str()),
            _ => None,
        }
    }
}
