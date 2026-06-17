use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::{
    ForgeQueryMutationAuthorityIdentity, ForgeQueryMutationTargetCollectionIdentity,
};

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

    pub fn bridge_backed_support_family(self) -> &'static str {
        match self {
            Self::DirectEntityIdentity => "direct_entity_identity",
            Self::DirectRelationIdentity => "direct_relation_identity",
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
    authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    resolved_entity_identity: ForgeQueryEntityIdentity,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
}

impl ForgeQueryExistingEntityTarget {
    pub fn new(
        authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        resolved_entity_identity: ForgeQueryEntityIdentity,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            authoritative_identity,
            resolved_entity_identity,
            target_collection: None,
        })
    }

    pub fn in_target_collection(
        mut self,
        collection: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        self.target_collection = Some(ForgeQueryMutationTargetCollectionIdentity::new(
            "existing-truth-binding",
            normalize_non_empty(
                collection.into(),
                "existing-truth target collection may not be empty",
            )?,
        ));
        Ok(self)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryMutationAuthorityIdentity,
        ForgeQueryEntityIdentity,
        Option<ForgeQueryMutationTargetCollectionIdentity>,
    ) {
        (
            self.authoritative_identity,
            self.resolved_entity_identity,
            self.target_collection,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingRelationTarget {
    authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    resolved_relation_identity: ForgeQueryEntityIdentity,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
}

impl ForgeQueryExistingRelationTarget {
    pub fn new(
        authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        resolved_relation_identity: ForgeQueryEntityIdentity,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            authoritative_identity,
            resolved_relation_identity,
            target_collection: None,
        })
    }

    pub fn in_target_collection(
        mut self,
        collection: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        self.target_collection = Some(ForgeQueryMutationTargetCollectionIdentity::new(
            "existing-truth-binding",
            normalize_non_empty(
                collection.into(),
                "existing-truth target collection may not be empty",
            )?,
        ));
        Ok(self)
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryMutationAuthorityIdentity,
        ForgeQueryEntityIdentity,
        Option<ForgeQueryMutationTargetCollectionIdentity>,
    ) {
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
    authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    resolved_target_identity: ForgeQueryEntityIdentity,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
    binding_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryExistingTruthTargetBinding {
    pub fn direct_entity(
        authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        resolved_entity_identity: ForgeQueryEntityIdentity,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::from_entity_target(ForgeQueryExistingEntityTarget::new(
            authoritative_identity,
            resolved_entity_identity,
        )?)
    }

    pub fn direct_relation(
        authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        resolved_relation_identity: ForgeQueryEntityIdentity,
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
        self.target_collection = Some(ForgeQueryMutationTargetCollectionIdentity::new(
            "existing-truth-binding",
            normalize_non_empty(
                collection.into(),
                "existing-truth target collection may not be empty",
            )?,
        ));
        self.binding_identity = existing_truth_binding_identity(
            self.family,
            &self.authoritative_identity,
            &self.resolved_target_identity,
            self.target_collection.as_ref(),
        );
        Ok(self)
    }

    pub fn family(&self) -> ForgeQueryExistingTruthBindingFamily {
        self.family
    }

    pub fn authoritative_identity(&self) -> &ForgeQueryMutationAuthorityIdentity {
        &self.authoritative_identity
    }

    pub fn resolved_target_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.resolved_target_identity
    }

    pub fn resolved_entity_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.resolved_target_identity
    }

    pub fn resolved_entity_artifact_identity(&self) -> ForgeQueryEntityIdentity {
        self.resolved_target_identity.clone()
    }

    pub fn resolved_relation_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.resolved_target_identity
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection
            .as_ref()
            .map(ForgeQueryMutationTargetCollectionIdentity::as_str)
    }

    pub fn binding_digest(&self) -> String {
        self.binding_identity.as_str().to_string()
    }

    pub fn binding_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.binding_identity
    }

    fn new(
        family: ForgeQueryExistingTruthBindingFamily,
        authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        resolved_target_identity: ForgeQueryEntityIdentity,
        target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let binding_identity = existing_truth_binding_identity(
            family,
            &authoritative_identity,
            &resolved_target_identity,
            target_collection.as_ref(),
        );
        Ok(Self {
            family,
            authoritative_identity,
            resolved_target_identity,
            target_collection,
            binding_identity,
        })
    }
}

fn existing_truth_binding_identity(
    family: ForgeQueryExistingTruthBindingFamily,
    authoritative_identity: &ForgeQueryMutationAuthorityIdentity,
    resolved_target_identity: &ForgeQueryEntityIdentity,
    target_collection: Option<&ForgeQueryMutationTargetCollectionIdentity>,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "existing-truth-binding")
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("outcome"),
            crate::runtime::ForgeQueryExistingTruthBindingOutcome::ExistingAuthoritativeTarget
                .as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("authoritative"),
            authoritative_identity.evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("resolved"),
            &resolved_target_identity.evidence_identity(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("collection"),
            target_collection
                .map(crate::runtime::ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
        )
        .seal()
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
        let denial_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
        )
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
                .target_collection()
                .map(|collection| {
                    crate::runtime::ForgeQueryMutationTargetCollectionIdentity::new(
                        "existing-truth-binding",
                        collection,
                    )
                })
                .as_ref()
                .map(crate::runtime::ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
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

fn normalize_non_empty(value: String, message: &str) -> Result<String, ForgeQueryWorkspaceError> {
    if value.trim().is_empty() {
        return Err(ForgeQueryWorkspaceError::new(message));
    }
    Ok(value)
}
