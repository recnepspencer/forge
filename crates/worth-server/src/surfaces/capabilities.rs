use crate::WorthServerSurfaceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthServerSurfaceCapabilities {
    family: WorthServerSurfaceFamily,
    registration_state: WorthServerSurfaceRegistrationState,
}

impl WorthServerSurfaceCapabilities {
    pub(crate) fn enabled(family: WorthServerSurfaceFamily) -> Self {
        Self {
            family,
            registration_state: WorthServerSurfaceRegistrationState::Enabled,
        }
    }

    pub(crate) fn absent(family: WorthServerSurfaceFamily) -> Self {
        Self {
            family,
            registration_state: WorthServerSurfaceRegistrationState::Absent,
        }
    }

    pub(crate) fn disabled(family: WorthServerSurfaceFamily) -> Self {
        Self {
            family,
            registration_state: WorthServerSurfaceRegistrationState::Disabled,
        }
    }

    pub fn family(&self) -> WorthServerSurfaceFamily {
        self.family
    }

    pub fn is_registered(&self) -> bool {
        !self.is_absent()
    }

    pub fn is_absent(&self) -> bool {
        matches!(
            self.registration_state,
            WorthServerSurfaceRegistrationState::Absent
        )
    }

    pub fn is_disabled(&self) -> bool {
        matches!(
            self.registration_state,
            WorthServerSurfaceRegistrationState::Disabled
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthServerSurfaceRegistrationState {
    Enabled,
    Absent,
    Disabled,
}
