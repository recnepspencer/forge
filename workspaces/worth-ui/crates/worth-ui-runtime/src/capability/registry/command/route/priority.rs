/// Explicit priority used only after route-scope precedence.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiCommandRoutePriority(i16);

impl UiCommandRoutePriority {
    pub const fn normal() -> Self {
        Self(0)
    }

    pub const fn new(value: i16) -> Self {
        Self(value)
    }

    pub const fn value(self) -> i16 {
        self.0
    }
}
