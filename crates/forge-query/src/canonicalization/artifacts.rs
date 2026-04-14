use crate::authoring::{
    AspectFieldKey, AspectName, DeliveredFieldName, FieldName, OrderingDirection,
    PredicateSelector, QueryFamily, RelationName, ResultShapeFamily, RootEntityKey,
    ScalarPredicateValue,
};
use crate::binding::IdentityBindingDescriptor;
use crate::identity::{CanonicalEquivalence, CanonicalQueryDigest, CanonicalResultShapeDigest};
use crate::result_shape::{canonical_result_field_digest_part, source_projection_key};

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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalScalarSet(Vec<ScalarPredicateValue>);

impl CanonicalScalarSet {
    pub(crate) fn new(values: impl IntoIterator<Item = ScalarPredicateValue>) -> Self {
        let mut values: Vec<_> = values.into_iter().collect();
        values.sort();
        values.dedup();
        Self(values)
    }

    pub(crate) fn as_slice(&self) -> &[ScalarPredicateValue] {
        &self.0
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn first(&self) -> Option<&ScalarPredicateValue> {
        self.0.first()
    }

    pub(crate) fn contains(&self, value: &ScalarPredicateValue) -> bool {
        self.0.binary_search(value).is_ok()
    }

    pub(crate) fn intersect(&self, other: &Self) -> Self {
        let mut intersection = Vec::with_capacity(self.len().min(other.len()));
        let mut left_index = 0;
        let mut right_index = 0;
        let left = self.as_slice();
        let right = other.as_slice();

        while left_index < left.len() && right_index < right.len() {
            match left[left_index].cmp(&right[right_index]) {
                std::cmp::Ordering::Less => left_index += 1,
                std::cmp::Ordering::Greater => right_index += 1,
                std::cmp::Ordering::Equal => {
                    intersection.push(left[left_index].clone());
                    left_index += 1;
                    right_index += 1;
                }
            }
        }

        Self(intersection)
    }

    pub(crate) fn filtered(&self, mut keep: impl FnMut(&ScalarPredicateValue) -> bool) -> Self {
        let mut reduced = Vec::with_capacity(self.len());
        reduced.extend(self.0.iter().filter(|value| keep(value)).cloned());
        Self(reduced)
    }

    fn digest_part(&self) -> String {
        format!(
            "set:[{}]",
            self.0
                .iter()
                .map(scalar_digest_part)
                .collect::<Vec<_>>()
                .join(",")
        )
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
            field.source_aspect = AspectName::new(source_aspect)
                .expect("test rewrite must keep non-empty source aspect");
            field.source_field = FieldName::new(source_field)
                .expect("test rewrite must keep non-empty source field");
            field.delivered_name = DeliveredFieldName::new(delivered_name)
                .expect("test rewrite must keep non-empty delivered name");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CanonicalScalarSet;
    use crate::authoring::ScalarPredicateValue;

    #[test]
    fn canonical_scalar_set_normalizes_order_and_duplicates() {
        let set = CanonicalScalarSet::new([
            ScalarPredicateValue::Integer(3),
            ScalarPredicateValue::Integer(1),
            ScalarPredicateValue::Integer(3),
            ScalarPredicateValue::Integer(2),
        ]);

        assert_eq!(
            set.as_slice(),
            &[
                ScalarPredicateValue::Integer(1),
                ScalarPredicateValue::Integer(2),
                ScalarPredicateValue::Integer(3),
            ]
        );
    }
}
