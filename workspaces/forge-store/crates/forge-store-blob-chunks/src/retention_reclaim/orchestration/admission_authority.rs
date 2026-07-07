use forge_store_physical_isolation::BlobOrphanReclaimBarrier;

use crate::retention_reclaim::admission::reachability_gate::admit_via_reachability_gate;
use crate::retention_reclaim::counters::BlobRetentionReclaimCounterSnapshot;
use crate::retention_reclaim::denial::BlobRetentionReclaimDenial;
use crate::retention_reclaim::holds::BlobRetentionHold;
use crate::retention_reclaim::types::admission::BlobRetentionReclaimAdmission;
use crate::retention_reclaim::verification::hold_blocking::deny_retention_hold;
use crate::{BlobChunkIdentity, BlobChunkReachabilityRegistry, S6BlobReclaimNonClaimHandoff};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlobRetentionReclaimAdmissionAuthority {
    _private: (),
}

impl BlobRetentionReclaimAdmissionAuthority {
    pub const fn store_owned() -> Self {
        Self { _private: () }
    }

    pub fn admit_reachability_orphan(
        self,
        reachability: &BlobChunkReachabilityRegistry,
        chunk_identity: &BlobChunkIdentity,
        s6_posture: S6BlobReclaimNonClaimHandoff,
    ) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
        admit_via_reachability_gate(reachability, chunk_identity, s6_posture, None)
    }

    pub fn admit_abandoned_resume_orphan(
        self,
        reachability: &BlobChunkReachabilityRegistry,
        chunk_identity: &BlobChunkIdentity,
        barrier: &BlobOrphanReclaimBarrier,
        s6_posture: S6BlobReclaimNonClaimHandoff,
    ) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
        admit_via_reachability_gate(reachability, chunk_identity, s6_posture, Some(barrier))
    }

    pub fn deny_retention_hold(self, hold: &BlobRetentionHold) -> BlobRetentionReclaimDenial {
        deny_retention_hold(hold)
    }

    pub fn deny_missing_s6_reclaim_posture(self) -> BlobRetentionReclaimDenial {
        BlobRetentionReclaimDenial::MissingS6ReclaimPosture {
            counters: BlobRetentionReclaimCounterSnapshot::start()
                .with_orphan_candidate()
                .record_replay_convergence_check()
                .record_s6_posture_denial(),
        }
    }
}
