use crate::{LogSequenceNumber, PageLsn};
use forge_store_physical_format::{PageGenerationCell, PhysicalPageId};

use super::RedoRecordMaterializedForm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedoRecordGrammar {
    target_page: PhysicalPageId,
    target_generation: RedoRecordTargetGeneration,
    redo_lsn: LogSequenceNumber,
    operation_form: RedoRecordOperationForm,
    integrity_binding: RedoRecordIntegrityBinding,
    idempotence_basis: RedoRecordIdempotenceBasis,
    page_lsn_basis: PageLsn,
}

impl RedoRecordGrammar {
    pub fn from_materialized_record(
        record: RedoRecordMaterializedForm,
    ) -> Result<Self, RedoRecordGrammarDenial> {
        Self::admit(
            Some(record.target_page),
            Some(record.target_generation),
            Some(record.redo_lsn),
            Some(record.operation_form),
            Some(record.integrity_binding),
            Some(record.idempotence_basis),
            Some(record.page_lsn_basis),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn admit(
        target_page: Option<PhysicalPageId>,
        target_generation: Option<RedoRecordTargetGeneration>,
        redo_lsn: Option<LogSequenceNumber>,
        operation_form: Option<RedoRecordOperationForm>,
        integrity_binding: Option<RedoRecordIntegrityBinding>,
        idempotence_basis: Option<RedoRecordIdempotenceBasis>,
        page_lsn_basis: Option<PageLsn>,
    ) -> Result<Self, RedoRecordGrammarDenial> {
        Ok(Self {
            target_page: target_page.ok_or(RedoRecordGrammarDenial::new(
                RedoRecordGrammarDenialKind::MissingTargetPage,
            ))?,
            target_generation: target_generation.ok_or(RedoRecordGrammarDenial::new(
                RedoRecordGrammarDenialKind::MissingTargetGeneration,
            ))?,
            redo_lsn: redo_lsn.ok_or(RedoRecordGrammarDenial::new(
                RedoRecordGrammarDenialKind::MissingRedoLsn,
            ))?,
            operation_form: operation_form.ok_or(RedoRecordGrammarDenial::new(
                RedoRecordGrammarDenialKind::MissingOperationForm,
            ))?,
            integrity_binding: integrity_binding.ok_or(RedoRecordGrammarDenial::new(
                RedoRecordGrammarDenialKind::MissingIntegrityBinding,
            ))?,
            idempotence_basis: idempotence_basis.ok_or(RedoRecordGrammarDenial::new(
                RedoRecordGrammarDenialKind::MissingIdempotenceBasis,
            ))?,
            page_lsn_basis: page_lsn_basis.ok_or(RedoRecordGrammarDenial::new(
                RedoRecordGrammarDenialKind::MissingPageLsnBasis,
            ))?,
        })
    }

    pub const fn target_page(&self) -> PhysicalPageId {
        self.target_page
    }

    pub const fn target_generation(&self) -> RedoRecordTargetGeneration {
        self.target_generation
    }

    pub const fn redo_lsn(&self) -> LogSequenceNumber {
        self.redo_lsn
    }

    pub const fn page_lsn_basis(&self) -> PageLsn {
        self.page_lsn_basis
    }

    pub fn operation_form(&self) -> &RedoRecordOperationForm {
        &self.operation_form
    }

    pub fn integrity_binding(&self) -> &RedoRecordIntegrityBinding {
        &self.integrity_binding
    }

    pub fn idempotence_basis(&self) -> &RedoRecordIdempotenceBasis {
        &self.idempotence_basis
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedoRecordTargetGeneration {
    generation: PageGenerationCell,
}

impl RedoRecordTargetGeneration {
    pub const fn new(generation: PageGenerationCell) -> Self {
        Self { generation }
    }

    pub const fn generation(self) -> PageGenerationCell {
        self.generation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedoRecordOperationForm {
    digest: String,
}

impl RedoRecordOperationForm {
    pub fn declared_digest(digest: impl Into<String>) -> Self {
        Self {
            digest: digest.into(),
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedoRecordIntegrityBinding {
    digest: String,
}

impl RedoRecordIntegrityBinding {
    pub fn declared_digest(digest: impl Into<String>) -> Self {
        Self {
            digest: digest.into(),
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedoRecordIdempotenceBasis {
    digest: String,
}

impl RedoRecordIdempotenceBasis {
    pub fn declared_digest(digest: impl Into<String>) -> Self {
        Self {
            digest: digest.into(),
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedoRecordGrammarDenial {
    kind: RedoRecordGrammarDenialKind,
}

impl RedoRecordGrammarDenial {
    pub(crate) const fn new(kind: RedoRecordGrammarDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> RedoRecordGrammarDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedoRecordGrammarDenialKind {
    MissingTargetPage,
    MissingTargetGeneration,
    MissingRedoLsn,
    MissingOperationForm,
    MissingIntegrityBinding,
    MissingIdempotenceBasis,
    MissingPageLsnBasis,
}
