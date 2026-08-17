//! Separate alpha-atlas owner and its qualified geometry.

use std::ops::{Deref, DerefMut};

use super::ownership::AtlasStore;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AlphaAtlasStore(AtlasStore);

impl AlphaAtlasStore {
    pub(crate) fn new() -> Self {
        Self(AtlasStore::new(1_024, 1_024, 4, 1))
    }
}

impl Deref for AlphaAtlasStore {
    type Target = AtlasStore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AlphaAtlasStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
