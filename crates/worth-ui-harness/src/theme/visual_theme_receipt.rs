use super::{HarnessDensity, HarnessThemeTokenCatalog, HarnessVisualTokenRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessVisualThemeReceipt {
    density: HarnessDensity,
    covered_roles: Vec<HarnessVisualTokenRole>,
    theme_token_count: usize,
    sizing_contract_count: usize,
}

impl HarnessVisualThemeReceipt {
    pub(crate) fn new(
        density: HarnessDensity,
        catalog: &HarnessThemeTokenCatalog,
        sizing_contract_count: usize,
    ) -> Self {
        let covered_roles = catalog
            .bindings()
            .iter()
            .map(|binding| binding.role())
            .collect();
        Self {
            density,
            covered_roles,
            theme_token_count: catalog.descriptors().len(),
            sizing_contract_count,
        }
    }

    pub fn density(&self) -> HarnessDensity {
        self.density
    }

    pub fn covers(&self, role: HarnessVisualTokenRole) -> bool {
        self.covered_roles.contains(&role)
    }

    pub fn theme_token_count(&self) -> usize {
        self.theme_token_count
    }

    pub fn sizing_contract_count(&self) -> usize {
        self.sizing_contract_count
    }
}
