use crate::authoring::WorthQueryPredicateOperand;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalScalarSet(Vec<WorthQueryPredicateOperand>);

impl CanonicalScalarSet {
    pub fn new(values: impl IntoIterator<Item = WorthQueryPredicateOperand>) -> Self {
        let mut values: Vec<_> = values.into_iter().collect();
        values.sort();
        values.dedup();
        Self(values)
    }

    pub fn as_slice(&self) -> &[WorthQueryPredicateOperand] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first(&self) -> Option<&WorthQueryPredicateOperand> {
        self.0.first()
    }

    pub fn contains(&self, value: &WorthQueryPredicateOperand) -> bool {
        self.0.binary_search(value).is_ok()
    }

    pub fn intersect(&self, other: &Self) -> Self {
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

    pub fn filtered(&self, mut keep: impl FnMut(&WorthQueryPredicateOperand) -> bool) -> Self {
        let mut reduced = Vec::with_capacity(self.len());
        reduced.extend(self.0.iter().filter(|value| keep(value)).cloned());
        Self(reduced)
    }

    pub fn digest_part(&self) -> String {
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
    use crate::authoring::WorthQueryPredicateOperand;

    #[test]
    fn canonical_scalar_set_normalizes_order_and_duplicates() {
        let set = CanonicalScalarSet::new([
            WorthQueryPredicateOperand::int64(3),
            WorthQueryPredicateOperand::int64(1),
            WorthQueryPredicateOperand::int64(3),
            WorthQueryPredicateOperand::int64(2),
        ]);

        assert_eq!(
            set.as_slice(),
            &[
                WorthQueryPredicateOperand::int64(1),
                WorthQueryPredicateOperand::int64(2),
                WorthQueryPredicateOperand::int64(3),
            ]
        );
    }
}
