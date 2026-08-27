#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiWindowFocus {
    Focused,
    Unfocused,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiFocusVisibleModality {
    Initial,
    Keyboard,
    Pointer,
}

impl UiWindowFocus {
    pub(in crate::runtime) const fn from_host_observation(focused: bool) -> Self {
        if focused {
            Self::Focused
        } else {
            Self::Unfocused
        }
    }

    pub(in crate::runtime) const fn is_focused(self) -> bool {
        matches!(self, Self::Focused)
    }
}

impl UiFocusVisibleModality {
    pub(in crate::runtime) const fn is_keyboard(self) -> bool {
        matches!(self, Self::Keyboard)
    }
}
