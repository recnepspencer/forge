use crate::authoring::{
    AspectFieldKey, AspectName, DeliveredFieldName, FieldName, OrderingDirection,
    PredicateSelector, RelationName, ScalarPredicateValue,
};
use crate::result_shape::{canonical_result_field_digest_part, source_projection_key};

use super::scalar_set::CanonicalScalarSet;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalProjectionEntry {
    pub(crate) aspect: AspectName,
    pub(crate) field: FieldName,
}

impl CanonicalProjectionEntry {
    pub(crate) fn digest_part(&self) -> String {
        format!("projection:{}:{}", self.aspect, self.field)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalTraversalEntry {
    pub(crate) relation: RelationName,
    pub(crate) depth: u8,
}

impl CanonicalTraversalEntry {
    pub(crate) fn digest_part(&self) -> String {
        format!("traversal:{}:{}", self.relation, self.depth)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalPredicateEntry {
    pub(crate) aspect: AspectName,
    pub(crate) field: FieldName,
    pub(crate) family: CanonicalPredicateFamily,
    pub(crate) operand: CanonicalPredicateOperand,
}

impl CanonicalPredicateEntry {
    pub(crate) fn from_authored(predicate: &PredicateSelector) -> Self {
        match predicate {
            PredicateSelector::Equality(predicate) => Self {
                aspect: predicate.aspect_name().clone(),
                field: predicate.field_name().clone(),
                family: CanonicalPredicateFamily::Equality,
                operand: CanonicalPredicateOperand::Scalar(predicate.value().clone()),
            },
            PredicateSelector::IntegerComparison(predicate) => Self {
                aspect: predicate.aspect_name().clone(),
                field: predicate.field_name().clone(),
                family: match predicate.operator() {
                    crate::authoring::IntegerComparisonOperator::GreaterThan => {
                        CanonicalPredicateFamily::IntegerGreaterThan
                    }
                    crate::authoring::IntegerComparisonOperator::LessThan => {
                        CanonicalPredicateFamily::IntegerLessThan
                    }
                },
                operand: CanonicalPredicateOperand::Scalar(ScalarPredicateValue::Integer(
                    predicate.value(),
                )),
            },
            PredicateSelector::StringContains(predicate) => Self {
                aspect: predicate.aspect_name().clone(),
                field: predicate.field_name().clone(),
                family: CanonicalPredicateFamily::StringContains,
                operand: CanonicalPredicateOperand::Scalar(ScalarPredicateValue::String(
                    predicate.value().to_string(),
                )),
            },
            PredicateSelector::SetMembership(predicate) => Self {
                aspect: predicate.aspect_name().clone(),
                field: predicate.field_name().clone(),
                family: CanonicalPredicateFamily::ScalarMembership,
                operand: CanonicalPredicateOperand::ScalarSet(CanonicalScalarSet::new(
                    predicate.values().iter().cloned(),
                )),
            },
            PredicateSelector::Presence(predicate) => Self {
                aspect: predicate.aspect_name().clone(),
                field: predicate.field_name().clone(),
                family: CanonicalPredicateFamily::PresenceIsPresent,
                operand: CanonicalPredicateOperand::Presence(predicate.kind().digest_key()),
            },
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "predicate:{}:{}:{}:{}",
            self.family.digest_key(),
            self.aspect,
            self.field,
            self.operand.digest_part()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CanonicalPredicateOperand {
    Scalar(ScalarPredicateValue),
    ScalarSet(CanonicalScalarSet),
    Presence(&'static str),
}

impl CanonicalPredicateOperand {
    pub(crate) fn digest_part(&self) -> String {
        match self {
            Self::Scalar(value) => scalar_digest_part(value),
            Self::ScalarSet(values) => values.digest_part(),
            Self::Presence(kind) => format!("presence:{kind}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CanonicalPredicateFamily {
    Equality,
    IntegerGreaterThan,
    IntegerLessThan,
    StringContains,
    ScalarMembership,
    PresenceIsPresent,
}

impl CanonicalPredicateFamily {
    pub(crate) fn digest_key(self) -> &'static str {
        match self {
            Self::Equality => "eq",
            Self::IntegerGreaterThan => "gt-int",
            Self::IntegerLessThan => "lt-int",
            Self::StringContains => "contains-str",
            Self::ScalarMembership => "in-set",
            Self::PresenceIsPresent => "is-present",
        }
    }
}

pub(crate) fn scalar_digest_part(value: &ScalarPredicateValue) -> String {
    match value {
        ScalarPredicateValue::String(value) => format!("string:{value}"),
        ScalarPredicateValue::Integer(value) => format!("integer:{value}"),
        ScalarPredicateValue::Boolean(value) => format!("boolean:{value}"),
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalOrderingEntry {
    pub(crate) aspect: AspectName,
    pub(crate) field: FieldName,
    pub(crate) direction: OrderingDirection,
}

impl CanonicalOrderingEntry {
    pub(crate) fn digest_part(&self) -> String {
        format!(
            "ordering:{}:{}:{:?}",
            self.aspect, self.field, self.direction
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalResultField {
    pub(crate) source_aspect: AspectName,
    pub(crate) source_field: FieldName,
    pub(crate) delivered_name: DeliveredFieldName,
}

impl CanonicalResultField {
    pub(crate) fn digest_part(&self) -> String {
        canonical_result_field_digest_part(
            &self.source_aspect,
            &self.source_field,
            &self.delivered_name,
        )
    }

    pub(crate) fn source_projection_key(&self) -> AspectFieldKey {
        source_projection_key(&self.source_aspect, &self.source_field)
    }
}
