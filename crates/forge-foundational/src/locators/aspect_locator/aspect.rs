use crate::aspects::AspectKey;

use super::super::LocatorAuthority;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AspectLocator {
    authority: LocatorAuthority,
    aspect_key: AspectKey,
}

impl AspectLocator {
    pub fn new(authority: LocatorAuthority, aspect_key: AspectKey) -> Self {
        Self {
            authority,
            aspect_key,
        }
    }

    pub const fn authority(&self) -> LocatorAuthority {
        self.authority
    }

    pub fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }
}
