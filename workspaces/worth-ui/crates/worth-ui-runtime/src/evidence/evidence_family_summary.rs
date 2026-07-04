use super::UiEvidenceFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiEvidenceFamilySummary {
    family: UiEvidenceFamily,
    ref_count: usize,
}

impl UiEvidenceFamilySummary {
    pub(crate) fn new(family: UiEvidenceFamily, ref_count: usize) -> Self {
        Self { family, ref_count }
    }

    pub fn family(&self) -> UiEvidenceFamily {
        self.family
    }

    pub fn ref_count(&self) -> usize {
        self.ref_count
    }
}
