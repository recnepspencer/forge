use crate::capability::ThemeTokenId;

use super::{
    RawColorOutsideTokenDefinition, ThemeTokenAlias, ThemeTokenFamily, ThemeTokenSource,
    ThemeTokenValue,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThemeTokenDescriptor {
    id: ThemeTokenId,
    family: ThemeTokenFamily,
    source: ThemeTokenSource,
    value: Option<ThemeTokenValue>,
    alias: Option<ThemeTokenAlias>,
    raw_color_outside_token_definition: Option<RawColorOutsideTokenDefinition>,
}

impl ThemeTokenDescriptor {
    pub fn define(
        id: ThemeTokenId,
        family: ThemeTokenFamily,
        source: ThemeTokenSource,
        value: ThemeTokenValue,
    ) -> Self {
        Self {
            id,
            family,
            source,
            value: Some(value),
            alias: None,
            raw_color_outside_token_definition: None,
        }
    }

    pub fn alias(
        id: ThemeTokenId,
        family: ThemeTokenFamily,
        source: ThemeTokenSource,
        alias: ThemeTokenAlias,
    ) -> Self {
        Self {
            id,
            family,
            source,
            value: None,
            alias: Some(alias),
            raw_color_outside_token_definition: None,
        }
    }

    pub fn missing_definition_for_diagnostics(
        id: ThemeTokenId,
        family: ThemeTokenFamily,
        source: ThemeTokenSource,
    ) -> Self {
        Self {
            id,
            family,
            source,
            value: None,
            alias: None,
            raw_color_outside_token_definition: None,
        }
    }

    pub fn raw_color_outside_token_definition_for_diagnostics(
        id: ThemeTokenId,
        raw_color: RawColorOutsideTokenDefinition,
    ) -> Self {
        Self {
            id,
            family: ThemeTokenFamily::text(),
            source: ThemeTokenSource::application(),
            value: None,
            alias: None,
            raw_color_outside_token_definition: Some(raw_color),
        }
    }

    pub fn id(&self) -> &ThemeTokenId {
        &self.id
    }

    pub fn family(&self) -> &ThemeTokenFamily {
        &self.family
    }

    pub fn source(&self) -> &ThemeTokenSource {
        &self.source
    }

    pub fn value(&self) -> Option<&ThemeTokenValue> {
        self.value.as_ref()
    }

    pub fn alias_target(&self) -> Option<&ThemeTokenId> {
        self.alias.as_ref().map(ThemeTokenAlias::target_id)
    }

    pub fn alias_definition(&self) -> Option<&ThemeTokenAlias> {
        self.alias.as_ref()
    }

    pub(crate) fn has_definition(&self) -> bool {
        self.value.is_some() ^ self.alias.is_some()
    }

    pub(crate) fn has_raw_color_outside_token_definition(&self) -> bool {
        self.raw_color_outside_token_definition.is_some()
    }
}
