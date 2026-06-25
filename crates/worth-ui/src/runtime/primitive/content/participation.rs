#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveContentParticipationPosture {
    Present,
    Absent,
    HiddenFromPaint,
    HiddenFromAccessibility,
    Inert,
    Loading,
    Unsupported,
    Denied,
}

impl WorthUiPrimitiveContentParticipationPosture {
    pub fn participates_in_layout(self) -> bool {
        matches!(
            self,
            Self::Present
                | Self::HiddenFromPaint
                | Self::HiddenFromAccessibility
                | Self::Inert
                | Self::Loading
        )
    }

    pub fn participates_in_paint(self) -> bool {
        matches!(
            self,
            Self::Present | Self::HiddenFromAccessibility | Self::Inert | Self::Loading
        )
    }

    pub fn participates_in_accessibility(self) -> bool {
        matches!(
            self,
            Self::Present | Self::HiddenFromPaint | Self::Inert | Self::Loading
        )
    }

    pub fn participates_in_events(self) -> bool {
        matches!(
            self,
            Self::Present | Self::HiddenFromAccessibility | Self::Loading
        )
    }

    pub fn retains_state(self) -> bool {
        !matches!(self, Self::Absent | Self::Denied | Self::Unsupported)
    }

    pub fn token(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Absent => "absent",
            Self::HiddenFromPaint => "hidden_from_paint",
            Self::HiddenFromAccessibility => "hidden_from_accessibility",
            Self::Inert => "inert",
            Self::Loading => "loading",
            Self::Unsupported => "unsupported",
            Self::Denied => "denied",
        }
    }
}
