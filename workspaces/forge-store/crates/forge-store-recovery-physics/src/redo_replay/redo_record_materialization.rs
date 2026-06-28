use crate::{LogSequenceNumber, PageLsn};
use forge_store_physical_format::PhysicalPageId;

use super::{
    RedoRecordIdempotenceBasis, RedoRecordIntegrityBinding, RedoRecordOperationForm,
    RedoRecordTargetGeneration,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedoRecordMaterializedForm {
    pub(crate) target_page: PhysicalPageId,
    pub(crate) target_generation: RedoRecordTargetGeneration,
    pub(crate) redo_lsn: LogSequenceNumber,
    pub(crate) operation_form: RedoRecordOperationForm,
    pub(crate) integrity_binding: RedoRecordIntegrityBinding,
    pub(crate) idempotence_basis: RedoRecordIdempotenceBasis,
    pub(crate) page_lsn_basis: PageLsn,
}

impl RedoRecordMaterializedForm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        target_page: PhysicalPageId,
        target_generation: RedoRecordTargetGeneration,
        redo_lsn: LogSequenceNumber,
        operation_form: RedoRecordOperationForm,
        integrity_binding: RedoRecordIntegrityBinding,
        idempotence_basis: RedoRecordIdempotenceBasis,
        page_lsn_basis: PageLsn,
    ) -> Self {
        Self {
            target_page,
            target_generation,
            redo_lsn,
            operation_form,
            integrity_binding,
            idempotence_basis,
            page_lsn_basis,
        }
    }
}
