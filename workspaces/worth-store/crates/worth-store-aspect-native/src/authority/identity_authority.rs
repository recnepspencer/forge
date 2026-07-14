use worth_foundational::AspectKey;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreAspectIdentity {
    aspect_key: AspectKey,
}

impl StoreAspectIdentity {
    pub const fn from_aspect_key(aspect_key: AspectKey) -> Self {
        Self { aspect_key }
    }

    pub const fn aspect_key(&self) -> &AspectKey {
        &self.aspect_key
    }
}
