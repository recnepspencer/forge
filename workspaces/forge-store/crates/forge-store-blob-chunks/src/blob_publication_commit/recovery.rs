use forge_store_recovery_physics::{
    PartialPublicationClassification, PartialPublicationCounterSnapshot,
    PartialPublicationReplayedCrashEdge, RecoveredOrRejectedPartialPublication,
    UnacknowledgedPublicationOutcome,
};

use crate::{ChunkTreeRoot, LogicalContentDigest};

use super::{
    evidence_identity::{recovery_evidence_digest, BlobPublicationRecoveryOperationDigest},
    BlobPublicationCounterSnapshot, BlobPublicationDenial, BlobPublicationSessionCloseout,
    BlobReachabilityStaging, BlobRootCandidateForPublication,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPublicationCrashPoint {
    AfterChunkWrite,
    AfterChecksumAdmission,
    AfterChunkTreeNodeDurability,
    AfterRootCandidateFormation,
    AfterReachabilityStaging,
    AfterPublicationRecordWrite,
    AfterSessionClose,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationRecoveryEvidence {
    crash_point: BlobPublicationCrashPoint,
    evidence_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationPreWalReplayEvidence {
    operation_digest: String,
    classification_digest: String,
    replay_read_identity: String,
    counters: PartialPublicationCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPublicationRecoveredState {
    DurableChunkNotVisible {
        counters: BlobPublicationCounterSnapshot,
    },
    ChecksumAdmittedNotVisible {
        counters: BlobPublicationCounterSnapshot,
    },
    ChunkTreeNodeDurableNotVisible {
        counters: BlobPublicationCounterSnapshot,
    },
    RootCandidateNotVisible {
        counters: BlobPublicationCounterSnapshot,
    },
    ReachabilityStagedNotVisible {
        counters: BlobPublicationCounterSnapshot,
    },
    PublicationRecordReplayableNotVisible {
        counters: BlobPublicationCounterSnapshot,
    },
    SessionClosedAwaitingVisibilityCommit {
        counters: BlobPublicationCounterSnapshot,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPublicationRecoveryReplay {
    evidence: BlobPublicationRecoveryEvidence,
    recovered_state: BlobPublicationRecoveredState,
}

impl BlobPublicationRecoveryEvidence {
    pub fn chunk_write_replayed(
        digest: &LogicalContentDigest,
        replay: BlobPublicationPreWalReplayEvidence,
    ) -> Result<Self, BlobPublicationDenial> {
        let operation_digest = chunk_write_recovery_operation_digest(digest);
        let replay = replay.require_operation(&operation_digest)?;
        Ok(Self::new(
            BlobPublicationCrashPoint::AfterChunkWrite,
            recovery_evidence_digest(
                "chunk-write",
                replay.replay_read_identity(),
                digest.digest().as_str(),
            ),
        ))
    }

    pub fn checksum_admitted(
        digest: &LogicalContentDigest,
        replay: BlobPublicationPreWalReplayEvidence,
    ) -> Result<Self, BlobPublicationDenial> {
        let operation_digest = checksum_recovery_operation_digest(digest);
        let replay = replay.require_operation(&operation_digest)?;
        Ok(Self::new(
            BlobPublicationCrashPoint::AfterChecksumAdmission,
            recovery_evidence_digest(
                "checksum-admitted",
                replay.replay_read_identity(),
                digest.digest().as_str(),
            ),
        ))
    }

    pub fn chunk_tree_node_durable(
        root: &ChunkTreeRoot,
        replay: BlobPublicationPreWalReplayEvidence,
    ) -> Result<Self, BlobPublicationDenial> {
        let operation_digest = chunk_tree_recovery_operation_digest(root);
        let replay = replay.require_operation(&operation_digest)?;
        Ok(Self::new(
            BlobPublicationCrashPoint::AfterChunkTreeNodeDurability,
            recovery_evidence_digest(
                "chunk-tree-durable",
                replay.replay_read_identity(),
                root.digest().as_str(),
            ),
        ))
    }

    pub fn root_candidate(
        candidate: &BlobRootCandidateForPublication,
        replay: BlobPublicationPreWalReplayEvidence,
    ) -> Result<Self, BlobPublicationDenial> {
        let operation_digest = root_candidate_recovery_operation_digest(candidate);
        let replay = replay.require_operation(&operation_digest)?;
        Ok(Self::new(
            BlobPublicationCrashPoint::AfterRootCandidateFormation,
            recovery_evidence_digest(
                "root-candidate",
                replay.replay_read_identity(),
                candidate.intent().chunk_tree_root().digest().as_str(),
            ),
        ))
    }

    pub fn reachability_staged(
        staged: &BlobReachabilityStaging,
        replay: BlobPublicationPreWalReplayEvidence,
    ) -> Result<Self, BlobPublicationDenial> {
        let operation_digest = reachability_recovery_operation_digest(staged);
        let replay = replay.require_operation(&operation_digest)?;
        Ok(Self::new(
            BlobPublicationCrashPoint::AfterReachabilityStaging,
            recovery_evidence_digest(
                "reachability-staged",
                replay.replay_read_identity(),
                staged
                    .staging_identity()
                    .publication_record_digest()
                    .as_str(),
            ),
        ))
    }

    pub fn publication_record_replayable(
        classification: &PartialPublicationClassification,
    ) -> Result<Self, BlobPublicationDenial> {
        if !matches!(
            classification.recovered_or_rejected(),
            RecoveredOrRejectedPartialPublication::ReplayableUnacknowledgedWal { .. }
        ) {
            return Err(BlobPublicationDenial::WalReplayEvidenceRequired {
                counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
            });
        }
        Ok(Self::new(
            BlobPublicationCrashPoint::AfterPublicationRecordWrite,
            classification.classification_digest(),
        ))
    }

    pub fn session_closed(closeout: &BlobPublicationSessionCloseout) -> Self {
        Self::new(
            BlobPublicationCrashPoint::AfterSessionClose,
            recovery_evidence_digest(
                "session-closed",
                closeout.wal_commit().replay_classification_digest(),
                closeout
                    .wal_commit()
                    .staging_identity()
                    .publication_record_digest()
                    .as_str(),
            ),
        )
    }

    fn new(crash_point: BlobPublicationCrashPoint, evidence_digest: impl Into<String>) -> Self {
        Self {
            crash_point,
            evidence_digest: evidence_digest.into(),
        }
    }

    pub const fn crash_point(&self) -> BlobPublicationCrashPoint {
        self.crash_point
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}

impl BlobPublicationPreWalReplayEvidence {
    pub fn from_chunk_write_replay(
        digest: &LogicalContentDigest,
        replay: &PartialPublicationReplayedCrashEdge,
    ) -> Result<Self, BlobPublicationDenial> {
        Self::from_replayed_crash_edge(replay, &chunk_write_recovery_operation_digest(digest))
    }

    pub fn from_checksum_admitted_replay(
        digest: &LogicalContentDigest,
        replay: &PartialPublicationReplayedCrashEdge,
    ) -> Result<Self, BlobPublicationDenial> {
        Self::from_replayed_crash_edge(replay, &checksum_recovery_operation_digest(digest))
    }

    pub fn from_chunk_tree_node_durable_replay(
        root: &ChunkTreeRoot,
        replay: &PartialPublicationReplayedCrashEdge,
    ) -> Result<Self, BlobPublicationDenial> {
        Self::from_replayed_crash_edge(replay, &chunk_tree_recovery_operation_digest(root))
    }

    pub fn from_root_candidate_replay(
        candidate: &BlobRootCandidateForPublication,
        replay: &PartialPublicationReplayedCrashEdge,
    ) -> Result<Self, BlobPublicationDenial> {
        Self::from_replayed_crash_edge(replay, &root_candidate_recovery_operation_digest(candidate))
    }

    pub fn from_reachability_staged_replay(
        staged: &BlobReachabilityStaging,
        replay: &PartialPublicationReplayedCrashEdge,
    ) -> Result<Self, BlobPublicationDenial> {
        Self::from_replayed_crash_edge(replay, &reachability_recovery_operation_digest(staged))
    }

    #[cfg(test)]
    pub(crate) fn chunk_write_recovery_operation_digest(
        digest: &LogicalContentDigest,
    ) -> BlobPublicationRecoveryOperationDigest {
        chunk_write_recovery_operation_digest(digest)
    }

    #[cfg(test)]
    pub(crate) fn checksum_admitted_recovery_operation_digest(
        digest: &LogicalContentDigest,
    ) -> BlobPublicationRecoveryOperationDigest {
        checksum_recovery_operation_digest(digest)
    }

    #[cfg(test)]
    pub(crate) fn chunk_tree_node_durable_recovery_operation_digest(
        root: &ChunkTreeRoot,
    ) -> BlobPublicationRecoveryOperationDigest {
        chunk_tree_recovery_operation_digest(root)
    }

    #[cfg(test)]
    pub(crate) fn root_candidate_recovery_operation_digest(
        candidate: &BlobRootCandidateForPublication,
    ) -> BlobPublicationRecoveryOperationDigest {
        root_candidate_recovery_operation_digest(candidate)
    }

    #[cfg(test)]
    pub(crate) fn reachability_staged_recovery_operation_digest(
        staged: &BlobReachabilityStaging,
    ) -> BlobPublicationRecoveryOperationDigest {
        reachability_recovery_operation_digest(staged)
    }

    fn from_replayed_crash_edge(
        replay: &PartialPublicationReplayedCrashEdge,
        expected_operation_digest: &BlobPublicationRecoveryOperationDigest,
    ) -> Result<Self, BlobPublicationDenial> {
        if replay.outcome() == UnacknowledgedPublicationOutcome::NoWalAppendObserved
            && replay.before_wal_append_operation_digest()
                == Some(expected_operation_digest.as_str())
        {
            Ok(Self {
                operation_digest: expected_operation_digest.as_str().to_owned(),
                classification_digest: replay.classification_digest().to_owned(),
                replay_read_identity: replay.replay_read_identity().to_owned(),
                counters: replay.counters(),
            })
        } else {
            Err(BlobPublicationDenial::WalReplayEvidenceRequired {
                counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
            })
        }
    }

    fn require_operation(
        self,
        expected_operation_digest: &BlobPublicationRecoveryOperationDigest,
    ) -> Result<Self, BlobPublicationDenial> {
        if self.operation_digest == expected_operation_digest.as_str() {
            Ok(self)
        } else {
            Err(BlobPublicationDenial::WalReplayEvidenceRequired {
                counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
            })
        }
    }

    pub fn classification_digest(&self) -> &str {
        &self.classification_digest
    }

    pub fn replay_read_identity(&self) -> &str {
        &self.replay_read_identity
    }

    pub const fn counters(&self) -> PartialPublicationCounterSnapshot {
        self.counters
    }
}

impl BlobPublicationRecoveryReplay {
    pub fn recover(evidence: BlobPublicationRecoveryEvidence) -> Self {
        let counters = BlobPublicationCounterSnapshot::start().with_recovered_state();
        let recovered_state = match evidence.crash_point() {
            BlobPublicationCrashPoint::AfterChunkWrite => {
                BlobPublicationRecoveredState::DurableChunkNotVisible { counters }
            }
            BlobPublicationCrashPoint::AfterChecksumAdmission => {
                BlobPublicationRecoveredState::ChecksumAdmittedNotVisible { counters }
            }
            BlobPublicationCrashPoint::AfterChunkTreeNodeDurability => {
                BlobPublicationRecoveredState::ChunkTreeNodeDurableNotVisible { counters }
            }
            BlobPublicationCrashPoint::AfterRootCandidateFormation => {
                BlobPublicationRecoveredState::RootCandidateNotVisible { counters }
            }
            BlobPublicationCrashPoint::AfterReachabilityStaging => {
                BlobPublicationRecoveredState::ReachabilityStagedNotVisible { counters }
            }
            BlobPublicationCrashPoint::AfterPublicationRecordWrite => {
                BlobPublicationRecoveredState::PublicationRecordReplayableNotVisible { counters }
            }
            BlobPublicationCrashPoint::AfterSessionClose => {
                BlobPublicationRecoveredState::SessionClosedAwaitingVisibilityCommit { counters }
            }
        };
        Self {
            evidence,
            recovered_state,
        }
    }

    pub const fn crash_point(&self) -> BlobPublicationCrashPoint {
        self.evidence.crash_point()
    }

    pub const fn evidence(&self) -> &BlobPublicationRecoveryEvidence {
        &self.evidence
    }

    pub const fn recovered_state(&self) -> BlobPublicationRecoveredState {
        self.recovered_state
    }
}

fn chunk_write_recovery_operation_digest(
    digest: &LogicalContentDigest,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::chunk_write(digest)
}

fn checksum_recovery_operation_digest(
    digest: &LogicalContentDigest,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::checksum_admitted(digest)
}

fn chunk_tree_recovery_operation_digest(
    root: &ChunkTreeRoot,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::chunk_tree_durable(root)
}

fn root_candidate_recovery_operation_digest(
    candidate: &BlobRootCandidateForPublication,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::root_candidate(candidate)
}

fn reachability_recovery_operation_digest(
    staged: &BlobReachabilityStaging,
) -> BlobPublicationRecoveryOperationDigest {
    BlobPublicationRecoveryOperationDigest::reachability_staged(staged)
}
