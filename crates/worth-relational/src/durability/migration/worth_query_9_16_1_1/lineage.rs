use serde::Deserialize;

use crate::history::data::{BranchId, RelationalCommitReceipt};
use crate::identity::data::LineageId;
use crate::lineage::data::{
    FinalizedLineageEventBatch, LineageDecisionKind, LineageDecisionLog, LineageDecisionRecord,
    LineageEventKind, LineageEventRecord, LineageFinalizationArtifact, PublishedLineageArtifact,
};

#[derive(Deserialize)]
pub(super) struct LegacyPublishedLineageArtifact {
    branch_id: BranchId,
    lineage_event_ids: Vec<u64>,
    lineage_events: Vec<LegacyLineageEventRecord>,
    lineage_decision_log: Vec<LegacyLineageDecisionRecord>,
    digest_basis: serde::de::IgnoredAny,
    counters: serde::de::IgnoredAny,
}

#[derive(Debug, Clone)]
pub(crate) struct LegacyLineageReadmissionProvenance {
    correspondence_events: Vec<LegacyCorrespondenceEventProvenance>,
    correspondence_decisions: Vec<LegacyCorrespondenceDecisionProvenance>,
}

#[derive(Debug, Clone)]
struct LegacyCorrespondenceEventProvenance {
    event_id: u64,
    sources: Vec<LineageId>,
    targets: Vec<LineageId>,
}

#[derive(Debug, Clone)]
struct LegacyCorrespondenceDecisionProvenance {
    candidate_id: Option<u64>,
    event_id: Option<u64>,
    sources: Vec<LineageId>,
    targets: Vec<LineageId>,
}

pub(super) struct LegacyLineageReadmissionError {
    candidate_id: Option<u64>,
    rejection: Option<LegacyCorrespondencePromotionRejectionClass>,
}

impl LegacyLineageReadmissionError {
    pub(super) fn detail(&self) -> String {
        format!(
            "WORTH Query 9.16.1.1 correspondence rejection cannot be represented by current lineage vocabulary: candidate {:?}, rejection {:?}",
            self.candidate_id, self.rejection
        )
    }
}

#[derive(Deserialize)]
struct LegacyLineageEventRecord {
    event_id: u64,
    commit: RelationalCommitReceipt,
    branch_id: BranchId,
    kind: LegacyLineageEventKind,
    sources: Vec<LineageId>,
    targets: Vec<LineageId>,
}

#[derive(Deserialize)]
enum LegacyLineageEventKind {
    Create,
    Replace,
    Split,
    Merge,
    Retire,
    Correspond,
}

#[derive(Deserialize)]
struct LegacyLineageDecisionRecord {
    branch_id: BranchId,
    kind: LegacyLineageDecisionKind,
    event_id: Option<u64>,
    candidate_id: Option<LegacyCorrespondenceCandidateId>,
    sources: Vec<LineageId>,
    targets: Vec<LineageId>,
    rejection_class: Option<LegacyCorrespondencePromotionRejectionClass>,
}

#[derive(Deserialize)]
enum LegacyLineageDecisionKind {
    CreateAccepted,
    ReplaceAccepted,
    RetireAccepted,
    CorrespondencePromotionAccepted,
    CorrespondencePromotionRejected,
}

#[derive(Clone, Copy, Deserialize)]
struct LegacyCorrespondenceCandidateId(u64);

#[derive(Debug, Clone, Copy, Deserialize)]
enum LegacyCorrespondencePromotionRejectionClass {
    CandidateMissing,
    MissingLineageReference,
    EmptyEndpointSet,
    DuplicateEndpointReference,
    OverlappingSourceAndTarget,
    CommitBranchMismatch,
    BranchScopeMismatch,
    CommitNotBranchHead,
    AuthorityPublicationFailed,
}

impl LegacyPublishedLineageArtifact {
    pub(super) fn readmit(
        self,
        publication_commit: &RelationalCommitReceipt,
        metadata_only: bool,
    ) -> Result<
        (PublishedLineageArtifact, LegacyLineageReadmissionProvenance),
        LegacyLineageReadmissionError,
    > {
        let _legacy_integrity_evidence = (self.lineage_event_ids, self.digest_basis, self.counters);
        let mut correspondence_events = Vec::new();
        let events = self
            .lineage_events
            .into_iter()
            .map(|event| {
                let kind = match event.kind {
                    LegacyLineageEventKind::Create => LineageEventKind::Create,
                    LegacyLineageEventKind::Replace => LineageEventKind::Replace,
                    LegacyLineageEventKind::Split => LineageEventKind::Split,
                    LegacyLineageEventKind::Merge => LineageEventKind::Merge,
                    LegacyLineageEventKind::Retire => LineageEventKind::Retire,
                    LegacyLineageEventKind::Correspond => {
                        correspondence_events.push(LegacyCorrespondenceEventProvenance {
                            event_id: event.event_id,
                            sources: event.sources.clone(),
                            targets: event.targets.clone(),
                        });
                        // Legacy correspondence moved lineage reachability from
                        // sources to targets. Current replacement vocabulary is
                        // the exact operational readmission of that movement.
                        LineageEventKind::Replace
                    }
                };
                LineageEventRecord {
                    event_id: event.event_id,
                    commit: if metadata_only {
                        publication_commit.clone()
                    } else {
                        event.commit
                    },
                    branch_id: event.branch_id,
                    kind,
                    sources: event.sources,
                    targets: event.targets,
                }
            })
            .collect();
        let mut correspondence_decisions = Vec::new();
        let mut decisions = Vec::with_capacity(self.lineage_decision_log.len());
        for decision in self.lineage_decision_log {
            let kind = match decision.kind {
                LegacyLineageDecisionKind::CreateAccepted => LineageDecisionKind::CreateAccepted,
                LegacyLineageDecisionKind::ReplaceAccepted => LineageDecisionKind::ReplaceAccepted,
                LegacyLineageDecisionKind::RetireAccepted => LineageDecisionKind::RetireAccepted,
                LegacyLineageDecisionKind::CorrespondencePromotionAccepted => {
                    correspondence_decisions.push(decision.provenance());
                    LineageDecisionKind::ReplaceAccepted
                }
                LegacyLineageDecisionKind::CorrespondencePromotionRejected => {
                    return Err(LegacyLineageReadmissionError {
                        candidate_id: decision.candidate_id.map(|candidate| candidate.0),
                        rejection: decision.rejection_class,
                    });
                }
            };
            decisions.push(LineageDecisionRecord {
                branch_id: decision.branch_id,
                kind,
                event_id: decision.event_id,
                sources: decision.sources,
                targets: decision.targets,
            });
        }
        let current = LineageFinalizationArtifact::new(
            self.branch_id,
            FinalizedLineageEventBatch::new(events),
            LineageDecisionLog::new(decisions),
        )
        .publish();
        let provenance = LegacyLineageReadmissionProvenance {
            correspondence_events,
            correspondence_decisions,
        };
        Ok((current, provenance))
    }
}

impl LegacyLineageDecisionRecord {
    fn provenance(&self) -> LegacyCorrespondenceDecisionProvenance {
        LegacyCorrespondenceDecisionProvenance {
            candidate_id: self.candidate_id.map(|candidate| candidate.0),
            event_id: self.event_id,
            sources: self.sources.clone(),
            targets: self.targets.clone(),
        }
    }
}

impl LegacyLineageReadmissionProvenance {
    pub(crate) fn validates_translation(&self, current: &PublishedLineageArtifact) -> bool {
        let events_preserved = self.correspondence_events.iter().all(|legacy| {
            current.lineage_events().iter().any(|event| {
                event.event_id() == legacy.event_id
                    && event.kind() == LineageEventKind::Replace
                    && event.sources() == legacy.sources
                    && event.targets() == legacy.targets
            })
        });
        let decisions_preserved = self.correspondence_decisions.iter().all(|legacy| {
            if legacy.candidate_id.is_none() {
                return false;
            }
            current.lineage_decision_log().iter().any(|decision| {
                decision.kind() == &LineageDecisionKind::ReplaceAccepted
                    && decision.event_id() == legacy.event_id
                    && decision.sources() == legacy.sources
                    && decision.targets() == legacy.targets
            })
        });
        events_preserved && decisions_preserved
    }
}
