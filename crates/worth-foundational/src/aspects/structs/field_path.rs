use super::FieldKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CanonicalFieldPath(Vec<FieldKey>);

impl CanonicalFieldPath {
    pub fn new(fields: impl IntoIterator<Item = FieldKey>) -> Option<Self> {
        let fields: Vec<_> = fields.into_iter().collect();
        if fields.is_empty() {
            None
        } else {
            Some(Self(fields))
        }
    }

    pub fn single(field: FieldKey) -> Self {
        Self(vec![field])
    }

    pub fn fields(&self) -> &[FieldKey] {
        &self.0
    }

    pub fn owned_allocation_capacity_bytes(&self) -> usize {
        self.0
            .capacity()
            .saturating_mul(std::mem::size_of::<FieldKey>())
            .saturating_add(
                self.0
                    .iter()
                    .map(FieldKey::owned_allocation_capacity_bytes)
                    .sum(),
            )
    }
}
