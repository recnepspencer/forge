use crate::authoring::ScalarPredicateValue;

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

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "set:[{}]",
            self.0
                .iter()
                .map(super::entries::scalar_digest_part)
                .collect::<Vec<_>>()
                .join(",")
        )
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
