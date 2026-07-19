#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryMilestoneClosureStatus {
    Open,
    Partial,
    Closed,
}

impl WorthQueryMilestoneClosureStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Partial => "partial",
            Self::Closed => "closed",
        }
    }
}
