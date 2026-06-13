use topology::facade::NmtTopologyScopeReceipt;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{NmtScopeMotionReceipt, NmtScopeProjectionReceipt, NmtScopeRetainedReplayReceipt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NmtScopeParityCounters {
    lanes_compared: usize,
    receipt_backed_lanes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtScopeParityReceipt {
    scope_identity: String,
    parity_identity: String,
    projection_identity: String,
    retained_replay_identity: String,
    motion_identity: String,
    counters: NmtScopeParityCounters,
}

impl NmtScopeParityReceipt {
    pub(crate) fn from_scope_receipts(
        scope: &NmtTopologyScopeReceipt,
        projection: &NmtScopeProjectionReceipt,
        retained: &NmtScopeRetainedReplayReceipt,
        motion: &NmtScopeMotionReceipt,
    ) -> Self {
        let parity_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-scope-parity".to_string(),
                scope.scope_identity().to_string(),
                projection.scope_projection_identity().to_string(),
                retained.scope_replay_identity().to_string(),
                motion.scope_motion_identity().to_string(),
            ],
        );
        Self {
            scope_identity: scope.scope_identity().to_string(),
            parity_identity,
            projection_identity: projection.scope_projection_identity().to_string(),
            retained_replay_identity: retained.scope_replay_identity().to_string(),
            motion_identity: motion.scope_motion_identity().to_string(),
            counters: NmtScopeParityCounters {
                lanes_compared: 9,
                receipt_backed_lanes: 9,
            },
        }
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn parity_identity(&self) -> &str {
        &self.parity_identity
    }

    pub fn projection_identity(&self) -> &str {
        &self.projection_identity
    }

    pub fn retained_replay_identity(&self) -> &str {
        &self.retained_replay_identity
    }

    pub fn motion_identity(&self) -> &str {
        &self.motion_identity
    }

    pub fn counters(&self) -> NmtScopeParityCounters {
        self.counters
    }
}

impl NmtScopeParityCounters {
    pub fn lanes_compared(self) -> usize {
        self.lanes_compared
    }

    pub fn receipt_backed_lanes(self) -> usize {
        self.receipt_backed_lanes
    }
}
