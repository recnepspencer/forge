use serde::{Deserialize, Serialize};

use crate::data::transaction::CommitAuthority;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitAuthorityContract {
    pub authority: CommitAuthority,
    pub version_publication_serialized: bool,
    pub lineage_finalization_serialized: bool,
    pub patch_publication_serialized: bool,
}

impl Default for CommitAuthorityContract {
    fn default() -> Self {
        Self {
            authority: CommitAuthority::default(),
            version_publication_serialized: true,
            lineage_finalization_serialized: true,
            patch_publication_serialized: true,
        }
    }
}
