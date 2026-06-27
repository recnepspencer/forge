use crate::capability::MosaicRegionRole;

/// Structural target family a mosaic placement policy may place into.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicPlacementTarget {
    RegionRole(MosaicRegionRole),
    RegionStack(MosaicRegionRole),
    PluginImperativeMutationForDiagnostics,
    MissingForDiagnostics,
}

impl MosaicPlacementTarget {
    pub fn region_role(region_role: MosaicRegionRole) -> Self {
        Self::RegionRole(region_role)
    }

    pub fn region_stack(region_role: MosaicRegionRole) -> Self {
        Self::RegionStack(region_role)
    }

    pub fn plugin_imperative_mutation_for_diagnostics() -> Self {
        Self::PluginImperativeMutationForDiagnostics
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn is_imperative_mutation(&self) -> bool {
        matches!(self, Self::PluginImperativeMutationForDiagnostics)
    }

    pub(crate) fn is_product_domain_region_role(&self) -> bool {
        matches!(
            self,
            Self::RegionRole(role) | Self::RegionStack(role) if role.is_product_domain_name()
        )
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::RegionRole(role) => format!("region_role:{}", role.digest_basis()),
            Self::RegionStack(role) => format!("region_stack:{}", role.digest_basis()),
            Self::PluginImperativeMutationForDiagnostics => "plugin_imperative_mutation".to_owned(),
            Self::MissingForDiagnostics => "missing".to_owned(),
        }
    }
}
