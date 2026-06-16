use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumedContinuityAuthorityIdentity {
    label: String,
    evidence_identity: ForgeQueryEvidenceIdentity,
}

impl ConsumedContinuityAuthorityIdentity {
    pub(crate) fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        let evidence_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::ProjectionConsumedContinuityAuthorityIdentity,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("authority_identity"),
            label.as_str(),
        )
        .seal();
        Self {
            label,
            evidence_identity,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.evidence_identity
    }
}
use crate::runtime::{
    ForgeQueryContinuityMutationFamily, ForgeQueryContinuityOutcomeClass,
    ForgeQueryMutationTargetClass,
};
use forge_foundational::facade::AspectKey;
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumedEntityIdentityFact {
    source_row_identity: String,
    entity_identity: ForgeQueryEntityIdentity,
}

impl ConsumedEntityIdentityFact {
    pub fn source_row_identity(&self) -> &str {
        &self.source_row_identity
    }

    pub fn entity_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.entity_identity
    }

    pub(crate) fn new(
        source_row_identity: impl Into<String>,
        entity_identity: ForgeQueryEntityIdentity,
    ) -> Self {
        Self {
            source_row_identity: source_row_identity.into(),
            entity_identity,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumedViewLocalIdentityFact {
    source_row_identity: String,
    view_local_identity: String,
}

impl ConsumedViewLocalIdentityFact {
    pub fn source_row_identity(&self) -> &str {
        &self.source_row_identity
    }

    pub fn view_local_identity(&self) -> &str {
        &self.view_local_identity
    }

    pub(crate) fn new(
        source_row_identity: impl Into<String>,
        view_local_identity: impl Into<String>,
    ) -> Self {
        Self {
            source_row_identity: source_row_identity.into(),
            view_local_identity: view_local_identity.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsumedMembershipFact {
    source_row_identity: String,
    member_identity: Value,
    grouping_aspect: AspectKey,
    grouping_value: Value,
}

impl ConsumedMembershipFact {
    pub fn source_row_identity(&self) -> &str {
        &self.source_row_identity
    }

    pub fn member_identity(&self) -> &Value {
        &self.member_identity
    }

    pub fn grouping_aspect(&self) -> &str {
        self.grouping_aspect.as_str()
    }

    pub fn native_grouping_aspect_key(&self) -> &AspectKey {
        &self.grouping_aspect
    }

    pub fn grouping_value(&self) -> &Value {
        &self.grouping_value
    }

    pub(crate) fn new(
        source_row_identity: impl Into<String>,
        member_identity: Value,
        grouping_aspect: AspectKey,
        grouping_value: Value,
    ) -> Self {
        Self {
            source_row_identity: source_row_identity.into(),
            member_identity,
            grouping_aspect,
            grouping_value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsumedFieldValueFact {
    source_row_identity: String,
    field_key: String,
    value: Value,
}

impl ConsumedFieldValueFact {
    pub fn source_row_identity(&self) -> &str {
        &self.source_row_identity
    }

    pub fn field_key(&self) -> &str {
        &self.field_key
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub(crate) fn new(
        source_row_identity: impl Into<String>,
        field_key: impl Into<String>,
        value: Value,
    ) -> Self {
        Self {
            source_row_identity: source_row_identity.into(),
            field_key: field_key.into(),
            value,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumedTargetIdentityFact {
    target_identity: ForgeQueryEntityIdentity,
}

impl ConsumedTargetIdentityFact {
    pub fn target_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.target_identity
    }

    pub(crate) fn new(target_identity: ForgeQueryEntityIdentity) -> Self {
        Self { target_identity }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumedSourceReferenceFact {
    label: &'static str,
    identity: String,
}

impl ConsumedSourceReferenceFact {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub(crate) fn new(label: &'static str, identity: impl Into<String>) -> Self {
        Self {
            label,
            identity: identity.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumedEffectContinuityFact {
    family: ForgeQueryContinuityMutationFamily,
    outcome_class: ForgeQueryContinuityOutcomeClass,
    prior_authoritative_identity: ConsumedContinuityAuthorityIdentity,
    successor_authoritative_identities: Vec<ConsumedContinuityAuthorityIdentity>,
    resolved_target_entity_identity: Option<ForgeQueryEntityIdentity>,
    target_collection: Option<String>,
    lineage_digest: String,
    continuity_resolution_digest: String,
}

impl ConsumedEffectContinuityFact {
    pub fn family(&self) -> ForgeQueryContinuityMutationFamily {
        self.family
    }

    pub fn outcome_class(&self) -> ForgeQueryContinuityOutcomeClass {
        self.outcome_class
    }

    pub fn prior_authoritative_identity(&self) -> &ConsumedContinuityAuthorityIdentity {
        &self.prior_authoritative_identity
    }

    pub fn prior_authoritative_identity_label(&self) -> &str {
        self.prior_authoritative_identity.label()
    }

    pub fn successor_authoritative_identities(&self) -> &[ConsumedContinuityAuthorityIdentity] {
        &self.successor_authoritative_identities
    }

    pub fn resolved_target_entity_identity(&self) -> Option<&ForgeQueryEntityIdentity> {
        self.resolved_target_entity_identity.as_ref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn lineage_digest(&self) -> &str {
        &self.lineage_digest
    }

    pub fn continuity_resolution_digest(&self) -> &str {
        &self.continuity_resolution_digest
    }

    pub(crate) fn new(
        family: ForgeQueryContinuityMutationFamily,
        outcome_class: ForgeQueryContinuityOutcomeClass,
        prior_authoritative_identity: ConsumedContinuityAuthorityIdentity,
        successor_authoritative_identities: Vec<ConsumedContinuityAuthorityIdentity>,
        resolved_target_entity_identity: Option<ForgeQueryEntityIdentity>,
        target_collection: Option<String>,
        lineage_digest: impl Into<String>,
        continuity_resolution_digest: impl Into<String>,
    ) -> Self {
        Self {
            family,
            outcome_class,
            prior_authoritative_identity,
            successor_authoritative_identities,
            resolved_target_entity_identity,
            target_collection,
            lineage_digest: lineage_digest.into(),
            continuity_resolution_digest: continuity_resolution_digest.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConsumedRelationEndpointFact {
    MutationTarget {
        target_class: ForgeQueryMutationTargetClass,
        collection: Option<String>,
        entity_identity: Option<ForgeQueryEntityIdentity>,
    },
    GroupedProjection {
        source_row_identity: String,
        member_identity: Value,
        grouping_aspect: AspectKey,
        grouping_value: Value,
    },
}

impl ConsumedRelationEndpointFact {
    pub fn target_class(&self) -> Option<ForgeQueryMutationTargetClass> {
        match self {
            Self::MutationTarget { target_class, .. } => Some(*target_class),
            Self::GroupedProjection { .. } => None,
        }
    }

    pub fn collection(&self) -> Option<&str> {
        match self {
            Self::MutationTarget { collection, .. } => collection.as_deref(),
            Self::GroupedProjection { .. } => None,
        }
    }

    pub fn entity_identity(&self) -> Option<&ForgeQueryEntityIdentity> {
        match self {
            Self::MutationTarget {
                entity_identity, ..
            } => entity_identity.as_ref(),
            Self::GroupedProjection { .. } => None,
        }
    }

    pub fn source_row_identity(&self) -> Option<&str> {
        match self {
            Self::GroupedProjection {
                source_row_identity,
                ..
            } => Some(source_row_identity),
            Self::MutationTarget { .. } => None,
        }
    }

    pub fn member_identity(&self) -> Option<&Value> {
        match self {
            Self::GroupedProjection {
                member_identity, ..
            } => Some(member_identity),
            Self::MutationTarget { .. } => None,
        }
    }

    pub fn grouping_aspect(&self) -> Option<&str> {
        match self {
            Self::GroupedProjection {
                grouping_aspect, ..
            } => Some(grouping_aspect.as_str()),
            Self::MutationTarget { .. } => None,
        }
    }

    pub fn native_grouping_aspect_key(&self) -> Option<&AspectKey> {
        match self {
            Self::GroupedProjection {
                grouping_aspect, ..
            } => Some(grouping_aspect),
            Self::MutationTarget { .. } => None,
        }
    }

    pub fn grouping_value(&self) -> Option<&Value> {
        match self {
            Self::GroupedProjection { grouping_value, .. } => Some(grouping_value),
            Self::MutationTarget { .. } => None,
        }
    }

    pub(crate) fn new(
        target_class: ForgeQueryMutationTargetClass,
        collection: Option<String>,
        entity_identity: Option<ForgeQueryEntityIdentity>,
    ) -> Self {
        Self::MutationTarget {
            target_class,
            collection,
            entity_identity,
        }
    }

    pub(crate) fn grouped(
        source_row_identity: impl Into<String>,
        member_identity: Value,
        grouping_aspect: AspectKey,
        grouping_value: Value,
    ) -> Self {
        Self::GroupedProjection {
            source_row_identity: source_row_identity.into(),
            member_identity,
            grouping_aspect,
            grouping_value,
        }
    }
}
