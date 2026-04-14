use super::AuthoringError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AspectFieldSelector {
    aspect: String,
    field: String,
}

impl AspectFieldSelector {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        let aspect = aspect.into();
        let field = field.into();
        if aspect.trim().is_empty() || field.trim().is_empty() {
            return Err(AuthoringError::EmptyProjectionSelector);
        }
        Ok(Self { aspect, field })
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }
}
