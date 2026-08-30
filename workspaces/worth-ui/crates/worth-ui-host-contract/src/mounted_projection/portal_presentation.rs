use crate::UiMountedInstanceIdentity;

/// Exact mounted affiliation between product-authored Portal content and the
/// presented Portal mechanic that owns its current presentation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedPortalPresentationAffinity {
    owner: UiMountedInstanceIdentity,
    portal_identity: u64,
}

impl UiMountedPortalPresentationAffinity {
    #[doc(hidden)]
    pub const fn from_runtime_mounting(
        owner: UiMountedInstanceIdentity,
        portal_identity: u64,
    ) -> Self {
        Self {
            owner,
            portal_identity,
        }
    }

    pub const fn owner(self) -> UiMountedInstanceIdentity {
        self.owner
    }

    pub const fn portal_identity(self) -> u64 {
        self.portal_identity
    }
}
