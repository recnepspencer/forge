use crate::{CompletedResidencyBoundaryReceipt, RecordViewEvidenceReport, RecordViewEvidenceRow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectedIntegrityViewEvidence {
    protected_view_count: u32,
    resident_bytes: u64,
    pinned_pages: u64,
}

impl ProtectedIntegrityViewEvidence {
    pub fn from_zero_copy_report(
        report: RecordViewEvidenceReport,
        receipt: &CompletedResidencyBoundaryReceipt,
        WORTHd_view_access_denied: bool,
    ) -> Result<Self, ProtectedIntegrityViewEvidenceDenial> {
        let resident = receipt.resident_memory().counters();
        let copy = receipt.copy_materialization().counters();
        let pin_lifecycle = resident.pin_lifecycle();
        if report.row() != RecordViewEvidenceRow::ZeroCopyLeaseScopedPhysicalBytes {
            return Err(ProtectedIntegrityViewEvidenceDenial::WrongRecordViewRow);
        }
        if report.counters().zero_copy_admission_count() != copy.zero_copy_admission_count() {
            return Err(ProtectedIntegrityViewEvidenceDenial::RecordViewCounterMismatch);
        }
        if !WORTHd_view_access_denied {
            return Err(ProtectedIntegrityViewEvidenceDenial::WORTHdViewDenialMissing);
        }
        if resident.resident_bytes().as_bytes() == 0
            || pin_lifecycle.successful_pin_count() == 0
            || copy.zero_copy_admission_count() == 0
        {
            return Err(ProtectedIntegrityViewEvidenceDenial::MissingProtectedViewBasis);
        }
        Ok(Self {
            protected_view_count: copy.zero_copy_admission_count() as u32,
            resident_bytes: resident.resident_bytes().as_bytes(),
            pinned_pages: pin_lifecycle.successful_pin_count(),
        })
    }

    pub const fn protected_view_count(self) -> u32 {
        self.protected_view_count
    }

    pub const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub const fn pinned_pages(self) -> u64 {
        self.pinned_pages
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectedIntegrityViewEvidenceDenial {
    WrongRecordViewRow,
    RecordViewCounterMismatch,
    WORTHdViewDenialMissing,
    MissingProtectedViewBasis,
}
