use crate::UiEvidenceRichness;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceBudget {
    Bounded { richness: UiEvidenceRichness },
    Exhaustive,
}

impl UiEvidenceBudget {
    pub fn bounded(richness: UiEvidenceRichness) -> Self {
        Self::Bounded { richness }
    }

    pub fn exhaustive() -> Self {
        Self::Exhaustive
    }

    pub fn richness(self) -> UiEvidenceRichness {
        match self {
            Self::Bounded { richness } => richness,
            Self::Exhaustive => UiEvidenceRichness::full(),
        }
    }
}

impl Default for UiEvidenceBudget {
    fn default() -> Self {
        Self::bounded(UiEvidenceRichness::summary())
    }
}
