use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryWorkspaceError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryExistingTruthBindingFamily {
    DirectEntityIdentity,
}

impl ForgeQueryExistingTruthBindingFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectEntityIdentity => "direct-entity-identity",
        }
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthBindingFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthTargetBinding {
    family: ForgeQueryExistingTruthBindingFamily,
    authoritative_identity: String,
    resolved_entity_identity: String,
    target_collection: Option<String>,
}

impl ForgeQueryExistingTruthTargetBinding {
    pub fn direct_entity(
        authoritative_identity: impl Into<String>,
        resolved_entity_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let authoritative_identity = authoritative_identity.into();
        let resolved_entity_identity = resolved_entity_identity.into();
        if authoritative_identity.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "existing-truth authoritative identity may not be empty",
            ));
        }
        if resolved_entity_identity.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "existing-truth resolved entity identity may not be empty",
            ));
        }
        Ok(Self {
            family: ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity,
            authoritative_identity,
            resolved_entity_identity,
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
                "existing-truth target collection may not be empty",
            ));
        }
        self.target_collection = Some(collection);
        Ok(self)
    }

    pub fn family(&self) -> ForgeQueryExistingTruthBindingFamily {
        self.family
    }

    pub fn authoritative_identity(&self) -> &str {
        &self.authoritative_identity
    }

    pub fn resolved_entity_identity(&self) -> &str {
        &self.resolved_entity_identity
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn binding_digest(&self) -> String {
        hash_parts(&[
            "bridge-existing-truth-binding".to_string(),
            format!("family:{:?}", self.family),
            format!(
                "outcome:{:?}",
                crate::runtime::ForgeQueryExistingTruthBindingOutcome::ExistingAuthoritativeTarget
            ),
            format!("authoritative:{}", self.authoritative_identity),
            format!("resolved:{}", self.resolved_entity_identity),
            format!("collection:{}", self.target_collection().unwrap_or("none")),
        ])
    }
}

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
    pub(crate) fn new(
        binding: &ForgeQueryExistingTruthTargetBinding,
        kind: ForgeQueryExistingTruthBindingDenialKind,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let denial_digest = hash_parts(&[
            "forge_query_existing_truth_binding_denial_v1".to_string(),
            format!("family:{}", binding.family()),
            format!("authoritative:{}", binding.authoritative_identity()),
            format!("resolved:{}", binding.resolved_entity_identity()),
            format!("collection:{}", binding.target_collection().unwrap_or("")),
            format!("kind:{kind}"),
            format!("message:{message}"),
        ]);
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
            self.binding.authoritative_identity(),
            self.message
        )
    }
}

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
