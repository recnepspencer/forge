use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryWorkspaceError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryExistingTruthBindingFamily {
    DirectEntityIdentity,
    DirectRelationIdentity,
}

impl ForgeQueryExistingTruthBindingFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectEntityIdentity => "direct-entity-identity",
            Self::DirectRelationIdentity => "direct-relation-identity",
        }
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthBindingFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingEntityTarget {
    authoritative_identity: String,
    resolved_entity_identity: String,
    target_collection: Option<String>,
}

impl ForgeQueryExistingEntityTarget {
    pub fn new(
        authoritative_identity: impl Into<String>,
        resolved_entity_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            authoritative_identity: normalize_non_empty(
                authoritative_identity.into(),
                "existing-truth authoritative identity may not be empty",
            )?,
            resolved_entity_identity: normalize_non_empty(
                resolved_entity_identity.into(),
                "existing-truth resolved entity identity may not be empty",
            )?,
            target_collection: None,
        })
    }

    pub fn in_target_collection(
        mut self,
        collection: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        self.target_collection = Some(normalize_non_empty(
            collection.into(),
            "existing-truth target collection may not be empty",
        )?);
        Ok(self)
    }

    pub(crate) fn into_parts(self) -> (String, String, Option<String>) {
        (
            self.authoritative_identity,
            self.resolved_entity_identity,
            self.target_collection,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingRelationTarget {
    authoritative_identity: String,
    resolved_relation_identity: String,
    target_collection: Option<String>,
}

impl ForgeQueryExistingRelationTarget {
    pub fn new(
        authoritative_identity: impl Into<String>,
        resolved_relation_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            authoritative_identity: normalize_non_empty(
                authoritative_identity.into(),
                "existing-truth authoritative identity may not be empty",
            )?,
            resolved_relation_identity: normalize_non_empty(
                resolved_relation_identity.into(),
                "existing-truth resolved relation identity may not be empty",
            )?,
            target_collection: None,
        })
    }

    pub fn in_target_collection(
        mut self,
        collection: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        self.target_collection = Some(normalize_non_empty(
            collection.into(),
            "existing-truth target collection may not be empty",
        )?);
        Ok(self)
    }

    pub(crate) fn into_parts(self) -> (String, String, Option<String>) {
        (
            self.authoritative_identity,
            self.resolved_relation_identity,
            self.target_collection,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthTargetBinding {
    family: ForgeQueryExistingTruthBindingFamily,
    authoritative_identity: String,
    resolved_target_identity: String,
    target_collection: Option<String>,
}

impl ForgeQueryExistingTruthTargetBinding {
    pub fn direct_entity(
        authoritative_identity: impl Into<String>,
        resolved_entity_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::from_entity_target(ForgeQueryExistingEntityTarget::new(
            authoritative_identity,
            resolved_entity_identity,
        )?)
    }

    pub fn direct_relation(
        authoritative_identity: impl Into<String>,
        resolved_relation_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::from_relation_target(ForgeQueryExistingRelationTarget::new(
            authoritative_identity,
            resolved_relation_identity,
        )?)
    }

    pub fn from_entity_target(
        target: ForgeQueryExistingEntityTarget,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let (authoritative_identity, resolved_target_identity, target_collection) =
            target.into_parts();
        Self::new(
            ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity,
            authoritative_identity,
            resolved_target_identity,
            target_collection,
        )
    }

    pub fn from_relation_target(
        target: ForgeQueryExistingRelationTarget,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let (authoritative_identity, resolved_target_identity, target_collection) =
            target.into_parts();
        Self::new(
            ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity,
            authoritative_identity,
            resolved_target_identity,
            target_collection,
        )
    }

    pub fn in_target_collection(
        mut self,
        collection: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        self.target_collection = Some(normalize_non_empty(
            collection.into(),
            "existing-truth target collection may not be empty",
        )?);
        Ok(self)
    }

    pub fn family(&self) -> ForgeQueryExistingTruthBindingFamily {
        self.family
    }

    pub fn authoritative_identity(&self) -> &str {
        &self.authoritative_identity
    }

    pub fn resolved_target_identity(&self) -> &str {
        &self.resolved_target_identity
    }

    pub fn resolved_entity_identity(&self) -> &str {
        &self.resolved_target_identity
    }

    pub fn resolved_relation_identity(&self) -> &str {
        &self.resolved_target_identity
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
            format!("resolved:{}", self.resolved_target_identity),
            format!("collection:{}", self.target_collection().unwrap_or("none")),
        ])
    }

    fn new(
        family: ForgeQueryExistingTruthBindingFamily,
        authoritative_identity: String,
        resolved_target_identity: String,
        target_collection: Option<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            family,
            authoritative_identity: normalize_non_empty(
                authoritative_identity,
                "existing-truth authoritative identity may not be empty",
            )?,
            resolved_target_identity: normalize_non_empty(
                resolved_target_identity,
                "existing-truth resolved target identity may not be empty",
            )?,
            target_collection,
        })
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
    #[cfg(test)]
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
            format!("resolved:{}", binding.resolved_target_identity()),
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

fn normalize_non_empty(value: String, message: &str) -> Result<String, ForgeQueryWorkspaceError> {
    if value.trim().is_empty() {
        return Err(ForgeQueryWorkspaceError::new(message));
    }
    Ok(value)
}
