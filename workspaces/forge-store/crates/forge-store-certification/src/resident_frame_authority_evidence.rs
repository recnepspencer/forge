use forge_store_buffer_pool::{
    ResidentFrameCounterSnapshot, ResidentFrameDenial, ResidentFrameShortcutAttempt,
    ResidentFrameTable, ResidentGenerationSeparationProof,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameAuthorityEvidenceReport {
    row: ResidentFrameAuthorityEvidenceRow,
    counters: ResidentFrameCounterSnapshot,
}

impl ResidentFrameAuthorityEvidenceReport {
    pub fn from_table(
        row: ResidentFrameAuthorityEvidenceRow,
        table: &ResidentFrameTable,
    ) -> Result<Self, ResidentFrameAuthorityEvidenceDenial> {
        if !row.accepts_table() {
            return Err(ResidentFrameAuthorityEvidenceDenial::WrongEvidenceRow);
        }
        Ok(Self {
            row,
            counters: table.counters(),
        })
    }

    pub fn from_forbidden_denial(
        attempt: ResidentFrameShortcutAttempt,
        denial: ResidentFrameDenial,
    ) -> Result<Self, ResidentFrameAuthorityEvidenceDenial> {
        if denial.kind() != attempt.denial_kind() {
            return Err(ResidentFrameAuthorityEvidenceDenial::ForbiddenDenialMismatch);
        }
        Ok(Self {
            row: ResidentFrameAuthorityEvidenceRow::ForbiddenResidencyProofRejected(attempt),
            counters: ResidentFrameCounterSnapshot::empty(),
        })
    }

    pub const fn from_generation_separation(proof: ResidentGenerationSeparationProof) -> Self {
        Self {
            row: ResidentFrameAuthorityEvidenceRow::ResidentGenerationDomainSeparated,
            counters: proof.counters(),
        }
    }

    pub const fn row(self) -> ResidentFrameAuthorityEvidenceRow {
        self.row
    }

    pub const fn counters(self) -> ResidentFrameCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidentFrameAuthorityEvidenceRow {
    ResidentFrameTableAuthorityObserved,
    ResidentGenerationDomainSeparated,
    ExactResidentByteAccounting,
    ExactHitMissAccounting,
    ForbiddenResidencyProofRejected(ResidentFrameShortcutAttempt),
}

impl ResidentFrameAuthorityEvidenceRow {
    pub const fn s2_phase_two_table_rows() -> &'static [Self] {
        &[
            Self::ResidentFrameTableAuthorityObserved,
            Self::ExactResidentByteAccounting,
            Self::ExactHitMissAccounting,
        ]
    }

    const fn accepts_table(self) -> bool {
        matches!(
            self,
            Self::ResidentFrameTableAuthorityObserved
                | Self::ExactResidentByteAccounting
                | Self::ExactHitMissAccounting
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidentFrameAuthorityEvidenceDenial {
    WrongEvidenceRow,
    ForbiddenDenialMismatch,
}
