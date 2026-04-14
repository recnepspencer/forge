use super::BindingError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QueryBindingSubject {
    RootEntity,
    TraversalRoot,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QueryBindingSlot(String);

impl QueryBindingSlot {
    pub fn new(slot: impl Into<String>) -> Result<Self, BindingError> {
        let slot = slot.into();
        if slot.trim().is_empty() {
            return Err(BindingError::EmptyBindingSlot);
        }
        Ok(Self(slot))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
