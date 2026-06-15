use std::collections::BTreeMap;

use worth_ui::facade::{ThemeTokenDescriptor, ThemeTokenId};

use super::HarnessVisualTokenRole;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessThemeTokenBinding {
    role: HarnessVisualTokenRole,
    token_id: ThemeTokenId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessThemeTokenCatalog {
    bindings: Vec<HarnessThemeTokenBinding>,
    descriptors: Vec<ThemeTokenDescriptor>,
}

impl HarnessThemeTokenBinding {
    pub fn new(role: HarnessVisualTokenRole, token_id: ThemeTokenId) -> Self {
        Self { role, token_id }
    }

    pub fn role(&self) -> HarnessVisualTokenRole {
        self.role
    }

    pub fn token_id(&self) -> &ThemeTokenId {
        &self.token_id
    }
}

impl HarnessThemeTokenCatalog {
    pub fn new(
        bindings: Vec<HarnessThemeTokenBinding>,
        descriptors: Vec<ThemeTokenDescriptor>,
    ) -> Self {
        Self {
            bindings,
            descriptors,
        }
    }

    pub fn bindings(&self) -> &[HarnessThemeTokenBinding] {
        &self.bindings
    }

    pub fn descriptors(&self) -> &[ThemeTokenDescriptor] {
        &self.descriptors
    }

    pub fn token_id_for(&self, role: HarnessVisualTokenRole) -> Option<&ThemeTokenId> {
        self.bindings
            .iter()
            .find(|binding| binding.role == role)
            .map(HarnessThemeTokenBinding::token_id)
    }

    pub(crate) fn duplicate_role(&self) -> Option<HarnessVisualTokenRole> {
        let mut counts = BTreeMap::<HarnessVisualTokenRole, usize>::new();
        for binding in &self.bindings {
            let count = counts.entry(binding.role()).or_default();
            *count += 1;
            if *count > 1 {
                return Some(binding.role());
            }
        }
        None
    }
}
