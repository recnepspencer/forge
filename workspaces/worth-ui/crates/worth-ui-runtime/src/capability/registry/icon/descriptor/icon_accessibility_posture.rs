#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconAccessibilityPosture {
    Decorative,
    LabelledByConsumer,
    SemanticStandalone,
    Missing,
}

impl IconAccessibilityPosture {
    pub fn decorative() -> Self {
        Self::Decorative
    }

    pub fn labelled_by_consumer() -> Self {
        Self::LabelledByConsumer
    }

    pub fn semantic_standalone() -> Self {
        Self::SemanticStandalone
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::Missing
    }

    pub(crate) fn is_missing(self) -> bool {
        matches!(self, Self::Missing)
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::Decorative => "decorative",
            Self::LabelledByConsumer => "labelled_by_consumer",
            Self::SemanticStandalone => "semantic_standalone",
            Self::Missing => "missing",
        }
    }
}
