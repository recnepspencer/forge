use crate::{ForgeServerOperationFamily, ForgeServerSurfaceFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationCapabilities {
    family: ForgeServerOperationFamily,
    registration_state: ForgeServerOperationRegistrationState,
    exposed_surfaces: Vec<ForgeServerSurfaceFamily>,
}

impl ForgeServerOperationCapabilities {
    pub(crate) fn enabled(
        family: ForgeServerOperationFamily,
        exposed_surfaces: Vec<ForgeServerSurfaceFamily>,
    ) -> Self {
        Self {
            family,
            registration_state: ForgeServerOperationRegistrationState::Enabled,
            exposed_surfaces,
        }
    }

    pub(crate) fn disabled(
        family: ForgeServerOperationFamily,
        exposed_surfaces: Vec<ForgeServerSurfaceFamily>,
    ) -> Self {
        Self {
            family,
            registration_state: ForgeServerOperationRegistrationState::Disabled,
            exposed_surfaces,
        }
    }

    pub(crate) fn absent(family: ForgeServerOperationFamily) -> Self {
        Self {
            family,
            registration_state: ForgeServerOperationRegistrationState::Absent,
            exposed_surfaces: Vec::new(),
        }
    }

    pub fn family(&self) -> ForgeServerOperationFamily {
        self.family
    }

    pub fn is_registered(&self) -> bool {
        !self.is_absent()
    }

    pub fn is_absent(&self) -> bool {
        matches!(
            self.registration_state,
            ForgeServerOperationRegistrationState::Absent
        )
    }

    pub fn is_disabled(&self) -> bool {
        matches!(
            self.registration_state,
            ForgeServerOperationRegistrationState::Disabled
        )
    }

    pub fn is_enabled(&self) -> bool {
        matches!(
            self.registration_state,
            ForgeServerOperationRegistrationState::Enabled
        )
    }

    pub fn exposed_surfaces(&self) -> &[ForgeServerSurfaceFamily] {
        &self.exposed_surfaces
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ForgeServerOperationRegistrationState {
    Enabled,
    Disabled,
    Absent,
}
