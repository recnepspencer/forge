use super::{AspectFieldKey, AspectName, AuthoringError, FieldName};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderingDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OrderingSelector {
    key: AspectFieldKey,
    direction: OrderingDirection,
}

impl OrderingSelector {
    pub fn ascending(
        aspect: impl Into<String>,
        field: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        Self::new(aspect, field, OrderingDirection::Ascending)
    }

    pub fn descending(
        aspect: impl Into<String>,
        field: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        Self::new(aspect, field, OrderingDirection::Descending)
    }

    pub fn ascending_source_field_key(key: AspectFieldKey) -> Self {
        Self::from_source_field_key(key, OrderingDirection::Ascending)
    }

    pub fn descending_source_field_key(key: AspectFieldKey) -> Self {
        Self::from_source_field_key(key, OrderingDirection::Descending)
    }

    fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        direction: OrderingDirection,
    ) -> Result<Self, AuthoringError> {
        Ok(Self {
            key: AspectFieldKey::from_authoring_parts(aspect, field)
                .map_err(|_| AuthoringError::EmptyOrderingSelector)?,
            direction,
        })
    }

    fn from_source_field_key(key: AspectFieldKey, direction: OrderingDirection) -> Self {
        Self { key, direction }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.key
    }

    pub fn aspect_name(&self) -> &AspectName {
        self.key.aspect()
    }

    pub fn field_name(&self) -> &FieldName {
        self.key.field()
    }

    pub fn direction(&self) -> OrderingDirection {
        self.direction
    }
}
