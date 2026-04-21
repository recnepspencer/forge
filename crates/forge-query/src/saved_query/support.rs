#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SavedQueryComplexityStatus {
    Verified,
    Debt,
}

impl SavedQueryComplexityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Debt => "debt",
        }
    }
}

pub fn runtime_backed_saved_query_support_profile() -> Vec<(
    super::artifact::SavedQueryPersistenceFamily,
    SavedQueryComplexityStatus,
)> {
    vec![(
        super::artifact::SavedQueryPersistenceFamily::EphemeralProcessOwned,
        SavedQueryComplexityStatus::Debt,
    )]
}
