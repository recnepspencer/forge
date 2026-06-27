use super::ThemeTokenDescriptor;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ThemeTokenKey {
    projection_basis: String,
}

impl ThemeTokenKey {
    pub(crate) fn from_descriptor(descriptor: &ThemeTokenDescriptor) -> Self {
        Self {
            projection_basis: theme_token_projection_basis(descriptor),
        }
    }

    pub fn projection_basis(&self) -> &str {
        &self.projection_basis
    }
}

fn theme_token_projection_basis(descriptor: &ThemeTokenDescriptor) -> String {
    [
        length_prefixed(descriptor.id().as_str()),
        descriptor.family().digest_basis(),
        descriptor.source().digest_basis().to_string(),
        value_basis(descriptor),
        alias_basis(descriptor),
    ]
    .join("|")
}

fn value_basis(descriptor: &ThemeTokenDescriptor) -> String {
    descriptor
        .value()
        .map(|value| format!("value:{}", value.digest_basis()))
        .unwrap_or_else(|| "value:none".to_string())
}

fn alias_basis(descriptor: &ThemeTokenDescriptor) -> String {
    descriptor
        .alias_definition()
        .map(|alias| format!("alias:{}", alias.digest_basis()))
        .unwrap_or_else(|| "alias:none".to_string())
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
