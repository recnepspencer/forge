use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryWorkspaceError;

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
    symbol: String,
    target_collection: Option<String>,
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
            symbol,
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
        self.target_collection = Some(collection);
        Ok(self)
    }

    pub fn family(&self) -> ForgeQuerySymbolicTargetReferenceFamily {
        self.family
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
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
        let denial_digest = hash_parts(&[
            "forge_query_symbolic_target_reference_denial_v1".to_string(),
            format!("family:{}", reference.family()),
            format!("symbol:{}", reference.symbol()),
            format!("collection:{}", reference.target_collection().unwrap_or("")),
            format!("kind:{kind}"),
            format!("message:{message}"),
        ]);
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
