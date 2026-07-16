use crate::authoring::{DeliveredFieldName, RelationName};
use crate::canonicalization::{
    CanonicalOrderingEntry, CanonicalPredicateEntry, CanonicalPredicateFamily,
    CanonicalPredicateOperand, CanonicalProjectionEntry, CanonicalResultField,
    CanonicalTraversalEntry,
};
use crate::schema_view::ScalarAspectType;
use worth_foundational::facade::{AspectKey, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedProjectionEntry {
    aspect_key: AspectKey,
    field_key: FieldKey,
    field_kind: ScalarAspectType,
}

impl ValidatedProjectionEntry {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field_key
    }

    pub fn field_kind(&self) -> &ScalarAspectType {
        &self.field_kind
    }

    pub(crate) fn from_canonical(
        entry: &CanonicalProjectionEntry,
        field_kind: ScalarAspectType,
    ) -> Self {
        Self {
            aspect_key: entry.field_key().native_aspect_key(),
            field_key: entry.field_key().native_field_key(),
            field_kind,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "validated-projection:{}:{}:{:?}",
            self.aspect_key.as_str(),
            self.field_key.as_str(),
            self.field_kind
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedTraversalEntry {
    relation: RelationName,
    depth: u8,
    max_depth: u8,
}

impl ValidatedTraversalEntry {
    pub fn relation_name(&self) -> &RelationName {
        &self.relation
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }

    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }

    pub(crate) fn from_canonical(entry: &CanonicalTraversalEntry, max_depth: u8) -> Self {
        Self {
            relation: entry.relation.clone(),
            depth: entry.depth,
            max_depth,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "validated-traversal:{}:{}:{}",
            self.relation, self.depth, self.max_depth
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedResultShapeBinding {
    source_aspect_key: AspectKey,
    source_field_key: FieldKey,
    delivered_name: DeliveredFieldName,
    field_kind: ScalarAspectType,
}

impl ValidatedResultShapeBinding {
    pub fn native_source_aspect_key(&self) -> &AspectKey {
        &self.source_aspect_key
    }

    pub fn native_source_field_key(&self) -> &FieldKey {
        &self.source_field_key
    }

    pub fn delivered_name(&self) -> &str {
        self.delivered_name.as_str()
    }

    pub fn field_kind(&self) -> &ScalarAspectType {
        &self.field_kind
    }

    pub(crate) fn from_canonical(
        field: &CanonicalResultField,
        field_kind: ScalarAspectType,
    ) -> Self {
        Self {
            source_aspect_key: field.source_field_key().native_aspect_key(),
            source_field_key: field.source_field_key().native_field_key(),
            delivered_name: field.delivered_name.clone(),
            field_kind,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "validated-result-binding:{}:{}:{}:{:?}",
            self.source_aspect_key.as_str(),
            self.source_field_key.as_str(),
            self.delivered_name,
            self.field_kind
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedPredicateEntry {
    aspect_key: AspectKey,
    field_key: FieldKey,
    predicate_family: &'static str,
    field_kind: ScalarAspectType,
    value_kind: &'static str,
    value_basis: String,
}

impl ValidatedPredicateEntry {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field_key
    }

    pub fn predicate_family(&self) -> &'static str {
        self.predicate_family
    }

    pub fn field_kind(&self) -> &ScalarAspectType {
        &self.field_kind
    }

    pub fn value_kind(&self) -> &'static str {
        self.value_kind
    }

    pub fn value_basis(&self) -> &str {
        &self.value_basis
    }

    pub(crate) fn from_canonical(
        entry: &CanonicalPredicateEntry,
        field_kind: ScalarAspectType,
        value_kind: &'static str,
    ) -> Self {
        Self {
            aspect_key: entry.field_key().native_aspect_key(),
            field_key: entry.field_key().native_field_key(),
            predicate_family: match entry.family {
                CanonicalPredicateFamily::Equality => "equality",
                CanonicalPredicateFamily::NativeGreaterThan => "native-greater-than",
                CanonicalPredicateFamily::NativeLessThan => "native-less-than",
                CanonicalPredicateFamily::StringContains => "string-contains",
                CanonicalPredicateFamily::ScalarMembership => "scalar-membership",
                CanonicalPredicateFamily::PresenceIsPresent => "presence-is-present",
            },
            field_kind,
            value_kind,
            value_basis: canonical_operand_basis(&entry.operand),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "validated-predicate:{}:{}:{}:{:?}:{}:{}",
            self.aspect_key.as_str(),
            self.field_key.as_str(),
            self.predicate_family,
            self.field_kind,
            self.value_kind,
            self.value_basis
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValidatedOrderingEntry {
    aspect_key: AspectKey,
    field_key: FieldKey,
    direction: &'static str,
    field_kind: ScalarAspectType,
    projected: bool,
}

impl ValidatedOrderingEntry {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field_key
    }

    pub fn direction(&self) -> &'static str {
        self.direction
    }

    pub fn field_kind(&self) -> &ScalarAspectType {
        &self.field_kind
    }

    pub fn projected(&self) -> bool {
        self.projected
    }

    pub(crate) fn from_canonical(
        entry: &CanonicalOrderingEntry,
        field_kind: ScalarAspectType,
        projected: bool,
    ) -> Self {
        Self {
            aspect_key: entry.field_key().native_aspect_key(),
            field_key: entry.field_key().native_field_key(),
            direction: match entry.direction {
                crate::authoring::OrderingDirection::Ascending => "ascending",
                crate::authoring::OrderingDirection::Descending => "descending",
            },
            field_kind,
            projected,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "validated-ordering:{}:{}:{}:{:?}:{}",
            self.aspect_key.as_str(),
            self.field_key.as_str(),
            self.direction,
            self.field_kind,
            self.projected
        )
    }
}

fn canonical_operand_basis(operand: &CanonicalPredicateOperand) -> String {
    operand.digest_part()
}
