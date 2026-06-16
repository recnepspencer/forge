use std::collections::BTreeSet;

use super::identity::compose_materialized_fact_posture_digest;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProjectionFactRequest {
    EntityIdentity,
    ViewLocalIdentity,
    TargetIdentity,
    SourceReference,
    EffectContinuity,
    Membership,
    RelationEndpoint,
    DisplayField(String),
    DerivedScalarField(String),
}

impl ProjectionFactRequest {
    pub fn kind(&self) -> ProjectionFactKind {
        match self {
            Self::EntityIdentity => ProjectionFactKind::EntityIdentity,
            Self::ViewLocalIdentity => ProjectionFactKind::ViewLocalIdentity,
            Self::TargetIdentity => ProjectionFactKind::TargetIdentity,
            Self::SourceReference => ProjectionFactKind::SourceReference,
            Self::EffectContinuity => ProjectionFactKind::EffectContinuity,
            Self::Membership => ProjectionFactKind::Membership,
            Self::RelationEndpoint => ProjectionFactKind::RelationEndpoint,
            Self::DisplayField(_) => ProjectionFactKind::DisplayField,
            Self::DerivedScalarField(_) => ProjectionFactKind::DerivedScalarField,
        }
    }

    pub fn field_key(&self) -> Option<&str> {
        match self {
            Self::DisplayField(field) | Self::DerivedScalarField(field) => Some(field.as_str()),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProjectionFactKind {
    EntityIdentity,
    ViewLocalIdentity,
    TargetIdentity,
    SourceReference,
    EffectContinuity,
    Membership,
    RelationEndpoint,
    DisplayField,
    DerivedScalarField,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionMaterializedFactPostureKind {
    Ordinary,
    TimeOnly,
    AsyncBacked,
    MixedCause,
    Remasked,
}

impl ProjectionMaterializedFactPostureKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::TimeOnly => "time_only",
            Self::AsyncBacked => "async_backed",
            Self::MixedCause => "mixed_cause",
            Self::Remasked => "remasked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionMaterializedFactPosture {
    kind: ProjectionMaterializedFactPostureKind,
    lower_declaration_digest: String,
    basis_digest: String,
    support_evidence_digest: String,
    runtime_origin_digest: Option<String>,
    posture_digest: String,
}

impl ProjectionMaterializedFactPosture {
    pub fn new(
        kind: ProjectionMaterializedFactPostureKind,
        lower_declaration_digest: impl Into<String>,
        basis_digest: impl Into<String>,
        support_evidence_digest: impl Into<String>,
        runtime_origin_digest: Option<String>,
    ) -> Self {
        let lower_declaration_digest = lower_declaration_digest.into();
        let basis_digest = basis_digest.into();
        let support_evidence_digest = support_evidence_digest.into();
        let posture_digest = compose_materialized_fact_posture_digest(
            kind,
            &lower_declaration_digest,
            &basis_digest,
            &support_evidence_digest,
            runtime_origin_digest.as_deref(),
        );
        Self {
            kind,
            lower_declaration_digest,
            basis_digest,
            support_evidence_digest,
            runtime_origin_digest,
            posture_digest,
        }
    }

    pub fn kind(&self) -> ProjectionMaterializedFactPostureKind {
        self.kind
    }

    pub fn lower_declaration_digest(&self) -> &str {
        &self.lower_declaration_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn support_evidence_digest(&self) -> &str {
        &self.support_evidence_digest
    }

    pub fn runtime_origin_digest(&self) -> Option<&str> {
        self.runtime_origin_digest.as_deref()
    }

    pub fn posture_digest(&self) -> &str {
        &self.posture_digest
    }
}

impl ProjectionFactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EntityIdentity => "entity_identity",
            Self::ViewLocalIdentity => "view_local_identity",
            Self::TargetIdentity => "target_identity",
            Self::SourceReference => "source_reference",
            Self::EffectContinuity => "effect_continuity",
            Self::Membership => "membership",
            Self::RelationEndpoint => "relation_endpoint",
            Self::DisplayField => "display_field",
            Self::DerivedScalarField => "derived_scalar_field",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::EntityIdentity,
            Self::ViewLocalIdentity,
            Self::TargetIdentity,
            Self::SourceReference,
            Self::EffectContinuity,
            Self::Membership,
            Self::RelationEndpoint,
            Self::DisplayField,
            Self::DerivedScalarField,
        ]
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProjectMaterializedFacts {
    requested: BTreeSet<ProjectionFactRequest>,
}

impl ProjectMaterializedFacts {
    pub fn declare() -> Self {
        Self::default()
    }

    pub fn entity_identities(mut self) -> Self {
        self.requested.insert(ProjectionFactRequest::EntityIdentity);
        self
    }

    pub fn view_local_identities(mut self) -> Self {
        self.requested
            .insert(ProjectionFactRequest::ViewLocalIdentity);
        self
    }

    pub fn target_identity(mut self) -> Self {
        self.requested.insert(ProjectionFactRequest::TargetIdentity);
        self
    }

    pub fn source_references(mut self) -> Self {
        self.requested
            .insert(ProjectionFactRequest::SourceReference);
        self
    }

    pub fn effect_continuity_facts(mut self) -> Self {
        self.requested
            .insert(ProjectionFactRequest::EffectContinuity);
        self
    }

    pub fn memberships(mut self) -> Self {
        self.requested.insert(ProjectionFactRequest::Membership);
        self
    }

    pub fn relation_endpoints(mut self) -> Self {
        self.requested
            .insert(ProjectionFactRequest::RelationEndpoint);
        self
    }

    pub fn display_field(mut self, field: impl Into<String>) -> Self {
        self.requested
            .insert(ProjectionFactRequest::DisplayField(field.into()));
        self
    }

    pub fn derived_scalar_field(mut self, field: impl Into<String>) -> Self {
        self.requested
            .insert(ProjectionFactRequest::DerivedScalarField(field.into()));
        self
    }

    pub fn requested(&self) -> impl Iterator<Item = &ProjectionFactRequest> {
        self.requested.iter()
    }

    pub fn requested_count(&self) -> usize {
        self.requested.len()
    }
}
