use crate::aspects::{AspectKey, CanonicalFieldPath};

use super::super::LocatorAuthority;
use super::AspectLocator;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AspectFieldLocator {
    aspect: AspectLocator,
    field_path: CanonicalFieldPath,
}

impl AspectFieldLocator {
    pub fn new(
        authority: LocatorAuthority,
        aspect_key: AspectKey,
        field_path: CanonicalFieldPath,
    ) -> Self {
        Self::from_aspect(AspectLocator::new(authority, aspect_key), field_path)
    }

    pub fn from_aspect(aspect: AspectLocator, field_path: CanonicalFieldPath) -> Self {
        Self { aspect, field_path }
    }

    pub fn aspect(&self) -> &AspectLocator {
        &self.aspect
    }

    pub fn field_path(&self) -> &CanonicalFieldPath {
        &self.field_path
    }

    pub fn owned_allocation_capacity_bytes(&self) -> usize {
        self.aspect
            .aspect_key()
            .owned_allocation_capacity_bytes()
            .saturating_add(self.field_path.owned_allocation_capacity_bytes())
    }
}
