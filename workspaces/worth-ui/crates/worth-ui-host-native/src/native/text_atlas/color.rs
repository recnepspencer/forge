//! Separate color-atlas owner and its qualified geometry.

use std::ops::{Deref, DerefMut};

use super::ownership::AtlasStore;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ColorAtlasStore(AtlasStore);

impl ColorAtlasStore {
    pub(crate) fn new() -> Self {
        Self(AtlasStore::new(2_048, 2_048, 2, 4))
    }
}

impl Deref for ColorAtlasStore {
    type Target = AtlasStore;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ColorAtlasStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
