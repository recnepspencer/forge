use crate::ForgeServerSurfaceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeServerSurfaceCapabilities {
    family: ForgeServerSurfaceFamily,
    registration_state: ForgeServerSurfaceRegistrationState,
}

impl ForgeServerSurfaceCapabilities {
    pub(crate) fn enabled(family: ForgeServerSurfaceFamily) -> Self {
        Self {
            family,
            registration_state: ForgeServerSurfaceRegistrationState::Enabled,
        }
    }

    pub(crate) fn absent(family: ForgeServerSurfaceFamily) -> Self {
        Self {
            family,
            registration_state: ForgeServerSurfaceRegistrationState::Absent,
        }
    }

    pub(crate) fn disabled(family: ForgeServerSurfaceFamily) -> Self {
        Self {
            family,
            registration_state: ForgeServerSurfaceRegistrationState::Disabled,
        }
    }

    pub fn family(&self) -> ForgeServerSurfaceFamily {
        self.family
    }

    pub fn is_registered(&self) -> bool {
        !self.is_absent()
    }

    pub fn is_absent(&self) -> bool {
        matches!(
            self.registration_state,
            ForgeServerSurfaceRegistrationState::Absent
        )
    }

    pub fn is_disabled(&self) -> bool {
        matches!(
            self.registration_state,
            ForgeServerSurfaceRegistrationState::Disabled
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ForgeServerSurfaceRegistrationState {
    Enabled,
    Absent,
    Disabled,
}
