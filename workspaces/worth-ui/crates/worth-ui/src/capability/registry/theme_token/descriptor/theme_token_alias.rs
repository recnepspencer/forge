use crate::capability::ThemeTokenId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThemeTokenAlias {
    target_id: ThemeTokenId,
}

impl ThemeTokenAlias {
    pub fn to(target_id: ThemeTokenId) -> Self {
        Self { target_id }
    }

    pub fn target_id(&self) -> &ThemeTokenId {
        &self.target_id
    }

    pub(crate) fn digest_basis(&self) -> String {
        length_prefixed(self.target_id.as_str())
    }
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
