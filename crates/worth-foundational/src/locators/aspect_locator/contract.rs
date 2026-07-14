use crate::aspects::AspectKey;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AspectContractLocator {
    aspect_key: AspectKey,
}

impl AspectContractLocator {
    pub fn new(aspect_key: AspectKey) -> Self {
        Self { aspect_key }
    }

    pub fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }
}
