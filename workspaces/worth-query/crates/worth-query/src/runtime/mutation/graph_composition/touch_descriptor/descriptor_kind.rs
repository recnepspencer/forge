#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryGraphTouchDescriptorKind {
    AuthoritativeMutationBatch,
    ReadFamily,
    LiveRead,
}

impl WorthQueryGraphTouchDescriptorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeMutationBatch => "authoritative-mutation-batch",
            Self::ReadFamily => "read-family",
            Self::LiveRead => "live-read",
        }
    }
}

impl std::fmt::Display for WorthQueryGraphTouchDescriptorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
