use crate::capability::AppearanceTokenId;

use super::{WorthUiAppearanceFamily, WorthUiAppearanceTokenSource, WorthUiAppearanceValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAppearanceTokenDescriptor {
    id: AppearanceTokenId,
    family: WorthUiAppearanceFamily,
    source: WorthUiAppearanceTokenSource,
    value: WorthUiAppearanceValue,
}

impl WorthUiAppearanceTokenDescriptor {
    pub fn define(
        id: AppearanceTokenId,
        family: WorthUiAppearanceFamily,
        source: WorthUiAppearanceTokenSource,
        value: WorthUiAppearanceValue,
    ) -> Self {
        Self {
            id,
            family,
            source,
            value,
        }
    }

    pub fn id(&self) -> &AppearanceTokenId {
        &self.id
    }

    pub fn family(&self) -> &WorthUiAppearanceFamily {
        &self.family
    }

    pub fn source(&self) -> &WorthUiAppearanceTokenSource {
        &self.source
    }

    pub fn value(&self) -> &WorthUiAppearanceValue {
        &self.value
    }
}
