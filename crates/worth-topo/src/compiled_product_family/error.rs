use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyCompiledProductFamilyErrorKind {
    NoDeclaredFamilyForConsumer,
    SchemaVocabularyAdmissionFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyCompiledProductFamilyError {
    kind: TopologyCompiledProductFamilyErrorKind,
    detail: String,
}

impl TopologyCompiledProductFamilyError {
    pub fn new(kind: TopologyCompiledProductFamilyErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }
}
