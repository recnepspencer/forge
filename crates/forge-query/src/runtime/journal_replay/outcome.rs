use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryJournalPositionSchedule, ForgeQueryJournalSegmentIdentity, ForgeQueryWriteReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalReplayOutcome {
    segment_identity: ForgeQueryJournalSegmentIdentity,
    write_receipts: Vec<ForgeQueryWriteReceipt>,
    position_schedule: ForgeQueryJournalPositionSchedule,
    expected_journal_position_count: usize,
    resolved_journal_position_count: usize,
    journal_gap_count: usize,
    scanned_entry_count: usize,
    truth_reconstruction_identity: ForgeQueryEvidenceIdentity,
    published_artifact_digest: ForgeQueryEvidenceIdentity,
    outcome_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryJournalReplayOutcome {
    pub(in crate::runtime) fn new(
        segment_identity: ForgeQueryJournalSegmentIdentity,
        write_receipts: Vec<ForgeQueryWriteReceipt>,
        expected_journal_position_count: usize,
        journal_gap_count: usize,
        scanned_entry_count: usize,
        truth_reconstruction_identity: ForgeQueryEvidenceIdentity,
        published_artifact_digest: ForgeQueryEvidenceIdentity,
    ) -> Self {
        let position_schedule = ForgeQueryJournalPositionSchedule::derive(
            write_receipts
                .iter()
                .map(|receipt| receipt.journal_position().clone()),
        );
        let resolved_journal_position_count = write_receipts.len();
        let receipt_identities = write_receipts
            .iter()
            .map(|receipt| receipt.commit_evidence_identity().clone())
            .collect::<Vec<_>>();
        let outcome_identity =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::JournalReplayOutcome)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("journal_segment_identity"),
                    segment_identity.identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("journal_position_schedule"),
                    position_schedule.schedule_digest(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("published_artifact_digest"),
                    &published_artifact_digest,
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("replayed_write_commit_identity"),
                    receipt_identities.iter(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("truth_reconstruction_identity"),
                    &truth_reconstruction_identity,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("expected_journal_position_count"),
                    expected_journal_position_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("resolved_journal_position_count"),
                    resolved_journal_position_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("journal_gap_count"),
                    journal_gap_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("scanned_entry_count"),
                    scanned_entry_count,
                )
                .seal();
        Self {
            segment_identity,
            write_receipts,
            position_schedule,
            expected_journal_position_count,
            resolved_journal_position_count,
            journal_gap_count,
            scanned_entry_count,
            truth_reconstruction_identity,
            published_artifact_digest,
            outcome_identity,
        }
    }

    pub fn segment_identity(&self) -> &ForgeQueryJournalSegmentIdentity {
        &self.segment_identity
    }

    pub fn write_receipts(&self) -> &[ForgeQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn position_schedule(&self) -> &ForgeQueryJournalPositionSchedule {
        &self.position_schedule
    }

    pub fn journal_gap_count(&self) -> usize {
        self.journal_gap_count
    }

    pub fn expected_journal_position_count(&self) -> usize {
        self.expected_journal_position_count
    }

    pub fn resolved_journal_position_count(&self) -> usize {
        self.resolved_journal_position_count
    }

    pub fn scanned_entry_count(&self) -> usize {
        self.scanned_entry_count
    }

    pub fn truth_reconstruction_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.truth_reconstruction_identity
    }

    pub fn published_artifact_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.published_artifact_digest
    }

    pub fn outcome_digest(&self) -> &str {
        self.outcome_identity.as_str()
    }

    pub fn outcome_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.outcome_identity
    }
}

pub(crate) fn journal_replay_truth_reconstruction_identity(
    committed_truth_identities: &[ForgeQueryEvidenceIdentity],
    expected_journal_position_count: usize,
    journal_gap_count: usize,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::JournalReplayOutcome)
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("committed_truth_identity"),
            committed_truth_identities.iter(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("expected_journal_position_count"),
            expected_journal_position_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("journal_gap_count"),
            journal_gap_count,
        )
        .seal()
}
