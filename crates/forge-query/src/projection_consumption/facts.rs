use std::collections::BTreeSet;

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
