#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphTouchDescriptorKind {
    AuthoritativeMutationBatch,
    ReadFamily,
    LiveRead,
}

impl ForgeQueryGraphTouchDescriptorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeMutationBatch => "authoritative-mutation-batch",
            Self::ReadFamily => "read-family",
            Self::LiveRead => "live-read",
        }
    }
}

impl std::fmt::Display for ForgeQueryGraphTouchDescriptorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
