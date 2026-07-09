#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryMutationFamily {
    Insert,
    Update,
    Assertion,
    Delete,
}

impl WorthQueryMutationFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Update => "update",
            Self::Assertion => "assertion",
            Self::Delete => "delete",
        }
    }
}

impl std::fmt::Display for WorthQueryMutationFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
