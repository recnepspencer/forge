use crate::runtime::WorthUiProjectionFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticSliceConsumers {
    projection_families: &'static [WorthUiProjectionFamily],
}

impl WorthUiSemanticSliceConsumers {
    pub const fn new(projection_families: &'static [WorthUiProjectionFamily]) -> Self {
        Self {
            projection_families,
        }
    }

    pub const fn projection_families(self) -> &'static [WorthUiProjectionFamily] {
        self.projection_families
    }

    pub fn contains(self, family: WorthUiProjectionFamily) -> bool {
        self.projection_families.contains(&family)
    }
}
