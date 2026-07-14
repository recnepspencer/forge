use worth_store_buffer_pool::DirtyPageIdentity;
use worth_store_physical_format::PageGenerationCell;

use super::PageLsn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollbackImagePublicationPosture {
    NotRequiredForAdmittedRedoOnlyMutation,
    RollbackImageProtectsUnadmittedBytes,
    RollbackImageRequiredButMissing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackImagePublicationDeclaration {
    dirty_identity: DirtyPageIdentity,
    page_generation: PageGenerationCell,
    page_lsn: PageLsn,
    declaration_digest: String,
}

impl RollbackImagePublicationDeclaration {
    pub fn declare(
        dirty_identity: DirtyPageIdentity,
        page_generation: PageGenerationCell,
        page_lsn: PageLsn,
        declaration_digest: impl Into<String>,
    ) -> Self {
        Self {
            dirty_identity,
            page_generation,
            page_lsn,
            declaration_digest: declaration_digest.into(),
        }
    }

    pub(crate) const fn dirty_identity(&self) -> DirtyPageIdentity {
        self.dirty_identity
    }

    pub const fn page_generation(&self) -> PageGenerationCell {
        self.page_generation
    }

    pub const fn page_lsn(&self) -> PageLsn {
        self.page_lsn
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }
}
