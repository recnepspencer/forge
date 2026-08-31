use worth_ui_host_contract::{
    UiHostObservationPresentationBasis, UiHostPointerIdentity, UiHostSurfacePosition,
    UiMountedInstanceIdentity, UiSemanticSurfaceIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPointerPresenceTargetTransition {
    pub(super) pointer: UiHostPointerIdentity,
    pub(super) surface: Option<UiSemanticSurfaceIdentity>,
    pub(super) previous: Option<UiMountedInstanceIdentity>,
    pub(super) current: Option<UiMountedInstanceIdentity>,
    pub(super) owner_revision: u64,
    pub(super) position: UiHostSurfacePosition,
    pub(super) presentation: UiHostObservationPresentationBasis,
}

impl UiPointerPresenceTargetTransition {
    pub const fn pointer(self) -> UiHostPointerIdentity {
        self.pointer
    }
    pub const fn surface(self) -> Option<UiSemanticSurfaceIdentity> {
        self.surface
    }
    pub const fn previous(self) -> Option<UiMountedInstanceIdentity> {
        self.previous
    }
    pub const fn current(self) -> Option<UiMountedInstanceIdentity> {
        self.current
    }
    pub const fn owner_revision(self) -> u64 {
        self.owner_revision
    }
    pub const fn position(self) -> UiHostSurfacePosition {
        self.position
    }
    pub const fn presentation(self) -> UiHostObservationPresentationBasis {
        self.presentation
    }
}
