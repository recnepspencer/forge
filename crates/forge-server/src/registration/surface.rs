use super::ForgeServerSurfaceFamily;
use crate::surfaces::ForgeServerSurfaceCapabilities;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerSurfaceRegistration {
    family: ForgeServerSurfaceFamily,
    implementation_state: ForgeServerSurfaceImplementationState,
}

impl ForgeServerSurfaceRegistration {
    pub(crate) fn disabled(family: ForgeServerSurfaceFamily) -> Self {
        Self {
            family,
            implementation_state: ForgeServerSurfaceImplementationState::Disabled,
        }
    }

    pub fn family(&self) -> ForgeServerSurfaceFamily {
        self.family
    }

    pub fn is_disabled(&self) -> bool {
        matches!(
            self.implementation_state,
            ForgeServerSurfaceImplementationState::Disabled
        )
    }

    pub(crate) fn capabilities(&self) -> ForgeServerSurfaceCapabilities {
        match self.implementation_state {
            ForgeServerSurfaceImplementationState::Disabled => {
                ForgeServerSurfaceCapabilities::disabled(self.family)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ForgeServerSurfaceImplementationState {
    Disabled,
}
