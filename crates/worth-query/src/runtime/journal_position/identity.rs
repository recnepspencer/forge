use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::memory_workspace::WorthQueryCommitIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryJournalPositionAdmissionError {
    reason: &'static str,
}

impl WorthQueryJournalPositionAdmissionError {
    fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    #[allow(dead_code)]
    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryJournalPositionAuthority {
    Absent,
    Preview,
    Committed,
}

impl WorthQueryJournalPositionAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Preview => "preview",
            Self::Committed => "committed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryJournalPosition {
    authority: WorthQueryJournalPositionAuthority,
    ordinal: u64,
    evidence_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryJournalPosition {
    pub(in crate::runtime) fn try_from_commit_identity(
        commit_identity: &WorthQueryCommitIdentity,
    ) -> Result<Self, WorthQueryJournalPositionAdmissionError> {
        match commit_identity {
            WorthQueryCommitIdentity::RelationalBridge { bridge_identity } => {
                let Some(commit_id) = bridge_identity.relational_commit_id() else {
                    return Err(WorthQueryJournalPositionAdmissionError::new(
                        "committed journal position requires relational commit payload",
                    ));
                };
                Ok(Self::committed(commit_id))
            }
            WorthQueryCommitIdentity::Preview { evidence_identity } => {
                Ok(Self::preview(evidence_identity.clone(), 0))
            }
            WorthQueryCommitIdentity::Absent => Ok(Self::absent()),
        }
    }

    pub(in crate::runtime) fn from_commit_identity(
        commit_identity: &WorthQueryCommitIdentity,
    ) -> Self {
        Self::try_from_commit_identity(commit_identity)
            .expect("runtime receipts must carry admissible journal position authority")
    }

    pub(in crate::runtime) fn preview(
        preview_identity: WorthQueryEvidenceIdentity,
        sequence: u64,
    ) -> Self {
        Self::new(
            WorthQueryJournalPositionAuthority::Preview,
            sequence,
            preview_journal_position_identity(&preview_identity, sequence),
        )
    }

    fn committed(commit_id: u64) -> Self {
        Self::new(
            WorthQueryJournalPositionAuthority::Committed,
            commit_id,
            committed_journal_position_identity(commit_id),
        )
    }

    fn absent() -> Self {
        Self::new(
            WorthQueryJournalPositionAuthority::Absent,
            0,
            absent_journal_position_identity(),
        )
    }

    fn new(
        authority: WorthQueryJournalPositionAuthority,
        ordinal: u64,
        evidence_identity: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            authority,
            ordinal,
            evidence_identity,
        }
    }

    pub fn authority(&self) -> WorthQueryJournalPositionAuthority {
        self.authority
    }

    pub fn ordinal_for_reporting(&self) -> u64 {
        self.ordinal
    }

    pub fn evidence_identity(&self) -> WorthQueryEvidenceIdentity {
        self.evidence_identity.clone()
    }

    pub fn evidence_identity_ref(&self) -> &WorthQueryEvidenceIdentity {
        &self.evidence_identity
    }
}

fn committed_journal_position_identity(commit_id: u64) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::JournalPositionIdentity)
        .field_shape(
            WorthQueryEvidenceTag::new("journal_position_authority"),
            WorthQueryJournalPositionAuthority::Committed.as_str(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("journal_position_ordinal"),
            commit_id.to_string(),
        )
        .seal()
}

fn preview_journal_position_identity(
    preview_identity: &WorthQueryEvidenceIdentity,
    sequence: u64,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::JournalPositionIdentity)
        .field_shape(
            WorthQueryEvidenceTag::new("journal_position_authority"),
            WorthQueryJournalPositionAuthority::Preview.as_str(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("preview_write_receipt_identity"),
            preview_identity.as_str(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("journal_position_ordinal"),
            sequence.to_string(),
        )
        .seal()
}

fn absent_journal_position_identity() -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::JournalPositionIdentity)
        .field_shape(
            WorthQueryEvidenceTag::new("journal_position_authority"),
            WorthQueryJournalPositionAuthority::Absent.as_str(),
        )
        .field_value(WorthQueryEvidenceTag::new("journal_position_ordinal"), "0")
        .seal()
}
