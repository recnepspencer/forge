use crate::capability::{MosaicRegionRole, SurfacePlacementClass};

/// Structural source family a mosaic placement policy may move from.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicPlacementSource {
    SurfaceClass(SurfacePlacementClass),
    RegionRole(MosaicRegionRole),
    PluginImperativeMutationForDiagnostics,
    MissingForDiagnostics,
}

impl MosaicPlacementSource {
    pub fn surface_class(surface_class: SurfacePlacementClass) -> Self {
        Self::SurfaceClass(surface_class)
    }

    pub fn region_role(region_role: MosaicRegionRole) -> Self {
        Self::RegionRole(region_role)
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

    pub(crate) fn is_unsupported_surface_class(&self) -> bool {
        matches!(self, Self::SurfaceClass(surface_class) if surface_class.is_unsupported())
    }

    pub(crate) fn is_product_domain_region_role(&self) -> bool {
        matches!(self, Self::RegionRole(role) if role.is_product_domain_name())
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::SurfaceClass(surface_class) => {
                format!("surface_class:{}", surface_class.digest_basis())
            }
            Self::RegionRole(role) => format!("region_role:{}", role.digest_basis()),
            Self::PluginImperativeMutationForDiagnostics => "plugin_imperative_mutation".to_owned(),
            Self::MissingForDiagnostics => "missing".to_owned(),
        }
    }
}
