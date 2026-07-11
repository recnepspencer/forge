use forge_store_physical_certification::layout_harness::scenario::{
    S8LayoutProductionApi, canonical_s8_layout_production_apis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8LayoutFamilyBuilders {
    production_apis: &'static [S8LayoutProductionApi],
}

pub fn s8_layout_family_builders() -> S8LayoutFamilyBuilders {
    S8LayoutFamilyBuilders {
        production_apis: canonical_s8_layout_production_apis(),
    }
}

impl S8LayoutFamilyBuilders {
    pub const fn production_apis(&self) -> &'static [S8LayoutProductionApi] {
        self.production_apis
    }
}
