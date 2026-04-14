use super::AuthoringError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OrderingDirection {
    Ascending,
    Descending,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OrderingSelector {
    aspect: String,
    field: String,
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

    fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        direction: OrderingDirection,
    ) -> Result<Self, AuthoringError> {
        let aspect = aspect.into();
        let field = field.into();
        if aspect.trim().is_empty() || field.trim().is_empty() {
            return Err(AuthoringError::EmptyOrderingSelector);
        }
        Ok(Self {
            aspect,
            field,
            direction,
        })
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn direction(&self) -> OrderingDirection {
        self.direction
    }
}
