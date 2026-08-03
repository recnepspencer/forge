use worth_store_physical_format::PageGenerationCell;

use super::PageLsn;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRedoApplicationBasis {
    target_generation: PageGenerationCell,
    redo_lsn: PageLsn,
    operation_digest: String,
    idempotence_basis_digest: String,
}

impl PageRedoApplicationBasis {
    pub fn new(
        target_generation: PageGenerationCell,
        redo_lsn: PageLsn,
        operation_digest: impl Into<String>,
        idempotence_basis_digest: impl Into<String>,
    ) -> Self {
        Self {
            target_generation,
            redo_lsn,
            operation_digest: operation_digest.into(),
            idempotence_basis_digest: idempotence_basis_digest.into(),
        }
    }

    pub const fn target_generation(&self) -> PageGenerationCell {
        self.target_generation
    }

    pub const fn redo_lsn(&self) -> PageLsn {
        self.redo_lsn
    }

    pub fn operation_digest(&self) -> &str {
        &self.operation_digest
    }

    pub fn idempotence_basis_digest(&self) -> &str {
        &self.idempotence_basis_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageRedoDigestState {
    page_generation: PageGenerationCell,
    page_lsn: PageLsn,
    physical_state_digest: String,
}

impl PageRedoDigestState {
    pub fn new(
        page_generation: PageGenerationCell,
        page_lsn: PageLsn,
        physical_state_digest: impl Into<String>,
    ) -> Self {
        Self {
            page_generation,
            page_lsn,
            physical_state_digest: physical_state_digest.into(),
        }
    }

    pub(super) fn after_redo(self, basis: &PageRedoApplicationBasis) -> Self {
        Self {
            page_generation: self.page_generation,
            page_lsn: basis.redo_lsn(),
            physical_state_digest: format!(
                "{}:{}",
                basis.operation_digest(),
                basis.idempotence_basis_digest()
            ),
        }
    }

    pub const fn page_generation(&self) -> PageGenerationCell {
        self.page_generation
    }

    pub const fn page_lsn(&self) -> PageLsn {
        self.page_lsn
    }

    pub fn physical_state_digest(&self) -> &str {
        &self.physical_state_digest
    }
}
