#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition {
    Migrate,
    Delete,
    Cap,
    QueryGap,
}

impl WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Migrate => "migrate",
            Self::Delete => "delete",
            Self::Cap => "cap",
            Self::QueryGap => "query_gap",
        }
    }

    pub const fn is_terminal_or_follow_on(self) -> bool {
        matches!(
            self,
            Self::Migrate | Self::Delete | Self::Cap | Self::QueryGap
        )
    }
}
