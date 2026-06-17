use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceIdentityScheme, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::{
    ForgeQueryJournalReplayDenial, ForgeQueryJournalReplayDenialKind,
    ForgeQueryJournalReplayDiagnostics, ForgeQueryJournalReplayOutcome,
    ForgeQueryJournalReplayRequest, ForgeQueryWriteReceipt,
};

use super::{journal_replay_truth_reconstruction_identity, ForgeQueryJournalReplayCounters};

#[derive(Clone, Debug, Default)]
pub(in crate::runtime) struct ForgeQueryJournalReplayRegistry {
    entries: Vec<ForgeQueryJournalReplayEntry>,
    counters: ForgeQueryJournalReplayCounters,
}

impl ForgeQueryJournalReplayRegistry {
    pub fn record_write_receipt(&mut self, receipt: &ForgeQueryWriteReceipt) {
        self.entries
            .push(ForgeQueryJournalReplayEntry::from_write_receipt(receipt));
    }

    pub fn record_batch_receipt(&mut self, receipt: &crate::runtime::ForgeQueryBatchWriteReceipt) {
        self.entries.extend(
            receipt
                .write_receipts()
                .iter()
                .map(ForgeQueryJournalReplayEntry::from_write_receipt),
        );
    }

    pub fn replay(
        &self,
        request: ForgeQueryJournalReplayRequest,
        current_snapshot_identity: &ForgeQuerySnapshotIdentity,
        published_artifact_digest: ForgeQueryEvidenceIdentity,
    ) -> Result<ForgeQueryJournalReplayOutcome, ForgeQueryJournalReplayDenial> {
        if let Some(basis_snapshot_identity) = request.basis_snapshot_identity() {
            if basis_snapshot_identity != current_snapshot_identity {
                return Err(self.deny(
                    ForgeQueryJournalReplayDenialKind::StaleBasisReplay,
                    "journal replay basis no longer matches the current runtime snapshot",
                ));
            }
        }
        let segment = request.segment_identity().clone();
        self.admit_segment_scheme(&segment)?;
        let retained_segment = self.entries_for_segment(&segment)?;
        let truth_reconstruction_identity = journal_replay_truth_reconstruction_identity(
            &retained_segment.committed_truth_identities(),
            retained_segment.expected_count,
            retained_segment.journal_gap_count,
        );
        let outcome = ForgeQueryJournalReplayOutcome::new(
            segment,
            retained_segment
                .entries
                .iter()
                .map(|entry| entry.write_receipt.clone())
                .collect(),
            retained_segment.expected_count,
            retained_segment.journal_gap_count,
            retained_segment.scanned_entry_count,
            truth_reconstruction_identity,
            published_artifact_digest,
        );
        self.counters.record_admitted_replay(
            retained_segment.scanned_entry_count,
            retained_segment.entries.len(),
            retained_segment.journal_gap_count,
            outcome.outcome_digest(),
        );
        Ok(outcome)
    }

    pub(in crate::runtime) fn diagnostics(&self) -> ForgeQueryJournalReplayDiagnostics {
        ForgeQueryJournalReplayDiagnostics::new(self.counters.snapshot(self.entries.len()))
    }

    pub(in crate::runtime) fn retain_replay_positions_for_certification(
        &mut self,
        retained_positions: &std::collections::BTreeSet<u64>,
    ) {
        self.entries
            .retain(|entry| retained_positions.contains(&entry.position));
    }

    fn admit_segment_scheme(
        &self,
        segment: &crate::runtime::ForgeQueryJournalSegmentIdentity,
    ) -> Result<(), ForgeQueryJournalReplayDenial> {
        if segment.identity().scheme() == ForgeQueryEvidenceIdentityScheme::V1 {
            return Ok(());
        }
        Err(self.deny(
            ForgeQueryJournalReplayDenialKind::CrossSchemeReplay,
            "journal segment identity scheme is not admitted by this replay surface",
        ))
    }

    fn entries_for_segment(
        &self,
        segment: &crate::runtime::ForgeQueryJournalSegmentIdentity,
    ) -> Result<ForgeQueryRetainedJournalSegment<'_>, ForgeQueryJournalReplayDenial> {
        let start = segment.start_position_for_reporting();
        let end = segment.end_position_for_reporting();
        let expected_count = (end - start + 1) as usize;
        let scanned_entry_count = self.entries.len();
        let entries = self
            .entries
            .iter()
            .filter(|entry| entry.position >= start && entry.position <= end)
            .collect::<Vec<_>>();
        if entries.is_empty() {
            return Err(self.deny(
                ForgeQueryJournalReplayDenialKind::UnknownSegmentIdentity,
                "journal segment does not resolve to retained replay entries",
            ));
        }
        let journal_gap_count = expected_count.saturating_sub(entries.len());
        if entries.len() != expected_count {
            return Err(self.deny(
                ForgeQueryJournalReplayDenialKind::JournalGap,
                "journal segment contains a gap in retained replay entries",
            ));
        }
        Ok(ForgeQueryRetainedJournalSegment {
            entries,
            expected_count,
            journal_gap_count,
            scanned_entry_count,
        })
    }

    fn deny(
        &self,
        kind: ForgeQueryJournalReplayDenialKind,
        message: impl Into<String>,
    ) -> ForgeQueryJournalReplayDenial {
        self.counters.record_denial(kind);
        ForgeQueryJournalReplayDenial::new(kind, message)
    }
}

struct ForgeQueryRetainedJournalSegment<'a> {
    entries: Vec<&'a ForgeQueryJournalReplayEntry>,
    expected_count: usize,
    journal_gap_count: usize,
    scanned_entry_count: usize,
}

impl<'a> ForgeQueryRetainedJournalSegment<'a> {
    fn committed_truth_identities(&self) -> Vec<ForgeQueryEvidenceIdentity> {
        self.entries
            .iter()
            .map(|entry| entry.committed_truth_identity.clone())
            .collect()
    }
}

#[derive(Clone, Debug)]
struct ForgeQueryJournalReplayEntry {
    position: u64,
    write_receipt: ForgeQueryWriteReceipt,
    committed_truth_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryJournalReplayEntry {
    fn from_write_receipt(receipt: &ForgeQueryWriteReceipt) -> Self {
        Self {
            position: receipt.journal_position().ordinal_for_reporting(),
            committed_truth_identity: receipt.committed_truth_identity().clone(),
            write_receipt: receipt.clone(),
        }
    }
}

pub(in crate::runtime) fn published_artifact_replay_digest(
    diagnostics: &crate::runtime::ForgeQueryPublishedArtifactDiagnostics,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::JournalReplayOutcome)
        .field_usize(
            ForgeQueryEvidenceTag::new("published_artifact_generation_count"),
            diagnostics.retained_generation_count(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("published_artifact_generation_ordinal"),
            diagnostics
                .generations()
                .iter()
                .map(|generation| generation.ordinal().to_string()),
        )
        .seal()
}
