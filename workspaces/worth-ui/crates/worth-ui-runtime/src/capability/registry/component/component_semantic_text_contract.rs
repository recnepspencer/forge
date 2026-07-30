use crate::capability::ThemeTokenId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentSemanticTextContract {
    theme_token: ThemeTokenId,
    layer_semantic_order: u32,
}

impl ComponentSemanticTextContract {
    pub fn body_default(theme_token: ThemeTokenId, layer_semantic_order: u32) -> Self {
        Self {
            theme_token,
            layer_semantic_order,
        }
    }

    pub fn theme_token(&self) -> &ThemeTokenId {
        &self.theme_token
    }

    pub fn layer_semantic_order(&self) -> u32 {
        self.layer_semantic_order
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!(
            "semantic-text:{}:{}",
            self.theme_token.as_str(),
            self.layer_semantic_order
        )
    }
}
