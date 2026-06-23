use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::{
    ForgeQueryMutationSymbolIdentity, ForgeQueryMutationTargetCollectionIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQuerySymbolicTargetReferenceFamily {
    SameBatchDeclaredTarget,
}

impl ForgeQuerySymbolicTargetReferenceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SameBatchDeclaredTarget => "same-batch-declared-target",
        }
    }
}

impl std::fmt::Display for ForgeQuerySymbolicTargetReferenceFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySymbolicTargetReference {
    family: ForgeQuerySymbolicTargetReferenceFamily,
    symbol: ForgeQueryMutationSymbolIdentity,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
}

impl ForgeQuerySymbolicTargetReference {
    pub fn new(symbol: impl Into<String>) -> Result<Self, ForgeQueryWorkspaceError> {
        let symbol = symbol.into();
        if symbol.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "symbolic target reference may not be empty",
            ));
        }
        Ok(Self {
            family: ForgeQuerySymbolicTargetReferenceFamily::SameBatchDeclaredTarget,
            symbol: ForgeQueryMutationSymbolIdentity::new("symbolic-target-reference", symbol),
            target_collection: None,
        })
    }

    pub fn in_target_collection(
        mut self,
        collection: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let collection = collection.into();
        if collection.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "symbolic target collection may not be empty",
            ));
        }
        self.target_collection = Some(ForgeQueryMutationTargetCollectionIdentity::new(
            "symbolic-target-reference",
            collection,
        ));
        Ok(self)
    }

    pub fn family(&self) -> ForgeQuerySymbolicTargetReferenceFamily {
        self.family
    }

    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    pub fn symbol_identity(&self) -> &ForgeQueryMutationSymbolIdentity {
        &self.symbol
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection
            .as_ref()
            .map(ForgeQueryMutationTargetCollectionIdentity::as_str)
    }

    pub fn target_collection_identity(
        &self,
    ) -> Option<&ForgeQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQuerySymbolicTargetReferenceDenialKind {
    RequiresBatchContext,
    UnresolvedSameBatchTarget,
    CollectionMismatch,
}

impl ForgeQuerySymbolicTargetReferenceDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequiresBatchContext => "requires-batch-context",
            Self::UnresolvedSameBatchTarget => "unresolved-same-batch-target",
            Self::CollectionMismatch => "collection-mismatch",
        }
    }
}

impl std::fmt::Display for ForgeQuerySymbolicTargetReferenceDenialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySymbolicTargetReferenceDenial {
    reference: ForgeQuerySymbolicTargetReference,
    kind: ForgeQuerySymbolicTargetReferenceDenialKind,
    message: String,
    denial_digest: String,
}

impl ForgeQuerySymbolicTargetReferenceDenial {
    pub(crate) fn new(
        reference: &ForgeQuerySymbolicTargetReference,
        kind: ForgeQuerySymbolicTargetReferenceDenialKind,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let denial_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "symbolic-target-reference-denial",
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("family"),
                    reference.family().as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("symbol"),
                    reference.symbol_identity().evidence_identity(),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("collection"),
                    reference
                        .target_collection_identity()
                        .map(ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .field_value(ForgeQueryEvidenceTag::new("message"), &message)
                .seal()
                .as_str()
                .to_string();
        Self {
            reference: reference.clone(),
            kind,
            message,
            denial_digest,
        }
    }

    pub fn reference(&self) -> &ForgeQuerySymbolicTargetReference {
        &self.reference
    }

    pub fn kind(&self) -> ForgeQuerySymbolicTargetReferenceDenialKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for ForgeQuerySymbolicTargetReferenceDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "symbolic target reference `{}` denied for symbol `{}`: {}",
            self.kind,
            self.reference.symbol(),
            self.message
        )
    }
}
