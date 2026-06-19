#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerProductSessionLifecycle {
    ReadOnlyPreview,
    MutationDraft,
    Closed,
}

impl ForgeServerProductSessionLifecycle {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyPreview => "read-only-preview",
            Self::MutationDraft => "mutation-draft",
            Self::Closed => "closed",
        }
    }
}
