use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompiledProductSemanticGraphVocabularyErrorKind {
    EmptyAuthorityOwner,
    EmptyAuthoritySurface,
    EmptyAuthorityDigest,
    EmptyAuthorityInstanceKind,
    EmptyAuthorityInstanceValue,
    EmptyLocalityKind,
    EmptyLocalityDigest,
    EmptyPriorProofDigest,
    EmptyEquivalencePolicyName,
    EmptyEquivalenceDimension,
    EmptyStageDigest,
    EmptyReusePosture,
    EmptyRebuildReason,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompiledProductSemanticGraphVocabularyError {
    kind: CompiledProductSemanticGraphVocabularyErrorKind,
    detail: String,
}

impl CompiledProductSemanticGraphVocabularyError {
    pub(crate) fn new(
        kind: CompiledProductSemanticGraphVocabularyErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> CompiledProductSemanticGraphVocabularyErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
