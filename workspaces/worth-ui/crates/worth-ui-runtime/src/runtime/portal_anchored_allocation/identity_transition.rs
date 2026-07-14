#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPortalAnchorIdentityTransition {
    Preserved {
        identity: super::UiPortalAnchorIdentity,
    },
    TargetReplaced {
        prior: super::UiPortalAnchorIdentity,
        current: super::UiPortalAnchorIdentity,
    },
    CoordinateSpaceReplaced {
        prior: super::UiPortalAnchorIdentity,
        current: super::UiPortalAnchorIdentity,
    },
}

impl UiPortalAnchorIdentityTransition {
    pub(crate) fn classify(
        prior: super::UiPortalAnchorIdentity,
        current: super::UiPortalAnchorIdentity,
    ) -> Self {
        if prior.target() != current.target() {
            Self::TargetReplaced { prior, current }
        } else if prior.coordinate_space() != current.coordinate_space() {
            Self::CoordinateSpaceReplaced { prior, current }
        } else {
            Self::Preserved { identity: current }
        }
    }

    pub const fn current(self) -> super::UiPortalAnchorIdentity {
        match self {
            Self::Preserved { identity } => identity,
            Self::TargetReplaced { current, .. }
            | Self::CoordinateSpaceReplaced { current, .. } => current,
        }
    }
}
