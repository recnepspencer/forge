use topology::facade::NmtTopologyScopeReceipt;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    NmtCertificationDenial, NmtCertificationDenialInput, NmtCertificationDenialKind,
    NmtScopeAttackCounters, NmtScopeProjectionReceipt,
};
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NmtScopeRetainedReplayCounters {
    scope_retained_artifact_rows: usize,
    scope_replay_rows: usize,
    scope_projection_consumed_rows: usize,
    parent_replay_rows_read: usize,
    scope_checkpoints_consumed: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtScopeRetainedReplayReceipt {
    parent_replay_identity: String,
    scope_identity: String,
    scope_projection_identity: String,
    scope_replay_identity: String,
    checkpoint_identity: String,
    counters: NmtScopeRetainedReplayCounters,
}

impl NmtScopeRetainedReplayReceipt {
    pub(crate) fn from_replay_scope(
        replay: &ReplayReceiptSet,
        scope: &NmtTopologyScopeReceipt,
        projection: &NmtScopeProjectionReceipt,
    ) -> Result<Self, NmtCertificationDenial> {
        if projection.scope_identity() != scope.scope_identity() {
            return Err(denial(
                NmtCertificationDenialKind::CrossScopeProjection,
                scope,
                Some(projection.scope_identity().to_string()),
                projection.scope_projection_identity().to_string(),
                "NMT retained replay scope requires projection evidence from the same topology scope.",
            ));
        }
        let replay_counters = replay.counters();
        if replay_counters.replay_rows() == 0 || replay_counters.retained_artifact_rows() == 0 {
            return Err(denial(
                NmtCertificationDenialKind::MissingScopeReplay,
                scope,
                None,
                replay.stage_identity().receipt_identity(),
                "NMT retained replay scope requires retained artifact and replay rows from production replay.",
            ));
        }
        let scope_replay_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-scope-retained-replay".to_string(),
                replay.stage_identity().receipt_identity(),
                replay.replay_checkpoint_identity().to_string(),
                scope.scope_identity().to_string(),
                projection.scope_projection_identity().to_string(),
            ],
        );
        let checkpoint_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-scope-replay-checkpoint".to_string(),
                replay.replay_checkpoint_identity().to_string(),
                scope.scope_identity().to_string(),
                projection.scope_projection_identity().to_string(),
            ],
        );
        Ok(Self {
            parent_replay_identity: replay.stage_identity().receipt_identity(),
            scope_identity: scope.scope_identity().to_string(),
            scope_projection_identity: projection.scope_projection_identity().to_string(),
            scope_replay_identity,
            checkpoint_identity,
            counters: NmtScopeRetainedReplayCounters {
                scope_retained_artifact_rows: projection
                    .counters()
                    .scope_projected_entities_consumed(),
                scope_replay_rows: 1,
                scope_projection_consumed_rows: projection
                    .counters()
                    .scope_projected_entities_consumed(),
                parent_replay_rows_read: replay_counters.replay_rows(),
                scope_checkpoints_consumed: 1,
            },
        })
    }

    pub fn parent_replay_identity(&self) -> &str {
        &self.parent_replay_identity
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn scope_projection_identity(&self) -> &str {
        &self.scope_projection_identity
    }

    pub fn scope_replay_identity(&self) -> &str {
        &self.scope_replay_identity
    }

    pub fn checkpoint_identity(&self) -> &str {
        &self.checkpoint_identity
    }

    pub fn counters(&self) -> NmtScopeRetainedReplayCounters {
        self.counters
    }
}

impl NmtScopeRetainedReplayCounters {
    pub fn scope_retained_artifact_rows(self) -> usize {
        self.scope_retained_artifact_rows
    }

    pub fn scope_replay_rows(self) -> usize {
        self.scope_replay_rows
    }

    pub fn scope_projection_consumed_rows(self) -> usize {
        self.scope_projection_consumed_rows
    }

    pub fn parent_replay_rows_read(self) -> usize {
        self.parent_replay_rows_read
    }

    pub fn scope_checkpoints_consumed(self) -> usize {
        self.scope_checkpoints_consumed
    }
}

fn denial(
    kind: NmtCertificationDenialKind,
    target: &NmtTopologyScopeReceipt,
    source_scope_identity: Option<String>,
    evidence: String,
    human_reason: impl Into<String>,
) -> NmtCertificationDenial {
    NmtCertificationDenial::new(NmtCertificationDenialInput {
        kind,
        target_scope_identity: Some(target.scope_identity().to_string()),
        source_scope_identity,
        target_scope_kind: Some(target.kind()),
        consumed_evidence_digest: evidence,
        human_reason: human_reason.into(),
        counters: NmtScopeAttackCounters::new(
            1,
            target.counters().scope_entity_count(),
            0,
            1,
            0,
            1,
        ),
    })
}
