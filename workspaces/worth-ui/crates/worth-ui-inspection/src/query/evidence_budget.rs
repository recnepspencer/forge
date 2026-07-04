#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceBudget {
    Narrow,
    Ordinary,
    Expanded,
}

impl UiEvidenceBudget {
    pub fn narrow() -> Self {
        Self::Narrow
    }

    pub fn ordinary() -> Self {
        Self::Ordinary
    }

    pub fn expanded() -> Self {
        Self::Expanded
    }
}

impl Default for UiEvidenceBudget {
    fn default() -> Self {
        Self::ordinary()
    }
}
