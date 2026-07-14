use worth_store_buffer_pool::{
    BoundedCopyRecordView, RecordCopyCounterSnapshot, RecordViewDenial, ResidentFrameTable,
    ZeroCopyRecordView,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordViewEvidenceReport {
    row: RecordViewEvidenceRow,
    counters: RecordCopyCounterSnapshot,
}

impl RecordViewEvidenceReport {
    pub fn from_zero_copy_view(
        row: RecordViewEvidenceRow,
        view: &ZeroCopyRecordView<'_>,
    ) -> Result<Self, RecordViewEvidenceDenial> {
        let counters = view.counters();
        match row {
            RecordViewEvidenceRow::ZeroCopyLeaseScopedPhysicalBytes
                if counters.zero_copy_admission_count() > 0
                    && !view.proves_semantic_domain_object()
                    && !view.physical_record_bytes().is_empty() =>
            {
                Ok(Self { row, counters })
            }
            RecordViewEvidenceRow::ZeroCopyLeaseScopedPhysicalBytes => {
                Err(RecordViewEvidenceDenial::UnprovenRecordViewRow)
            }
            _ => Err(RecordViewEvidenceDenial::WrongRow),
        }
    }

    pub fn from_bounded_copy_view(
        row: RecordViewEvidenceRow,
        view: &BoundedCopyRecordView,
    ) -> Result<Self, RecordViewEvidenceDenial> {
        let counters = view.counters();
        match row {
            RecordViewEvidenceRow::BoundedCopyRequiresAllocationAndExactCounters
                if counters.bounded_copy_count() > 0
                    && counters.copied_bytes() as usize == view.physical_record_bytes().len()
                    && !view.proves_semantic_domain_object() =>
            {
                Ok(Self { row, counters })
            }
            RecordViewEvidenceRow::BoundedCopyRequiresAllocationAndExactCounters => {
                Err(RecordViewEvidenceDenial::UnprovenRecordViewRow)
            }
            _ => Err(RecordViewEvidenceDenial::WrongRow),
        }
    }

    pub fn from_view_denial(
        row: RecordViewEvidenceRow,
        denial: RecordViewDenial,
    ) -> Result<Self, RecordViewEvidenceDenial> {
        let counters = denial.counters();
        match row {
            RecordViewEvidenceRow::InvalidInputsDenyBeforeConstruction
                if counters.denied_before_view_construction_count() > 0 =>
            {
                Ok(Self { row, counters })
            }
            RecordViewEvidenceRow::InvalidInputsDenyBeforeConstruction => {
                Err(RecordViewEvidenceDenial::UnprovenRecordViewRow)
            }
            _ => Err(RecordViewEvidenceDenial::WrongRow),
        }
    }

    pub fn from_table(
        row: RecordViewEvidenceRow,
        table: &ResidentFrameTable,
    ) -> Result<Self, RecordViewEvidenceDenial> {
        let counters = table.record_view_counters();
        match row {
            RecordViewEvidenceRow::ViewMutationConflictDeniedBeforeDirtyMutation
                if counters.dirty_mutation_conflict_denial_count() > 0 =>
            {
                Ok(Self { row, counters })
            }
            RecordViewEvidenceRow::ViewPublicationConflictDeniedBeforeScheduling
                if counters.publication_conflict_denial_count() > 0 =>
            {
                Ok(Self { row, counters })
            }
            RecordViewEvidenceRow::ViewMutationConflictDeniedBeforeDirtyMutation => {
                Err(RecordViewEvidenceDenial::UnprovenRecordViewRow)
            }
            RecordViewEvidenceRow::ViewPublicationConflictDeniedBeforeScheduling => {
                Err(RecordViewEvidenceDenial::UnprovenRecordViewRow)
            }
            _ => Err(RecordViewEvidenceDenial::WrongRow),
        }
    }

    pub const fn row(self) -> RecordViewEvidenceRow {
        self.row
    }

    pub const fn counters(self) -> RecordCopyCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordViewEvidenceRow {
    ZeroCopyLeaseScopedPhysicalBytes,
    BoundedCopyRequiresAllocationAndExactCounters,
    InvalidInputsDenyBeforeConstruction,
    ViewMutationConflictDeniedBeforeDirtyMutation,
    ViewPublicationConflictDeniedBeforeScheduling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordViewEvidenceDenial {
    WrongRow,
    UnprovenRecordViewRow,
}
