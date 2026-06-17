use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryCommitIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalPositionAdmissionError {
    reason: &'static str,
}

impl ForgeQueryJournalPositionAdmissionError {
    fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    #[allow(dead_code)]
    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ForgeQueryJournalPositionAuthority {
    Absent,
    Preview,
    Committed,
}

impl ForgeQueryJournalPositionAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Preview => "preview",
            Self::Committed => "committed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalPosition {
    authority: ForgeQueryJournalPositionAuthority,
    ordinal: u64,
    evidence_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryJournalPosition {
    pub(in crate::runtime) fn try_from_commit_identity(
        commit_identity: &ForgeQueryCommitIdentity,
    ) -> Result<Self, ForgeQueryJournalPositionAdmissionError> {
        match commit_identity {
            ForgeQueryCommitIdentity::RelationalBridge { bridge_identity } => {
                let Some(commit_id) = bridge_identity.relational_commit_id() else {
                    return Err(ForgeQueryJournalPositionAdmissionError::new(
                        "committed journal position requires relational commit payload",
                    ));
                };
                Ok(Self::committed(commit_id))
            }
            ForgeQueryCommitIdentity::Preview { evidence_identity } => {
                Ok(Self::preview(evidence_identity.clone(), 0))
            }
            ForgeQueryCommitIdentity::Absent => Ok(Self::absent()),
        }
    }

    pub(in crate::runtime) fn from_commit_identity(
        commit_identity: &ForgeQueryCommitIdentity,
    ) -> Self {
        Self::try_from_commit_identity(commit_identity)
            .expect("runtime receipts must carry admissible journal position authority")
    }

    pub(in crate::runtime) fn preview(
        preview_identity: ForgeQueryEvidenceIdentity,
        sequence: u64,
    ) -> Self {
        Self::new(
            ForgeQueryJournalPositionAuthority::Preview,
            sequence,
            preview_journal_position_identity(&preview_identity, sequence),
        )
    }

    fn committed(commit_id: u64) -> Self {
        Self::new(
            ForgeQueryJournalPositionAuthority::Committed,
            commit_id,
            committed_journal_position_identity(commit_id),
        )
    }

    fn absent() -> Self {
        Self::new(
            ForgeQueryJournalPositionAuthority::Absent,
            0,
            absent_journal_position_identity(),
        )
    }

    fn new(
        authority: ForgeQueryJournalPositionAuthority,
        ordinal: u64,
        evidence_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            authority,
            ordinal,
            evidence_identity,
        }
    }

    pub fn authority(&self) -> ForgeQueryJournalPositionAuthority {
        self.authority
    }

    pub fn ordinal_for_reporting(&self) -> u64 {
        self.ordinal
    }

    pub fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        self.evidence_identity.clone()
    }

    pub fn evidence_identity_ref(&self) -> &ForgeQueryEvidenceIdentity {
        &self.evidence_identity
    }
}

fn committed_journal_position_identity(commit_id: u64) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::JournalPositionIdentity)
        .field_shape(
            ForgeQueryEvidenceTag::new("journal_position_authority"),
            ForgeQueryJournalPositionAuthority::Committed.as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("journal_position_ordinal"),
            commit_id.to_string(),
        )
        .seal()
}

fn preview_journal_position_identity(
    preview_identity: &ForgeQueryEvidenceIdentity,
    sequence: u64,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::JournalPositionIdentity)
        .field_shape(
            ForgeQueryEvidenceTag::new("journal_position_authority"),
            ForgeQueryJournalPositionAuthority::Preview.as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("preview_write_receipt_identity"),
            preview_identity.as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("journal_position_ordinal"),
            sequence.to_string(),
        )
        .seal()
}

fn absent_journal_position_identity() -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::JournalPositionIdentity)
        .field_shape(
            ForgeQueryEvidenceTag::new("journal_position_authority"),
            ForgeQueryJournalPositionAuthority::Absent.as_str(),
        )
        .field_value(ForgeQueryEvidenceTag::new("journal_position_ordinal"), "0")
        .seal()
}
