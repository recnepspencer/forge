use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlannerOwnedRoutingSemanticGraphVocabularyErrorKind {
    EmptyAuthorityOwner,
    EmptyAdmittedPacketDigest,
    EmptySelectedFamilyName,
    EmptySelectedRouteName,
    EmptySelectedProductName,
    EmptyWitnessReason,
    EmptyDecisionTraceName,
    EmptyPublicProofName,
    EmptyDiagnosticContractName,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlannerOwnedRoutingSemanticGraphVocabularyError {
    kind: PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
    detail: String,
}

impl PlannerOwnedRoutingSemanticGraphVocabularyError {
    pub(crate) fn new(
        kind: PlannerOwnedRoutingSemanticGraphVocabularyErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> PlannerOwnedRoutingSemanticGraphVocabularyErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
