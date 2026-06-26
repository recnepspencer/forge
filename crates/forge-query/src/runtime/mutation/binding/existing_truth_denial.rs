#[cfg(test)]
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::mutation::binding::existing_truth::ForgeQueryExistingTruthTargetBinding;
#[cfg(test)]
use crate::runtime::ForgeQueryMutationTargetCollectionIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryExistingTruthBindingDenialKind {
    UnsupportedFamily,
    ResolvedTargetMissing,
    CollectionMismatch,
}

impl ForgeQueryExistingTruthBindingDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedFamily => "unsupported-family",
            Self::ResolvedTargetMissing => "resolved-target-missing",
            Self::CollectionMismatch => "collection-mismatch",
        }
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthBindingDenialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthBindingDenial {
    binding: ForgeQueryExistingTruthTargetBinding,
    kind: ForgeQueryExistingTruthBindingDenialKind,
    message: String,
    denial_digest: String,
}

impl ForgeQueryExistingTruthBindingDenial {
    #[cfg(test)]
    pub(crate) fn new(
        binding: &ForgeQueryExistingTruthTargetBinding,
        kind: ForgeQueryExistingTruthBindingDenialKind,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let denial_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-binding-denial",
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("family"),
                    binding.family().as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("authoritative"),
                    binding.authoritative_identity().evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("resolved"),
                    &binding.resolved_target_identity().evidence_identity(),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("collection"),
                    binding
                        .target_collection_identity()
                        .map(ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .field_value(ForgeQueryEvidenceTag::new("message"), &message)
                .seal()
                .as_str()
                .to_string();
        Self {
            binding: binding.clone(),
            kind,
            message,
            denial_digest,
        }
    }

    pub fn binding(&self) -> &ForgeQueryExistingTruthTargetBinding {
        &self.binding
    }

    pub fn kind(&self) -> ForgeQueryExistingTruthBindingDenialKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthBindingDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "existing-truth binding `{}` denied for authoritative `{}`: {}",
            self.kind,
            self.binding.authoritative_identity().as_str(),
            self.message
        )
    }
}
