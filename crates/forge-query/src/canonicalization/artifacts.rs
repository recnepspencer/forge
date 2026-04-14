use crate::authoring::{
    OrderingDirection, PredicateSelector, QueryFamily, ResultShapeFamily, RootEntityKey,
    ScalarPredicateValue,
};
use crate::binding::IdentityBindingDescriptor;
use crate::identity::{CanonicalEquivalence, CanonicalQueryDigest, CanonicalResultShapeDigest};
use crate::result_shape::{canonical_result_field_digest_part, source_projection_key};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalProjectionEntry {
    pub(crate) aspect: String,
    pub(crate) field: String,
}

impl CanonicalProjectionEntry {
    pub(crate) fn digest_part(&self) -> String {
        format!("projection:{}:{}", self.aspect, self.field)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalTraversalEntry {
    pub(crate) relation: String,
    pub(crate) depth: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalPredicateEntry {
    pub(crate) aspect: String,
    pub(crate) field: String,
    pub(crate) family: CanonicalPredicateFamily,
    pub(crate) operand: CanonicalPredicateOperand,
}

impl CanonicalPredicateEntry {
    pub(crate) fn from_authored(predicate: &PredicateSelector) -> Self {
        match predicate {
            PredicateSelector::Equality(predicate) => Self {
                aspect: predicate.aspect().to_string(),
                field: predicate.field().to_string(),
                family: CanonicalPredicateFamily::Equality,
                operand: CanonicalPredicateOperand::Scalar(predicate.value().clone()),
            },
            PredicateSelector::IntegerComparison(predicate) => Self {
                aspect: predicate.aspect().to_string(),
                field: predicate.field().to_string(),
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
                aspect: predicate.aspect().to_string(),
                field: predicate.field().to_string(),
                family: CanonicalPredicateFamily::StringContains,
                operand: CanonicalPredicateOperand::Scalar(ScalarPredicateValue::String(
                    predicate.value().to_string(),
                )),
            },
            PredicateSelector::SetMembership(predicate) => {
                let mut values = predicate.values().to_vec();
                values.sort();
                values.dedup();
                Self {
                    aspect: predicate.aspect().to_string(),
                    field: predicate.field().to_string(),
                    family: CanonicalPredicateFamily::ScalarMembership,
                    operand: CanonicalPredicateOperand::ScalarSet(values),
                }
            }
            PredicateSelector::Presence(predicate) => Self {
                aspect: predicate.aspect().to_string(),
                field: predicate.field().to_string(),
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
    ScalarSet(Vec<ScalarPredicateValue>),
    Presence(&'static str),
}

impl CanonicalPredicateOperand {
    pub(crate) fn digest_part(&self) -> String {
        match self {
            Self::Scalar(value) => scalar_digest_part(value),
            Self::ScalarSet(values) => format!(
                "set:[{}]",
                values
                    .iter()
                    .map(scalar_digest_part)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
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

fn scalar_digest_part(value: &ScalarPredicateValue) -> String {
    match value {
        ScalarPredicateValue::String(value) => format!("string:{value}"),
        ScalarPredicateValue::Integer(value) => format!("integer:{value}"),
        ScalarPredicateValue::Boolean(value) => format!("boolean:{value}"),
    }
}

impl CanonicalTraversalEntry {
    pub(crate) fn digest_part(&self) -> String {
        format!("traversal:{}:{}", self.relation, self.depth)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalOrderingEntry {
    pub(crate) aspect: String,
    pub(crate) field: String,
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
    pub(crate) source_aspect: String,
    pub(crate) source_field: String,
    pub(crate) delivered_name: String,
}

impl CanonicalResultField {
    pub(crate) fn digest_part(&self) -> String {
        canonical_result_field_digest_part(
            &self.source_aspect,
            &self.source_field,
            &self.delivered_name,
        )
    }

    pub(crate) fn source_projection_key(&self) -> (String, String) {
        source_projection_key(&self.source_aspect, &self.source_field)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalQueryArtifact {
    pub(crate) digest: CanonicalQueryDigest,
    pub(crate) family: QueryFamily,
    pub(crate) root: RootEntityKey,
    pub(crate) projection: Vec<CanonicalProjectionEntry>,
    pub(crate) predicates: Vec<CanonicalPredicateEntry>,
    pub(crate) ordering: Vec<CanonicalOrderingEntry>,
    pub(crate) traversal: Vec<CanonicalTraversalEntry>,
    pub(crate) identity_bindings: Vec<IdentityBindingDescriptor>,
}

impl CanonicalQueryArtifact {
    pub fn digest(&self) -> &CanonicalQueryDigest {
        &self.digest
    }

    pub fn family(&self) -> &QueryFamily {
        &self.family
    }

    pub fn root(&self) -> &RootEntityKey {
        &self.root
    }

    pub fn projection(&self) -> &[CanonicalProjectionEntry] {
        &self.projection
    }

    pub fn predicates(&self) -> &[CanonicalPredicateEntry] {
        &self.predicates
    }

    pub fn ordering(&self) -> &[CanonicalOrderingEntry] {
        &self.ordering
    }

    pub fn traversal(&self) -> &[CanonicalTraversalEntry] {
        &self.traversal
    }

    pub fn identity_bindings(&self) -> &[IdentityBindingDescriptor] {
        &self.identity_bindings
    }

    pub fn equivalence_to(&self, other: &Self) -> CanonicalEquivalence {
        if self.family == other.family
            && self.root == other.root
            && self.projection == other.projection
            && self.predicates == other.predicates
            && self.ordering == other.ordering
            && self.traversal == other.traversal
            && self.identity_bindings == other.identity_bindings
            && self.digest == other.digest
        {
            CanonicalEquivalence::Equivalent
        } else {
            CanonicalEquivalence::Distinct
        }
    }

    #[cfg(test)]
    pub(crate) fn reverse_projection_for_test(&mut self) {
        self.projection.reverse();
    }

    #[cfg(test)]
    pub(crate) fn corrupt_digest_for_test(&mut self, marker: &str) {
        self.digest = CanonicalQueryDigest::from_parts(&[marker.to_string()]);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalResultShapeArtifact {
    pub(crate) digest: CanonicalResultShapeDigest,
    pub(crate) family: ResultShapeFamily,
    pub(crate) fields: Vec<CanonicalResultField>,
}

impl CanonicalResultShapeArtifact {
    pub fn digest(&self) -> &CanonicalResultShapeDigest {
        &self.digest
    }

    pub fn family(&self) -> &ResultShapeFamily {
        &self.family
    }

    pub fn fields(&self) -> &[CanonicalResultField] {
        &self.fields
    }

    pub fn equivalence_to(&self, other: &Self) -> CanonicalEquivalence {
        if self.family == other.family && self.fields == other.fields && self.digest == other.digest
        {
            CanonicalEquivalence::Equivalent
        } else {
            CanonicalEquivalence::Distinct
        }
    }

    #[cfg(test)]
    pub(crate) fn reverse_fields_for_test(&mut self) {
        self.fields.reverse();
    }

    #[cfg(test)]
    pub(crate) fn corrupt_digest_for_test(&mut self, marker: &str) {
        self.digest = CanonicalResultShapeDigest::from_parts(&[marker.to_string()]);
    }

    #[cfg(test)]
    pub(crate) fn rewrite_first_field_for_test(
        &mut self,
        source_aspect: impl Into<String>,
        source_field: impl Into<String>,
        delivered_name: impl Into<String>,
    ) {
        if let Some(field) = self.fields.first_mut() {
            field.source_aspect = source_aspect.into();
            field.source_field = source_field.into();
            field.delivered_name = delivered_name.into();
        }
    }
}
