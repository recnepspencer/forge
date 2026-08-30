use crate::capability::{UiIntent, UiIntentId};

/// Explicit intent destination selected by one successful command route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandRouteDestination(UiIntentId);

impl UiCommandRouteDestination {
    pub const fn for_intent<I: UiIntent>() -> Self {
        Self(I::ID)
    }

    pub const fn intent(self) -> UiIntentId {
        self.0
    }
}
