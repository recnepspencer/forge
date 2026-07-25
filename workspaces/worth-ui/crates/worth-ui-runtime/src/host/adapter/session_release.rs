#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostSessionReleaseReceipt {
    host_session_identity: u64,
    released_surface_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostSessionReleaseIndeterminate {
    host_session_identity: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSessionReleaseOutcome {
    Released(UiHostSessionReleaseReceipt),
    ReleaseIndeterminate(UiHostSessionReleaseIndeterminate),
}

impl UiHostSessionReleaseReceipt {
    pub fn released(host_session_identity: u64, released_surface_count: usize) -> Self {
        Self {
            host_session_identity,
            released_surface_count,
        }
    }

    pub fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }

    pub fn released_surface_count(self) -> usize {
        self.released_surface_count
    }
}

impl UiHostSessionReleaseIndeterminate {
    pub fn after_effects_may_have_begun(host_session_identity: u64) -> Self {
        Self {
            host_session_identity,
        }
    }

    pub fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }
}
