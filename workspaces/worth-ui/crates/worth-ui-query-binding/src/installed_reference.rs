use std::fmt;

/// Compact binding-owned reference retained by Worth UI plan lowering.
///
/// The definition remains inspectable, but the installed-domain witness is
/// opaque and can only originate from an installed binding plan. Clones share
/// that exact Query installation authority.
#[derive(Clone)]
pub struct WorthUiInstalledQueryBindingReference {
    installed_domain: crate::WorthUiInstalledQueryDomain,
    definition: crate::WorthUiQueryViewDefinition,
}

impl WorthUiInstalledQueryBindingReference {
    pub(crate) fn new(
        installed_domain: crate::WorthUiInstalledQueryDomain,
        definition: crate::WorthUiQueryViewDefinition,
    ) -> Self {
        Self {
            installed_domain,
            definition,
        }
    }

    pub fn definition(&self) -> &crate::WorthUiQueryViewDefinition {
        &self.definition
    }

    pub(crate) fn installed_domain(&self) -> &crate::WorthUiInstalledQueryDomain {
        &self.installed_domain
    }
}

impl fmt::Debug for WorthUiInstalledQueryBindingReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthUiInstalledQueryBindingReference")
            .field("definition", &self.definition)
            .field("installed_domain_authority", &"sealed")
            .finish()
    }
}

impl PartialEq for WorthUiInstalledQueryBindingReference {
    fn eq(&self, other: &Self) -> bool {
        self.definition == other.definition
            && self
                .installed_domain
                .shares_authority_with(&other.installed_domain)
    }
}

impl Eq for WorthUiInstalledQueryBindingReference {}
