use std::{
    num::NonZeroU64,
    path::{Path, PathBuf},
};

/// A caller-declared physical namespace.
///
/// C.3 rejects relative and dot-segment paths, then compares the remaining
/// declaration exactly. It does not resolve symlinks, platform case aliases,
/// or cross-process namespace identity; C.4 owns those filesystem facts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredStoreRoot(PathBuf);

impl DeclaredStoreRoot {
    pub(crate) fn from_validated_path(path: PathBuf) -> Self {
        Self(path)
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

/// Identity of one admitted in-process runtime incarnation.
///
/// This value is observable identity, not runtime authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeIdentity(NonZeroU64);

impl RuntimeIdentity {
    pub(crate) fn generate() -> Option<Self> {
        loop {
            let mut bytes = [0_u8; 8];
            getrandom::fill(&mut bytes).ok()?;
            if let Some(identity) = NonZeroU64::new(u64::from_le_bytes(bytes)) {
                return Some(Self(identity));
            }
        }
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }

    pub(in crate::physical_runtime) const fn from_reopened(identity: NonZeroU64) -> Self {
        Self(identity)
    }
}
