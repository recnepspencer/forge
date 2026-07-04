#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologySelectedEquivalenceFamilyErrorKind {
    MissingDeclaredFamily,
    SchemaVocabularyAdmissionFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySelectedEquivalenceFamilyError {
    kind: TopologySelectedEquivalenceFamilyErrorKind,
    detail: String,
}

impl TopologySelectedEquivalenceFamilyError {
    pub(crate) fn new(
        kind: TopologySelectedEquivalenceFamilyErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> TopologySelectedEquivalenceFamilyErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
