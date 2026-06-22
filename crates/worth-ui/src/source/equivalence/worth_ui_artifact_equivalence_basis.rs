#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiArtifactEquivalenceBasis {
    SemanticArtifactMeaning,
}

impl WorthUiArtifactEquivalenceBasis {
    pub(crate) fn semantic() -> Self {
        Self::SemanticArtifactMeaning
    }
}
