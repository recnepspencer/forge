use super::admitted_field_kind::ForgeQueryGraphReadAdmittedSchemaFieldKind;
use crate::authoring::RelationName;
use forge_foundational::facade::{AspectKey, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryAdmittedGraphReadRelationDirection {
    Forward,
    Ancestor,
    Descendant,
}

impl ForgeQueryAdmittedGraphReadRelationDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Forward => "forward",
            Self::Ancestor => "ancestor",
            Self::Descendant => "descendant",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedGraphReadRelation {
    relation: RelationName,
    direction: ForgeQueryAdmittedGraphReadRelationDirection,
    depth: usize,
}

impl ForgeQueryAdmittedGraphReadRelation {
    pub fn relation_name(&self) -> &RelationName {
        &self.relation
    }

    pub(crate) fn terminal_relation_projection_for_boundary(&self) -> &str {
        self.relation.as_str()
    }

    pub fn direction(&self) -> &ForgeQueryAdmittedGraphReadRelationDirection {
        &self.direction
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub(crate) fn new(
        relation: RelationName,
        direction: ForgeQueryAdmittedGraphReadRelationDirection,
        depth: usize,
    ) -> Self {
        Self {
            relation: relation.into(),
            direction,
            depth,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "relation:{}:{}:{}",
            self.relation.as_str(),
            self.direction.as_str(),
            self.depth
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedGraphReadProjectionField {
    aspect: AspectKey,
    field: FieldKey,
    delivered_name: String,
    kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
}

impl ForgeQueryAdmittedGraphReadProjectionField {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field
    }

    pub fn delivered_name(&self) -> &str {
        &self.delivered_name
    }

    pub fn kind(&self) -> &ForgeQueryGraphReadAdmittedSchemaFieldKind {
        &self.kind
    }

    pub(crate) fn new(
        aspect: AspectKey,
        field: FieldKey,
        delivered_name: impl Into<String>,
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
    ) -> Self {
        Self {
            aspect,
            field,
            delivered_name: delivered_name.into(),
            kind,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "projection:{}:{}:{}:{}",
            self.aspect.as_str(),
            self.field.as_str(),
            self.delivered_name,
            self.kind.as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedGraphReadPredicateField {
    aspect: AspectKey,
    field: FieldKey,
    family: String,
    kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
}

impl ForgeQueryAdmittedGraphReadPredicateField {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn kind(&self) -> &ForgeQueryGraphReadAdmittedSchemaFieldKind {
        &self.kind
    }

    pub(crate) fn new(
        aspect: AspectKey,
        field: FieldKey,
        family: impl Into<String>,
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
    ) -> Self {
        Self {
            aspect,
            field,
            family: family.into(),
            kind,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "predicate:{}:{}:{}:{}",
            self.aspect.as_str(),
            self.field.as_str(),
            self.family,
            self.kind.as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedGraphReadOrderingField {
    aspect: AspectKey,
    field: FieldKey,
    direction: String,
    kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
}

impl ForgeQueryAdmittedGraphReadOrderingField {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field
    }

    pub fn direction(&self) -> &str {
        &self.direction
    }

    pub fn kind(&self) -> &ForgeQueryGraphReadAdmittedSchemaFieldKind {
        &self.kind
    }

    pub(crate) fn new(
        aspect: AspectKey,
        field: FieldKey,
        direction: impl Into<String>,
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
    ) -> Self {
        Self {
            aspect,
            field,
            direction: direction.into(),
            kind,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "ordering:{}:{}:{}:{}",
            self.aspect.as_str(),
            self.field.as_str(),
            self.direction,
            self.kind.as_str()
        )
    }
}
