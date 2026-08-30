#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDeclaredServiceUsagePosture {
    Portal,
    Focus,
    Motion,
    CommandRouting,
    Scroll,
    Selection,
}

impl UiDeclaredServiceUsagePosture {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        use crate::declaration::service::{
            UiDeclaredCommandRoutingContract, UiDeclaredFocusOwnershipContract,
            UiDeclaredMotionPolicyContract, UiDeclaredPortalSurfaceContract,
            UiDeclaredScrollOwnershipContract, UiDeclaredSelectionIdentityContract,
        };

        match self {
            Self::Portal => UiDeclaredPortalSurfaceContract::MountedOverlay.family(),
            Self::Focus => UiDeclaredFocusOwnershipContract::SemanticKeyboardFocus.family(),
            Self::Motion => UiDeclaredMotionPolicyContract::ReducedMotionAware.family(),
            Self::CommandRouting => UiDeclaredCommandRoutingContract::TypedInvocation.family(),
            Self::Scroll => UiDeclaredScrollOwnershipContract::RuntimeOwnedOffset.family(),
            Self::Selection => UiDeclaredSelectionIdentityContract::StableItemKey.family(),
        }
    }
}
