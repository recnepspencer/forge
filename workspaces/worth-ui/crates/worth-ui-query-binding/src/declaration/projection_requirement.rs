use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiProjectionFieldRequirementError {
    Empty,
    SurroundingWhitespace,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UiProjectionFieldRequirement {
    declared_name: Arc<str>,
}

impl UiProjectionFieldRequirement {
    pub fn declared(name: impl Into<String>) -> Result<Self, UiProjectionFieldRequirementError> {
        let name = name.into();
        if name.is_empty() {
            return Err(UiProjectionFieldRequirementError::Empty);
        }
        if name.trim() != name {
            return Err(UiProjectionFieldRequirementError::SurroundingWhitespace);
        }
        Ok(Self {
            declared_name: Arc::from(name),
        })
    }

    pub fn declared_name(&self) -> &str {
        self.declared_name.as_ref()
    }
}
