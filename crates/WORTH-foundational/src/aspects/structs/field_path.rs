use super::FieldKey;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
}
