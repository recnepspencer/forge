use super::{LayoutCourtroomTranscriptIdentity, LayoutEvidenceBundle};
use crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceKind as Evidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LayoutProofOutcomeKind {
    Denied,
    Stale,
    RebindRequired,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutProofOutcomeObservation {
    transcript_identity: LayoutCourtroomTranscriptIdentity,
    outcomes: [LayoutProofOutcomeKind; 4],
}

pub fn observe_layout_proof_outcomes(
    bundle: &LayoutEvidenceBundle,
) -> Option<LayoutProofOutcomeObservation> {
    let evidence = bundle.coverage().executed_evidence();
    [
        Evidence::HiddenBroadScanDenied,
        Evidence::BTreeReadinessStale,
        Evidence::RollbackRebindRequired,
        Evidence::MaintenanceDeferred,
    ]
    .into_iter()
    .all(|required| evidence.contains(required))
    .then_some(LayoutProofOutcomeObservation {
        transcript_identity: bundle.transcript_identity(),
        outcomes: [
            LayoutProofOutcomeKind::Denied,
            LayoutProofOutcomeKind::Stale,
            LayoutProofOutcomeKind::RebindRequired,
            LayoutProofOutcomeKind::Deferred,
        ],
    })
}

impl LayoutProofOutcomeObservation {
    pub const fn transcript_identity(&self) -> LayoutCourtroomTranscriptIdentity {
        self.transcript_identity
    }

    pub const fn outcomes(&self) -> &[LayoutProofOutcomeKind; 4] {
        &self.outcomes
    }
}
