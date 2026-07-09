use super::WorthServerSurfaceFamily;
use crate::surfaces::{
    compat_http::WorthServerCompatHttpRouteFamilies, WorthServerSurfaceCapabilities,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerSurfaceRegistration {
    family: WorthServerSurfaceFamily,
    implementation_state: WorthServerSurfaceImplementationState,
    details: WorthServerSurfaceRegistrationDetails,
}

impl WorthServerSurfaceRegistration {
    pub(crate) fn enabled(family: WorthServerSurfaceFamily) -> Self {
        Self {
            family,
            implementation_state: WorthServerSurfaceImplementationState::Enabled,
            details: WorthServerSurfaceRegistrationDetails::None,
        }
    }

    pub(crate) fn disabled(family: WorthServerSurfaceFamily) -> Self {
        Self {
            family,
            implementation_state: WorthServerSurfaceImplementationState::Disabled,
            details: WorthServerSurfaceRegistrationDetails::None,
        }
    }

    pub(crate) fn enabled_compat_http(
        family: WorthServerSurfaceFamily,
        route_families: WorthServerCompatHttpRouteFamilies,
    ) -> Self {
        Self {
            family,
            implementation_state: WorthServerSurfaceImplementationState::Enabled,
            details: WorthServerSurfaceRegistrationDetails::CompatHttp { route_families },
        }
    }

    pub(crate) fn disabled_compat_http(
        family: WorthServerSurfaceFamily,
        route_families: WorthServerCompatHttpRouteFamilies,
    ) -> Self {
        Self {
            family,
            implementation_state: WorthServerSurfaceImplementationState::Disabled,
            details: WorthServerSurfaceRegistrationDetails::CompatHttp { route_families },
        }
    }

    pub fn family(&self) -> WorthServerSurfaceFamily {
        self.family
    }

    pub fn is_disabled(&self) -> bool {
        matches!(
            self.implementation_state,
            WorthServerSurfaceImplementationState::Disabled
        )
    }

    pub(crate) fn capabilities(&self) -> WorthServerSurfaceCapabilities {
        match self.implementation_state {
            WorthServerSurfaceImplementationState::Enabled => {
                WorthServerSurfaceCapabilities::enabled(self.family)
            }
            WorthServerSurfaceImplementationState::Disabled => {
                WorthServerSurfaceCapabilities::disabled(self.family)
            }
        }
    }

    pub(crate) fn compat_http_route_families(&self) -> Option<WorthServerCompatHttpRouteFamilies> {
        match &self.details {
            WorthServerSurfaceRegistrationDetails::CompatHttp { route_families } => {
                Some(route_families.clone())
            }
            WorthServerSurfaceRegistrationDetails::None => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthServerSurfaceImplementationState {
    Enabled,
    Disabled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthServerSurfaceRegistrationDetails {
    None,
    CompatHttp {
        route_families: WorthServerCompatHttpRouteFamilies,
    },
}
