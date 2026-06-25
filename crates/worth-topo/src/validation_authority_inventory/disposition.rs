#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthValidationAuthorityDisposition {
    Migrate,
    Delete,
    Cap,
    QueryAccessGap,
    OutOfScope,
}

impl WorthValidationAuthorityDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Migrate => "migrate",
            Self::Delete => "delete",
            Self::Cap => "cap",
            Self::QueryAccessGap => "query-access-gap",
            Self::OutOfScope => "out-of-scope",
        }
    }
}
