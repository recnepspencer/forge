/// Declared command-route precedence level.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiCommandRouteScope {
    Application,
    Surface,
    ActiveRegion,
    FocusedControl,
    ActivePortal,
}

impl UiCommandRouteScope {
    pub(crate) const fn precedence(self) -> u8 {
        match self {
            Self::Application => 0,
            Self::Surface => 1,
            Self::ActiveRegion => 2,
            Self::FocusedControl => 3,
            Self::ActivePortal => 4,
        }
    }
}
