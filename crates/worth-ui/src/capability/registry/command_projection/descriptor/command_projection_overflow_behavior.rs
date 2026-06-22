/// Overflow posture for crowded command projection surfaces.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CommandProjectionOverflowBehavior {
    NoOverflow,
    CollapseToMore,
    ScrollWithinSurface,
}

impl CommandProjectionOverflowBehavior {
    pub fn no_overflow() -> Self {
        Self::NoOverflow
    }

    pub fn collapse_to_more() -> Self {
        Self::CollapseToMore
    }

    pub fn scroll_within_surface() -> Self {
        Self::ScrollWithinSurface
    }

    pub fn digest_basis(self) -> &'static str {
        match self {
            Self::NoOverflow => "no_overflow",
            Self::CollapseToMore => "collapse_to_more",
            Self::ScrollWithinSurface => "scroll_within_surface",
        }
    }
}
