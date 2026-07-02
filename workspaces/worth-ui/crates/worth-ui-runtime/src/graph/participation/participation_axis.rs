#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphParticipationAxis {
    Exists,
    Mounted,
    Visible,
    Layout,
    HitTest,
    Focus,
    Accessibility,
    Paint,
    Input,
    QueryBound,
    ServiceBound,
    Diagnostic,
}

impl UiGraphParticipationAxis {
    pub const COUNT: usize = 12;
    pub const ALL: [Self; Self::COUNT] = [
        Self::Exists,
        Self::Mounted,
        Self::Visible,
        Self::Layout,
        Self::HitTest,
        Self::Focus,
        Self::Accessibility,
        Self::Paint,
        Self::Input,
        Self::QueryBound,
        Self::ServiceBound,
        Self::Diagnostic,
    ];

    pub const fn as_index(self) -> usize {
        match self {
            Self::Exists => 0,
            Self::Mounted => 1,
            Self::Visible => 2,
            Self::Layout => 3,
            Self::HitTest => 4,
            Self::Focus => 5,
            Self::Accessibility => 6,
            Self::Paint => 7,
            Self::Input => 8,
            Self::QueryBound => 9,
            Self::ServiceBound => 10,
            Self::Diagnostic => 11,
        }
    }
}
