#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublicJourneyKind {
    PlatformEntry,
    Continuation,
    SignalFacing,
    ContributionComposed,
    HelperProjection,
    GroupedAuthoring,
    Recovery,
}

impl WorthQueryPublicJourneyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlatformEntry => "platform_entry",
            Self::Continuation => "continuation",
            Self::SignalFacing => "signal_facing",
            Self::ContributionComposed => "contribution_composed",
            Self::HelperProjection => "helper_projection",
            Self::GroupedAuthoring => "grouped_authoring",
            Self::Recovery => "recovery",
        }
    }
}
