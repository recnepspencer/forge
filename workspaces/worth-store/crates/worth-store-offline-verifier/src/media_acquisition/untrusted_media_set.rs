use std::path::{Path, PathBuf};

use worth_store_physical_backend::OfflineMediaConsistencyBasis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntrustedOfflineMediaSet {
    root: PathBuf,
    consistency_basis: OfflineMediaConsistencyBasis,
}

impl UntrustedOfflineMediaSet {
    pub fn from_root(
        root: impl Into<PathBuf>,
        consistency_basis: OfflineMediaConsistencyBasis,
    ) -> Self {
        Self {
            root: root.into(),
            consistency_basis,
        }
    }
    pub fn root(&self) -> &Path {
        &self.root
    }
    pub const fn consistency_basis(&self) -> &OfflineMediaConsistencyBasis {
        &self.consistency_basis
    }

    pub(crate) fn into_parts(self) -> (PathBuf, OfflineMediaConsistencyBasis) {
        (self.root, self.consistency_basis)
    }
}
