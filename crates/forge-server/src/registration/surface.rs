use super::ForgeServerSurfaceFamily;
use crate::surfaces::{
    compat_http::ForgeServerCompatHttpRouteFamilies, ForgeServerSurfaceCapabilities,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerSurfaceRegistration {
    family: ForgeServerSurfaceFamily,
    implementation_state: ForgeServerSurfaceImplementationState,
    details: ForgeServerSurfaceRegistrationDetails,
}

impl ForgeServerSurfaceRegistration {
    pub(crate) fn enabled(family: ForgeServerSurfaceFamily) -> Self {
        Self {
            family,
            implementation_state: ForgeServerSurfaceImplementationState::Enabled,
            details: ForgeServerSurfaceRegistrationDetails::None,
        }
    }

    pub(crate) fn disabled(family: ForgeServerSurfaceFamily) -> Self {
        Self {
            family,
            implementation_state: ForgeServerSurfaceImplementationState::Disabled,
            details: ForgeServerSurfaceRegistrationDetails::None,
        }
    }

    pub(crate) fn enabled_compat_http(
        family: ForgeServerSurfaceFamily,
        route_families: ForgeServerCompatHttpRouteFamilies,
    ) -> Self {
        Self {
            family,
            implementation_state: ForgeServerSurfaceImplementationState::Enabled,
            details: ForgeServerSurfaceRegistrationDetails::CompatHttp { route_families },
        }
    }

    pub(crate) fn disabled_compat_http(
        family: ForgeServerSurfaceFamily,
        route_families: ForgeServerCompatHttpRouteFamilies,
    ) -> Self {
        Self {
            family,
            implementation_state: ForgeServerSurfaceImplementationState::Disabled,
            details: ForgeServerSurfaceRegistrationDetails::CompatHttp { route_families },
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
            ForgeServerSurfaceImplementationState::Enabled => {
                ForgeServerSurfaceCapabilities::enabled(self.family)
            }
            ForgeServerSurfaceImplementationState::Disabled => {
                ForgeServerSurfaceCapabilities::disabled(self.family)
            }
        }
    }

    pub(crate) fn compat_http_route_families(&self) -> Option<ForgeServerCompatHttpRouteFamilies> {
        match &self.details {
            ForgeServerSurfaceRegistrationDetails::CompatHttp { route_families } => {
                Some(route_families.clone())
            }
            ForgeServerSurfaceRegistrationDetails::None => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ForgeServerSurfaceImplementationState {
    Enabled,
    Disabled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ForgeServerSurfaceRegistrationDetails {
    None,
    CompatHttp {
        route_families: ForgeServerCompatHttpRouteFamilies,
    },
}
