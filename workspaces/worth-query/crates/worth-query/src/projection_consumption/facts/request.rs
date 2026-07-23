use super::ProjectionFactFieldPath;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProjectionFactRequest {
    EntityIdentity,
    ViewLocalIdentity,
    TargetIdentity,
    SourceReference,
    EffectContinuity,
    Membership,
    RelationEndpoint,
    DisplayField(ProjectionFactFieldPath),
    DerivedField(ProjectionFactFieldPath),
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
            Self::DerivedField(_) => ProjectionFactKind::DerivedField,
        }
    }

    pub fn field_path(&self) -> Option<&ProjectionFactFieldPath> {
        match self {
            Self::DisplayField(field) | Self::DerivedField(field) => Some(field),
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
    DerivedField,
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
            Self::DerivedField => "derived_field",
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
            Self::DerivedField,
        ]
    }
}
