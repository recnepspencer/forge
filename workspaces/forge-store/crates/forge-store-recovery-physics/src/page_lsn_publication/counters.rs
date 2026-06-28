#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageLsnPublicationCounterSnapshot {
    dirty_publication_evidence_count: u64,
    wal_before_data_proof_count: u64,
    wal_before_data_denial_count: u64,
    no_undo_eligibility_count: u64,
    no_undo_denial_count: u64,
    page_flush_receipt_count: u64,
    missing_page_lsn_classification_count: u64,
    stale_page_redo_required_count: u64,
    current_page_redo_skip_count: u64,
    generation_mismatch_denial_count: u64,
    redo_basis_mismatch_denial_count: u64,
    redo_current_page_lsn_mismatch_denial_count: u64,
    idempotent_redo_application_count: u64,
}

impl PageLsnPublicationCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            dirty_publication_evidence_count: 0,
            wal_before_data_proof_count: 0,
            wal_before_data_denial_count: 0,
            no_undo_eligibility_count: 0,
            no_undo_denial_count: 0,
            page_flush_receipt_count: 0,
            missing_page_lsn_classification_count: 0,
            stale_page_redo_required_count: 0,
            current_page_redo_skip_count: 0,
            generation_mismatch_denial_count: 0,
            redo_basis_mismatch_denial_count: 0,
            redo_current_page_lsn_mismatch_denial_count: 0,
            idempotent_redo_application_count: 0,
        }
    }

    pub(crate) const fn with_dirty_publication_evidence(mut self) -> Self {
        self.dirty_publication_evidence_count += 1;
        self
    }

    pub(crate) const fn with_wal_before_data_proof(mut self) -> Self {
        self.wal_before_data_proof_count += 1;
        self
    }

    pub(crate) const fn with_wal_before_data_denial(mut self) -> Self {
        self.wal_before_data_denial_count += 1;
        self
    }

    pub(crate) const fn with_no_undo_eligibility(mut self) -> Self {
        self.no_undo_eligibility_count += 1;
        self
    }

    pub(crate) const fn with_no_undo_denial(mut self) -> Self {
        self.no_undo_denial_count += 1;
        self
    }

    pub(crate) const fn with_page_flush_receipt(mut self) -> Self {
        self.page_flush_receipt_count += 1;
        self
    }

    pub(crate) const fn with_missing_page_lsn_classification(mut self) -> Self {
        self.missing_page_lsn_classification_count += 1;
        self
    }

    pub(crate) const fn with_stale_page_redo_required(mut self) -> Self {
        self.stale_page_redo_required_count += 1;
        self
    }

    pub(crate) const fn with_current_page_redo_skip(mut self) -> Self {
        self.current_page_redo_skip_count += 1;
        self
    }

    pub(crate) const fn with_generation_mismatch_denial(mut self) -> Self {
        self.generation_mismatch_denial_count += 1;
        self
    }

    pub(crate) const fn with_redo_basis_mismatch_denial(mut self) -> Self {
        self.redo_basis_mismatch_denial_count += 1;
        self
    }

    pub(crate) const fn with_redo_current_page_lsn_mismatch_denial(mut self) -> Self {
        self.redo_current_page_lsn_mismatch_denial_count += 1;
        self
    }

    pub(crate) const fn with_idempotent_redo_application(mut self) -> Self {
        self.idempotent_redo_application_count += 1;
        self
    }

    pub const fn dirty_publication_evidence_count(self) -> u64 {
        self.dirty_publication_evidence_count
    }

    pub const fn wal_before_data_proof_count(self) -> u64 {
        self.wal_before_data_proof_count
    }

    pub const fn wal_before_data_denial_count(self) -> u64 {
        self.wal_before_data_denial_count
    }

    pub const fn no_undo_eligibility_count(self) -> u64 {
        self.no_undo_eligibility_count
    }

    pub const fn no_undo_denial_count(self) -> u64 {
        self.no_undo_denial_count
    }

    pub const fn page_flush_receipt_count(self) -> u64 {
        self.page_flush_receipt_count
    }

    pub const fn missing_page_lsn_classification_count(self) -> u64 {
        self.missing_page_lsn_classification_count
    }

    pub const fn stale_page_redo_required_count(self) -> u64 {
        self.stale_page_redo_required_count
    }

    pub const fn current_page_redo_skip_count(self) -> u64 {
        self.current_page_redo_skip_count
    }

    pub const fn generation_mismatch_denial_count(self) -> u64 {
        self.generation_mismatch_denial_count
    }

    pub const fn redo_basis_mismatch_denial_count(self) -> u64 {
        self.redo_basis_mismatch_denial_count
    }

    pub const fn redo_current_page_lsn_mismatch_denial_count(self) -> u64 {
        self.redo_current_page_lsn_mismatch_denial_count
    }

    pub const fn idempotent_redo_application_count(self) -> u64 {
        self.idempotent_redo_application_count
    }
}
