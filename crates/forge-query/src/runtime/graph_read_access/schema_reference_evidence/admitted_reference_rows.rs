use super::admitted_field_kind::ForgeQueryGraphReadAdmittedSchemaFieldKind;

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
    relation: String,
    direction: ForgeQueryAdmittedGraphReadRelationDirection,
    depth: usize,
}

impl ForgeQueryAdmittedGraphReadRelation {
    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn direction(&self) -> &ForgeQueryAdmittedGraphReadRelationDirection {
        &self.direction
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub(crate) fn new(
        relation: impl Into<String>,
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
            self.relation,
            self.direction.as_str(),
            self.depth
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedGraphReadProjectionField {
    aspect: String,
    field: String,
    delivered_name: String,
    kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
}

impl ForgeQueryAdmittedGraphReadProjectionField {
    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn delivered_name(&self) -> &str {
        &self.delivered_name
    }

    pub fn kind(&self) -> &ForgeQueryGraphReadAdmittedSchemaFieldKind {
        &self.kind
    }

    pub(crate) fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        delivered_name: impl Into<String>,
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
    ) -> Self {
        Self {
            aspect: aspect.into(),
            field: field.into(),
            delivered_name: delivered_name.into(),
            kind,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "projection:{}:{}:{}:{}",
            self.aspect,
            self.field,
            self.delivered_name,
            self.kind.as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedGraphReadPredicateField {
    aspect: String,
    field: String,
    family: String,
    kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
}

impl ForgeQueryAdmittedGraphReadPredicateField {
    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn kind(&self) -> &ForgeQueryGraphReadAdmittedSchemaFieldKind {
        &self.kind
    }

    pub(crate) fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        family: impl Into<String>,
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
    ) -> Self {
        Self {
            aspect: aspect.into(),
            field: field.into(),
            family: family.into(),
            kind,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "predicate:{}:{}:{}:{}",
            self.aspect,
            self.field,
            self.family,
            self.kind.as_str()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedGraphReadOrderingField {
    aspect: String,
    field: String,
    direction: String,
    kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
}

impl ForgeQueryAdmittedGraphReadOrderingField {
    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn direction(&self) -> &str {
        &self.direction
    }

    pub fn kind(&self) -> &ForgeQueryGraphReadAdmittedSchemaFieldKind {
        &self.kind
    }

    pub(crate) fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        direction: impl Into<String>,
        kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
    ) -> Self {
        Self {
            aspect: aspect.into(),
            field: field.into(),
            direction: direction.into(),
            kind,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "ordering:{}:{}:{}:{}",
            self.aspect,
            self.field,
            self.direction,
            self.kind.as_str()
        )
    }
}
