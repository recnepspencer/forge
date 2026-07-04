#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceRichness {
    RefsOnly,
    Summary,
    MaterializedDetail,
}

impl UiEvidenceRichness {
    pub fn refs_only() -> Self {
        Self::RefsOnly
    }

    pub fn summary() -> Self {
        Self::Summary
    }

    pub fn materialized_detail() -> Self {
        Self::MaterializedDetail
    }
}
