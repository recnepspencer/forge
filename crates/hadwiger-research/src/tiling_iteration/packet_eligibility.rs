#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TilingIterationActionEligibility {
    EligibleNextCheckerInput,
    Blocked,
    AdvisoryOnly,
    Unsupported,
}

impl TilingIterationActionEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EligibleNextCheckerInput => "eligible_next_checker_input",
            Self::Blocked => "blocked",
            Self::AdvisoryOnly => "advisory_only",
            Self::Unsupported => "unsupported",
        }
    }

    pub fn can_execute(self) -> bool {
        false
    }
}
