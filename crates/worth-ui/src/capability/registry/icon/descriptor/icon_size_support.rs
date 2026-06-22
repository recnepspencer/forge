#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconSizeSupport {
    Scalable,
    Fixed,
    Missing,
}

impl IconSizeSupport {
    pub fn scalable() -> Self {
        Self::Scalable
    }

    pub fn fixed() -> Self {
        Self::Fixed
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::Missing
    }

    pub(crate) fn is_missing(self) -> bool {
        matches!(self, Self::Missing)
    }

    pub(crate) fn digest_basis(self) -> &'static str {
        match self {
            Self::Scalable => "scalable",
            Self::Fixed => "fixed",
            Self::Missing => "missing",
        }
    }
}
