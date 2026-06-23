use crate::capability::DensityTokenId;

use super::{WorthUiDensityFamily, WorthUiDensityValue};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDensityTokenDescriptor {
    id: DensityTokenId,
    family: WorthUiDensityFamily,
    value: WorthUiDensityValue,
}

impl WorthUiDensityTokenDescriptor {
    pub fn define(
        id: DensityTokenId,
        family: WorthUiDensityFamily,
        value: WorthUiDensityValue,
    ) -> Self {
        Self { id, family, value }
    }

    pub fn id(&self) -> &DensityTokenId {
        &self.id
    }

    pub fn family(&self) -> &WorthUiDensityFamily {
        &self.family
    }

    pub fn value(&self) -> &WorthUiDensityValue {
        &self.value
    }
}
