#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicOverflowBehavior {
    Clip,
    ScrollWhenConstrained,
    ExpandParent,
    RejectOverflow,
    MissingForDiagnostics,
}

impl MosaicOverflowBehavior {
    pub fn clip() -> Self {
        Self::Clip
    }

    pub fn scroll_when_constrained() -> Self {
        Self::ScrollWhenConstrained
    }

    pub fn expand_parent() -> Self {
        Self::ExpandParent
    }

    pub fn reject_overflow() -> Self {
        Self::RejectOverflow
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Clip => "clip",
            Self::ScrollWhenConstrained => "scroll_when_constrained",
            Self::ExpandParent => "expand_parent",
            Self::RejectOverflow => "reject_overflow",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
