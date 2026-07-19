use worth_store_buffer_pool::BufferPoolEntryDenialKind;
use worth_store_readiness::PhysicalSubstrateReadiness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2EntryBoundaryEvidenceReport {
    row: S2EntryBoundaryEvidenceRow,
}

impl S2EntryBoundaryEvidenceReport {
    pub fn from_readiness(
        row: S2EntryBoundaryEvidenceRow,
        readiness: &PhysicalSubstrateReadiness,
    ) -> Result<Self, S2EntryBoundaryEvidenceDenial> {
        if !readiness.is_sealed() {
            return Err(S2EntryBoundaryEvidenceDenial::UnsealedReadiness);
        }
        if !row.accepts_readiness() {
            return Err(S2EntryBoundaryEvidenceDenial::WrongEvidenceRow);
        }
        Ok(Self { row })
    }

    pub const fn from_forbidden_attempt(attempt: S2ForbiddenEntryAttempt) -> Self {
        Self {
            row: S2EntryBoundaryEvidenceRow::ForbiddenEntryAttemptRejected(attempt),
        }
    }

    pub const fn row(&self) -> S2EntryBoundaryEvidenceRow {
        self.row
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S2EntryBoundaryEvidenceRow {
    ReadinessConsumed,
    HandoffFactsAvailable,
    ResidencyVocabularyEquivalent,
    ForbiddenEntryAttemptRejected(S2ForbiddenEntryAttempt),
}

impl S2EntryBoundaryEvidenceRow {
    pub const fn physical_substrate_readiness_rows() -> &'static [Self] {
        &[
            Self::ReadinessConsumed,
            Self::HandoffFactsAvailable,
            Self::ResidencyVocabularyEquivalent,
        ]
    }

    const fn accepts_readiness(&self) -> bool {
        matches!(
            self,
            Self::ReadinessConsumed
                | Self::HandoffFactsAvailable
                | Self::ResidencyVocabularyEquivalent
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S2ForbiddenEntryAttempt {
    RawPageId,
    RawPayloadView,
    CompatibilityBackendHandle,
    FoundationalProfileAsAuthority,
}

impl S2ForbiddenEntryAttempt {
    pub const fn physical_substrate_forbidden_attempts() -> &'static [Self] {
        &[
            Self::RawPageId,
            Self::RawPayloadView,
            Self::CompatibilityBackendHandle,
            Self::FoundationalProfileAsAuthority,
        ]
    }

    pub const fn buffer_pool_denial_kind(self) -> BufferPoolEntryDenialKind {
        match self {
            Self::RawPageId => BufferPoolEntryDenialKind::RawPageIdRejected,
            Self::RawPayloadView => BufferPoolEntryDenialKind::RawPayloadViewRejected,
            Self::CompatibilityBackendHandle => {
                BufferPoolEntryDenialKind::CompatibilityBackendHandleRejected
            }
            Self::FoundationalProfileAsAuthority => {
                BufferPoolEntryDenialKind::FoundationalEvidenceAsAuthorityRejected
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S2EntryBoundaryEvidenceDenial {
    UnsealedReadiness,
    WrongEvidenceRow,
}

#[cfg(test)]
mod tests {
    use crate::{
        S2EntryBoundaryEvidenceReport, S2EntryBoundaryEvidenceRow, S2ForbiddenEntryAttempt,
    };
    use worth_store_buffer_pool::BufferPoolEntryDenialKind;

    #[test]
    fn physical_substrate_entry_boundary_reports_every_forbidden_shortcut_with_buffer_pool_denial()
    {
        let expected_denials = [
            BufferPoolEntryDenialKind::RawPageIdRejected,
            BufferPoolEntryDenialKind::RawPayloadViewRejected,
            BufferPoolEntryDenialKind::CompatibilityBackendHandleRejected,
            BufferPoolEntryDenialKind::FoundationalEvidenceAsAuthorityRejected,
        ];

        for (attempt, denial) in S2ForbiddenEntryAttempt::physical_substrate_forbidden_attempts()
            .iter()
            .zip(expected_denials)
        {
            let report = S2EntryBoundaryEvidenceReport::from_forbidden_attempt(*attempt);

            assert_eq!(
                report.row(),
                S2EntryBoundaryEvidenceRow::ForbiddenEntryAttemptRejected(*attempt)
            );
            assert_eq!(attempt.buffer_pool_denial_kind(), denial);
        }
    }
}
