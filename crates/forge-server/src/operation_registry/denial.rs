use crate::{ForgeServerOperationFamily, ForgeServerSurfaceFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationDenial {
    UnregisteredFamily {
        family: ForgeServerOperationFamily,
        surface_family: ForgeServerSurfaceFamily,
    },
    DisabledFamily {
        family: ForgeServerOperationFamily,
        surface_family: ForgeServerSurfaceFamily,
    },
    SurfaceFamilyNotExposed {
        family: ForgeServerOperationFamily,
        surface_family: ForgeServerSurfaceFamily,
    },
    UnknownOperationName {
        family: ForgeServerOperationFamily,
        operation_name: String,
    },
}

impl ForgeServerOperationDenial {
    pub fn family(&self) -> ForgeServerOperationFamily {
        match self {
            Self::UnregisteredFamily { family, .. }
            | Self::DisabledFamily { family, .. }
            | Self::SurfaceFamilyNotExposed { family, .. }
            | Self::UnknownOperationName { family, .. } => *family,
        }
    }

    pub fn surface_family(&self) -> Option<ForgeServerSurfaceFamily> {
        match self {
            Self::UnregisteredFamily { surface_family, .. }
            | Self::DisabledFamily { surface_family, .. }
            | Self::SurfaceFamilyNotExposed { surface_family, .. } => Some(*surface_family),
            Self::UnknownOperationName { .. } => None,
        }
    }

    pub fn detail(&self) -> String {
        match self {
            Self::UnregisteredFamily {
                family,
                surface_family,
            } => format!(
                "operation family `{}` is not registered for surface family `{}`",
                family.as_str(),
                surface_family.as_str()
            ),
            Self::DisabledFamily {
                family,
                surface_family,
            } => format!(
                "operation family `{}` is registered but disabled for surface family `{}`",
                family.as_str(),
                surface_family.as_str()
            ),
            Self::SurfaceFamilyNotExposed {
                family,
                surface_family,
            } => format!(
                "operation family `{}` is not exposed on surface family `{}`",
                family.as_str(),
                surface_family.as_str()
            ),
            Self::UnknownOperationName {
                family,
                operation_name,
            } => format!(
                "operation name `{operation_name}` is not admitted for operation family `{}`",
                family.as_str()
            ),
        }
    }
}
