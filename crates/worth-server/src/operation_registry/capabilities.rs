use crate::{WorthServerOperationFamily, WorthServerSurfaceFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationCapabilities {
    family: WorthServerOperationFamily,
    registration_state: WorthServerOperationRegistrationState,
    exposed_surfaces: Vec<WorthServerSurfaceFamily>,
}

impl WorthServerOperationCapabilities {
    pub(crate) fn enabled(
        family: WorthServerOperationFamily,
        exposed_surfaces: Vec<WorthServerSurfaceFamily>,
    ) -> Self {
        Self {
            family,
            registration_state: WorthServerOperationRegistrationState::Enabled,
            exposed_surfaces,
        }
    }

    pub(crate) fn disabled(
        family: WorthServerOperationFamily,
        exposed_surfaces: Vec<WorthServerSurfaceFamily>,
    ) -> Self {
        Self {
            family,
            registration_state: WorthServerOperationRegistrationState::Disabled,
            exposed_surfaces,
        }
    }

    pub(crate) fn absent(family: WorthServerOperationFamily) -> Self {
        Self {
            family,
            registration_state: WorthServerOperationRegistrationState::Absent,
            exposed_surfaces: Vec::new(),
        }
    }

    pub fn family(&self) -> WorthServerOperationFamily {
        self.family
    }

    pub fn is_registered(&self) -> bool {
        !self.is_absent()
    }

    pub fn is_absent(&self) -> bool {
        matches!(
            self.registration_state,
            WorthServerOperationRegistrationState::Absent
        )
    }

    pub fn is_disabled(&self) -> bool {
        matches!(
            self.registration_state,
            WorthServerOperationRegistrationState::Disabled
        )
    }

    pub fn is_enabled(&self) -> bool {
        matches!(
            self.registration_state,
            WorthServerOperationRegistrationState::Enabled
        )
    }

    pub fn exposed_surfaces(&self) -> &[WorthServerSurfaceFamily] {
        &self.exposed_surfaces
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthServerOperationRegistrationState {
    Enabled,
    Disabled,
    Absent,
}
