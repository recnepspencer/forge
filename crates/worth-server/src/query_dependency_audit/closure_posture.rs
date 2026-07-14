#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthServerQueryDependencyClosurePosture {
    Ready,
    Blocked,
    StaticTestOnly,
}

impl WorthServerQueryDependencyClosurePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
            Self::StaticTestOnly => "static-test-only",
        }
    }
}
