#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceRichness {
    Summary,
    Full,
}

impl UiEvidenceRichness {
    pub fn summary() -> Self {
        Self::Summary
    }

    pub fn full() -> Self {
        Self::Full
    }
}
