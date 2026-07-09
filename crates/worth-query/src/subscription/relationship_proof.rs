#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionRelationshipProofPosture {
    NotRequired,
    Admitted,
    Drifted,
}

impl QuerySubscriptionRelationshipProofPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Admitted => "admitted",
            Self::Drifted => "drifted",
        }
    }

    pub(super) fn admits_subscription(&self) -> bool {
        match self {
            Self::NotRequired | Self::Admitted => true,
            Self::Drifted => false,
        }
    }
}
