#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductSessionLifecycle {
    ReadOnlyPreview,
    MutationDraft,
    Closed,
}

impl WorthServerProductSessionLifecycle {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyPreview => "read-only-preview",
            Self::MutationDraft => "mutation-draft",
            Self::Closed => "closed",
        }
    }
}
