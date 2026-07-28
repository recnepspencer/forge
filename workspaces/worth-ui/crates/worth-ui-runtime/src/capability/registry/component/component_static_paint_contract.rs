use crate::capability::ThemeTokenId;

use super::ComponentStaticPaintOrder;

/// Complete component-owned meaning for one opaque static fill.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentStaticPaintContract {
    theme_token: ThemeTokenId,
    order: ComponentStaticPaintOrder,
}

impl ComponentStaticPaintContract {
    pub fn opaque_fill(theme_token: ThemeTokenId, order: ComponentStaticPaintOrder) -> Self {
        Self { theme_token, order }
    }

    pub fn theme_token(&self) -> &ThemeTokenId {
        &self.theme_token
    }

    pub const fn order(&self) -> ComponentStaticPaintOrder {
        self.order
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!(
            "opaque-fill:{}:{}",
            self.theme_token.as_str(),
            self.order.rank()
        )
    }
}
