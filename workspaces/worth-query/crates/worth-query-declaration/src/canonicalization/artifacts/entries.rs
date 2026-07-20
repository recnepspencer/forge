use crate::authoring::{
    AspectFieldKey, DeliveredFieldName, OrderingDirection, PredicateSelector, RelationName,
    WorthQueryPredicateOperand,
};
use crate::result_shape::canonical_result_field_digest_part;

use super::scalar_set::CanonicalScalarSet;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalProjectionEntry {
    pub field: AspectFieldKey,
}

impl CanonicalProjectionEntry {
    pub fn field_key(&self) -> &AspectFieldKey {
        &self.field
    }

    pub fn digest_part(&self) -> String {
        format!("projection:{}:{}", self.field.aspect(), self.field.field())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalTraversalEntry {
    pub relation: RelationName,
    pub depth: u8,
}

impl CanonicalTraversalEntry {
    pub fn digest_part(&self) -> String {
        format!("traversal:{}:{}", self.relation, self.depth)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalPredicateEntry {
    pub field: AspectFieldKey,
    pub family: CanonicalPredicateFamily,
    pub operand: CanonicalPredicateOperand,
}

impl CanonicalPredicateEntry {
    pub fn from_authored(predicate: &PredicateSelector) -> Self {
        match predicate {
            PredicateSelector::Equality(predicate) => Self {
                field: predicate.target_field_key().clone(),
                family: CanonicalPredicateFamily::Equality,
                operand: CanonicalPredicateOperand::Scalar(predicate.value().clone()),
            },
            PredicateSelector::NativeComparison(predicate) => Self {
                field: predicate.target_field_key().clone(),
                family: match predicate.operator() {
                    crate::authoring::NativeComparisonOperator::GreaterThan => {
                        CanonicalPredicateFamily::NativeGreaterThan
                    }
                    crate::authoring::NativeComparisonOperator::LessThan => {
                        CanonicalPredicateFamily::NativeLessThan
                    }
                },
                operand: CanonicalPredicateOperand::Scalar(predicate.value().clone()),
            },
            PredicateSelector::StringContains(predicate) => Self {
                field: predicate.target_field_key().clone(),
                family: CanonicalPredicateFamily::StringContains,
                operand: CanonicalPredicateOperand::Scalar(WorthQueryPredicateOperand::string(
                    predicate.value().to_string(),
                )),
            },
            PredicateSelector::SetMembership(predicate) => Self {
                field: predicate.target_field_key().clone(),
                family: CanonicalPredicateFamily::ScalarMembership,
                operand: CanonicalPredicateOperand::ScalarSet(CanonicalScalarSet::new(
                    predicate.values().iter().cloned(),
                )),
            },
            PredicateSelector::Presence(predicate) => Self {
                field: predicate.target_field_key().clone(),
                family: CanonicalPredicateFamily::PresenceIsPresent,
                operand: CanonicalPredicateOperand::Presence(predicate.kind().digest_key()),
            },
        }
    }

    pub fn field_key(&self) -> &AspectFieldKey {
        &self.field
    }

    pub fn digest_part(&self) -> String {
        format!(
            "predicate:{}:{}:{}:{}",
            self.family.digest_key(),
            self.field.aspect(),
            self.field.field(),
            self.operand.digest_part()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CanonicalPredicateOperand {
    Scalar(WorthQueryPredicateOperand),
    ScalarSet(CanonicalScalarSet),
    Presence(&'static str),
}

impl CanonicalPredicateOperand {
    pub fn digest_part(&self) -> String {
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
    NativeGreaterThan,
    NativeLessThan,
    StringContains,
    ScalarMembership,
    PresenceIsPresent,
}

impl CanonicalPredicateFamily {
    pub fn digest_key(self) -> &'static str {
        match self {
            Self::Equality => "eq",
            Self::NativeGreaterThan => "gt-native",
            Self::NativeLessThan => "lt-native",
            Self::StringContains => "contains-str",
            Self::ScalarMembership => "in-set",
            Self::PresenceIsPresent => "is-present",
        }
    }
}

pub fn scalar_digest_part(value: &WorthQueryPredicateOperand) -> String {
    worth_foundational::facade::prepare_aspect_value_identity_basis(value.as_native())
        .as_str()
        .to_owned()
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalOrderingEntry {
    pub field: AspectFieldKey,
    pub direction: OrderingDirection,
}

impl CanonicalOrderingEntry {
    pub fn field_key(&self) -> &AspectFieldKey {
        &self.field
    }

    pub fn digest_part(&self) -> String {
        format!(
            "ordering:{}:{}:{:?}",
            self.field.aspect(),
            self.field.field(),
            self.direction
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalResultField {
    pub source: AspectFieldKey,
    pub delivered_name: DeliveredFieldName,
}

impl CanonicalResultField {
    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn digest_part(&self) -> String {
        canonical_result_field_digest_part(
            self.source.aspect(),
            self.source.field(),
            &self.delivered_name,
        )
    }

    pub fn source_projection_key(&self) -> AspectFieldKey {
        self.source.clone()
    }
}
